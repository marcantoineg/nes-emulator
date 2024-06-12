use common::assert_flag;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use crate::common::assert_no_flags;

mod ora_immediate {
    use super::*;

    #[test]
    fn test_0x09_ora_immediate_performs_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0101_0101;
        
        cpu.load_and_run_without_reset(vec![
            0x09, 0b0000_0101, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b0101_0101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x09_ora_immediate_handles_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_0000;

        cpu.load_and_run_without_reset(vec![
            0x09, 0b0000_0000, 0x00
        ]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x09_ora_immediate_handles_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_1111;

        cpu.load_and_run_without_reset(vec![
            0x09, 0b1000_0000, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b1000_1111);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod ora_zero_page {
    use super::*;
    
    #[test]
    fn test_0x05_ora_zero_page_performs_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0101_0101;
        cpu.memory.write(0x0010, 0b0011_1100);
        
        cpu.load_and_run_without_reset(vec![
            0x05, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b0111_1101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x05_ora_zero_page_handles_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_0000;
        cpu.memory.write(0x0010, 0b0000_0000);

        cpu.load_and_run_without_reset(vec![
            0x05, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x05_ora_zero_page_handles_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_1111;
        cpu.memory.write(0x0010, 0b1000_0000);

        cpu.load_and_run_without_reset(vec![
            0x05, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b1000_1111);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod ora_zero_page_x {
    use super::*;
    
    #[test]
    fn test_0x15_ora_zero_page_x_performs_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0101_0101;
        cpu.register_x = 0x01;
        cpu.memory.write(0x0011, 0b0011_1100);
        
        cpu.load_and_run_without_reset(vec![
            0x15, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b0111_1101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x15_ora_zero_page_x_handles_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_0000;
        cpu.register_x = 0x01;
        cpu.memory.write(0x0011, 0b0000_0000);

        cpu.load_and_run_without_reset(vec![
            0x15, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x15_ora_zero_page_x_handles_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_1111;
        cpu.register_x = 0x01;
        cpu.memory.write(0x0011, 0b1000_0000);

        cpu.load_and_run_without_reset(vec![
            0x15, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b1000_1111);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod ora_absolute {
    use super::*;
    
    #[test]
    fn test_0x0d_ora_absolute_performs_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0101_0101;
        cpu.memory.write(0x1011, 0b0011_1100);
        
        cpu.load_and_run_without_reset(vec![
            0x0D, 0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b0111_1101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x0d_ora_absolute_handles_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_0000;
        cpu.memory.write(0x1011, 0b0000_0000);

        cpu.load_and_run_without_reset(vec![
            0x0D, 0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x0d_ora_absolute_handles_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_1111;
        cpu.memory.write(0x1011, 0b1000_0000);

        cpu.load_and_run_without_reset(vec![
            0x0D, 0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b1000_1111);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod ora_absolute_x {
    use super::*;
    
    #[test]
    fn test_0x1d_ora_absolute_x_performs_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0101_0101;
        cpu.register_x = 0x01;
        cpu.memory.write(0x1012, 0b0011_1100);
        
        cpu.load_and_run_without_reset(vec![
            0x1D, 0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b0111_1101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x1d_ora_absolute_x_handles_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_0000;
        cpu.register_x = 0x01;
        cpu.memory.write(0x1012, 0b0000_0000);

        cpu.load_and_run_without_reset(vec![
            0x1D, 0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x1d_ora_absolute_x_handles_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_1111;
        cpu.register_x = 0x01;
        cpu.memory.write(0x1012, 0b1000_0000);

        cpu.load_and_run_without_reset(vec![
            0x1D, 0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b1000_1111);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod ora_absolute_y {
    use super::*;
    
    #[test]
    fn test_0x19_ora_absolute_y_performs_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0101_0101;
        cpu.register_y = 0x01;
        cpu.memory.write(0x1012, 0b0011_1100);
        
        cpu.load_and_run_without_reset(vec![
            0x19, 0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b0111_1101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x19_ora_absolute_y_handles_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_0000;
        cpu.register_y = 0x01;
        cpu.memory.write(0x1012, 0b0000_0000);

        cpu.load_and_run_without_reset(vec![
            0x19, 0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x19_ora_absolute_y_handles_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_1111;
        cpu.register_y = 0x01;
        cpu.memory.write(0x1012, 0b1000_0000);

        cpu.load_and_run_without_reset(vec![
            0x19, 0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b1000_1111);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod ora_indirect_x {
    use super::*;

    #[test]
    fn test_0x01_ora_indirect_x_performs_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0101_0101;
        cpu.register_x = 0x01;
        cpu.memory.write_u16(0x0011, 0x1010);
        cpu.memory.write(0x1010, 0b0100_0100);
        
        cpu.load_and_run_without_reset(vec![
            0x01, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b0101_0101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x01_ora_indirect_x_handles_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_0000;
        cpu.register_x = 0x01;
        cpu.memory.write_u16(0x0011, 0x1010);
        cpu.memory.write(0x1010, 0b0000_0000);

        cpu.load_and_run_without_reset(vec![
            0x01, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x01_ora_indirect_x_handles_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_1111;
        cpu.register_x = 0x01;
        cpu.memory.write_u16(0x0011, 0x1010);
        cpu.memory.write(0x1010, 0b1000_0000);

        cpu.load_and_run_without_reset(vec![
            0x01, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b1000_1111);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod ora_indirect_y {
    use super::*;

    #[test]
    fn test_0x11_ora_indirect_y_performs_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0101_0101;
        cpu.register_y = 0x01;
        cpu.memory.write_u16(0x0010, 0x1010);
        cpu.memory.write(0x1011, 0b0100_0100);
        
        cpu.load_and_run_without_reset(vec![
            0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b0101_0101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x11_ora_indirect_y_handles_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_0000;
        cpu.register_y = 0x01;
        cpu.memory.write_u16(0x0010, 0x1010);
        cpu.memory.write(0x1011, 0b0000_0000);

        cpu.load_and_run_without_reset(vec![
            0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x11_ora_indirect_y_handles_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_1111;
        cpu.register_y = 0x01;
        cpu.memory.write_u16(0x0010, 0x1010);
        cpu.memory.write(0x1011, 0b1000_0000);

        cpu.load_and_run_without_reset(vec![
            0x11, 0x10, 0x00
        ]);

        assert_eq!(cpu.register_a, 0b1000_1111);
        assert_flag(&cpu, Flags::Negative);
    }
}
