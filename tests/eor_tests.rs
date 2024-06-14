use nes_emulator::cpu::{Flags, CPU};
use std::vec;

mod common;
use crate::common::{assert_flag, assert_no_flags};

mod eor_immediate {
    use super::*;

    #[test]
    fn test_0x49_eor_performs_exclusive_or_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1101_0101;

        cpu.load_and_run_without_reset(vec![0x49, 0b1010_1010, 0x00]);

        assert_eq!(cpu.register_a, 0b0111_1111);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x49_eor_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;

        cpu.load_and_run_without_reset(vec![0x49, 0x01, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x49_eor_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0000;

        cpu.load_and_run_without_reset(vec![0x49, 0x00, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0000);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod eor_zero_page {
    use super::*;

    #[test]
    fn test_0x45_eor_performs_exclusive_or_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1101_0101;
        cpu.memory.write(0x10, 0b1010_1010);

        cpu.load_and_run_without_reset(vec![0x45, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b0111_1111);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x45_eor_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.memory.write(0x10, 0x01);

        cpu.load_and_run_without_reset(vec![0x45, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x45_eor_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0000;
        cpu.memory.write(0x10, 0x00);

        cpu.load_and_run_without_reset(vec![0x45, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0000);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod eor_zero_page_x {
    use super::*;

    #[test]
    fn test_0x55_eor_performs_exclusive_or_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1101_0101;
        cpu.register_x = 0x01;
        cpu.memory.write(0x11, 0b1010_1010);

        cpu.load_and_run_without_reset(vec![0x55, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b0111_1111);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x55_eor_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.register_x = 0x01;
        cpu.memory.write(0x11, 0x01);

        cpu.load_and_run_without_reset(vec![0x55, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x55_eor_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0000;
        cpu.register_x = 0x01;
        cpu.memory.write(0x11, 0x00);

        cpu.load_and_run_without_reset(vec![0x55, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0000);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod eor_absolute {
    use super::*;

    #[test]
    fn test_0x4d_eor_performs_exclusive_or_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1101_0101;
        cpu.memory.write(0x1010, 0b1010_1010);

        cpu.load_and_run_without_reset(vec![0x4D, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b0111_1111);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x4d_eor_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.memory.write(0x1010, 0x01);

        cpu.load_and_run_without_reset(vec![0x4D, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x4d_eor_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0000;
        cpu.memory.write(0x1010, 0x00);

        cpu.load_and_run_without_reset(vec![0x4D, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0000);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod eor_absolute_x {
    use super::*;

    #[test]
    fn test_0x5d_eor_performs_exclusive_or_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1101_0101;
        cpu.register_x = 0x01;
        cpu.memory.write(0x1011, 0b1010_1010);

        cpu.load_and_run_without_reset(vec![0x5D, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b0111_1111);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x5d_eor_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.register_x = 0x01;
        cpu.memory.write(0x1011, 0x01);

        cpu.load_and_run_without_reset(vec![0x5D, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x5d_eor_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0000;
        cpu.register_x = 0x01;
        cpu.memory.write(0x1011, 0x00);

        cpu.load_and_run_without_reset(vec![0x5D, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0000);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod eor_absolute_y {
    use super::*;

    #[test]
    fn test_0x59_eor_performs_exclusive_or_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1101_0101;
        cpu.register_y = 0x01;
        cpu.memory.write(0x1011, 0b1010_1010);

        cpu.load_and_run_without_reset(vec![0x59, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b0111_1111);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x59_eor_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.register_y = 0x01;
        cpu.memory.write(0x1011, 0x01);

        cpu.load_and_run_without_reset(vec![0x59, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x59_eor_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0000;
        cpu.register_y = 0x01;
        cpu.memory.write(0x1011, 0x00);

        cpu.load_and_run_without_reset(vec![0x59, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0000);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod eor_indirect_x {
    use super::*;

    #[test]
    fn test_0x41_eor_performs_exclusive_or_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1101_0101;
        cpu.register_x = 0x01;
        cpu.memory.write_u16(0x11, 0x1010);
        cpu.memory.write(0x1010, 0b1010_1010);

        cpu.load_and_run_without_reset(vec![0x41, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b0111_1111);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x41_eor_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.register_x = 0x01;
        cpu.memory.write_u16(0x11, 0x1010);
        cpu.memory.write(0x1010, 0x01);

        cpu.load_and_run_without_reset(vec![0x41, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x41_eor_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0000;
        cpu.register_x = 0x01;
        cpu.memory.write_u16(0x11, 0x1010);
        cpu.memory.write(0x1010, 0x00);

        cpu.load_and_run_without_reset(vec![0x41, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0000);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod eor_indirect_y {
    use super::*;

    #[test]
    fn test_0x51_eor_performs_exclusive_or_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1101_0101;
        cpu.register_y = 0x01;
        cpu.memory.write_u16(0x10, 0x1010);
        cpu.memory.write(0x1011, 0b1010_1010);

        cpu.load_and_run_without_reset(vec![0x51, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b0111_1111);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x51_eor_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.register_y = 0x01;
        cpu.memory.write_u16(0x10, 0x1010);
        cpu.memory.write(0x1011, 0x01);

        cpu.load_and_run_without_reset(vec![0x51, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0x51_eor_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0000;
        cpu.register_y = 0x01;
        cpu.memory.write_u16(0x10, 0x1010);
        cpu.memory.write(0x1011, 0x00);

        cpu.load_and_run_without_reset(vec![0x51, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0b1000_0000);
        assert_flag(&cpu, Flags::Negative);
    }
}
