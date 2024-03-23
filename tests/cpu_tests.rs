use std::vec;

use nes_emulator::cpu::CPU;

mod tax {
    use super::*;

    #[test]
    fn test_0xaa_tax_implied_copy_data() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;

        cpu.load_and_run_without_reset(vec![0xAA, 0x00]);

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.register_x, 0x01);
        assert_eq!(cpu.zero_flag(), false);
        assert_eq!(cpu.negative_flag(), false);
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x00;

        cpu.load_and_run_without_reset(vec![0xAA, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.zero_flag(), true);
    }

    #[test]
    fn test_0xaa_tax_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0000;

        cpu.load_and_run_without_reset(vec![0xAA, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0000);
        assert_eq!(cpu.register_x, 0b1000_0000);
        assert_eq!(cpu.negative_flag(), true);
    }
}

mod inx {
    use super::*;

    #[test]
    fn test_0xe8_inx_implied_increment_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;

        cpu.load_and_run_without_reset(vec![0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0x02);
        assert_eq!(cpu.zero_flag(), false);
        assert_eq!(cpu.negative_flag(), false);
    }

    #[test]
    fn test_0xe8_inx_implied_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        
        cpu.load_and_run_without_reset(vec![0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0)
    }

    #[test]
    fn test_0xe8_inx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xFF;

        cpu.load_and_run_without_reset(vec![0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.zero_flag(), true);
    }

    #[test]
    fn test_0xe8_inx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_x = 0b0111_1111;

        cpu.load_and_run_without_reset(vec![0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0b1000_0000);
        assert_eq!(cpu.negative_flag(), true);
    }
}

#[test]
fn test_load_and_run_reset_registeries() {
    let mut cpu = CPU::new();
    cpu.register_a = 1;
    cpu.register_x = 2;
    cpu.register_y = 3;
    cpu.status = 0b1111_1111;

    cpu.load_and_run(vec![0x00]);

    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.status, 0);
}
#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run_without_reset(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}
