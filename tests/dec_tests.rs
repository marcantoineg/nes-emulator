use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use crate::common::{assert_flag, assert_no_flags};

mod dec_zero_page {
    use super::*;

    #[test]
    fn test_0xc6_dec_decrease_memory_value_by_one() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x01, 0x02);

        cpu.load_and_run_without_reset(vec![0xC6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x01), 0x01);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0xc6_dec_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x01, 0x01);

        cpu.load_and_run_without_reset(vec![0xC6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x01), 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0xc6_dec_sets_negative_flag_and_wraps_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x01, 0x00);

        cpu.load_and_run_without_reset(vec![0xC6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x01), 0xFF);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod dec_zero_page_x {
    use super::*;

    #[test]
    fn test_0xd6_dec_decrease_memory_value_by_one() {
        let mut cpu = CPU::new();
        cpu.register_x = 1;
        cpu.memory.write(0x02, 0x02);

        cpu.load_and_run_without_reset(vec![0xD6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x02), 0x01);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0xd6_dec_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 1;
        cpu.memory.write(0x02, 0x01);

        cpu.load_and_run_without_reset(vec![0xD6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x02), 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0xd6_dec_sets_negative_flag_and_wraps_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 1;
        cpu.memory.write(0x02, 0x00);

        cpu.load_and_run_without_reset(vec![0xD6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x02), 0xFF);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod dec_absolute {
    use super::*;

    #[test]
    fn test_0xce_dec_decrease_memory_value_by_one() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x1010, 0x02);

        cpu.load_and_run_without_reset(vec![0xCE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1010), 0x01);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0xce_dec_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x1010, 0x01);

        cpu.load_and_run_without_reset(vec![0xCE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1010), 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0xce_dec_sets_negative_flag_and_wraps_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x1010, 0x00);

        cpu.load_and_run_without_reset(vec![0xCE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1010), 0xFF);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod dec_absolute_x {
    use super::*;

    #[test]
    fn test_0xde_dec_decrease_memory_value_by_one() {
        let mut cpu = CPU::new();
        cpu.register_x = 1;
        cpu.memory.write(0x1011, 0x02);

        cpu.load_and_run_without_reset(vec![0xDE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1011), 0x01);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0xde_dec_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 1;
        cpu.memory.write(0x1011, 0x01);

        cpu.load_and_run_without_reset(vec![0xDE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1011), 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0xde_dec_sets_negative_flag_and_wraps_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 1;
        cpu.memory.write(0x1011, 0x00);

        cpu.load_and_run_without_reset(vec![0xDE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1011), 0xFF);
        assert_flag(&cpu, Flags::Negative);
    }
}
