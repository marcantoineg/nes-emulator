use std::vec;

use nes_emulator::cpu::{Flags, CPU};

mod common;
use crate::common::{assert_flag, assert_no_flags};

mod inc_zero_page {
    use super::*;

    #[test]
    fn test_0xe6_inc_increments_memory_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x01, 0x01);

        cpu.load_and_run_without_reset(vec![0xE6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x01), 0x02);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0xe6_inc_wraps_and_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x01, 0xFF);

        cpu.load_and_run_without_reset(vec![0xE6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x01), 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0xe6_inc_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x01, 0x7F);

        cpu.load_and_run_without_reset(vec![0xE6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x01), 0x80);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod inc_zero_page_x {
    use super::*;

    #[test]
    fn test_0xf6_inc_increments_memory_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.memory.write(0x02, 0x01);

        cpu.load_and_run_without_reset(vec![0xF6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x02), 0x02);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0xe6_inc_wraps_and_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.memory.write(0x02, 0xFF);

        cpu.load_and_run_without_reset(vec![0xF6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x01), 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0xe6_inc_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.memory.write(0x02, 0x7F);

        cpu.load_and_run_without_reset(vec![0xF6, 0x01, 0x00]);

        assert_eq!(cpu.memory.read(0x02), 0x80);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod inc_absolute {
    use super::*;

    #[test]
    fn test_0xee_inc_increments_memory_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x1010, 0x01);

        cpu.load_and_run_without_reset(vec![0xEE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1010), 0x02);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0xee_inc_wraps_and_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x1010, 0xFF);

        cpu.load_and_run_without_reset(vec![0xEE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1010), 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0xee_inc_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x1010, 0x7F);

        cpu.load_and_run_without_reset(vec![0xEE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1010), 0x80);
        assert_flag(&cpu, Flags::Negative);
    }
}

mod inc_absolute_x {
    use super::*;
    
    #[test]
    fn test_0xfe_inc_increments_memory_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.memory.write(0x1011, 0x01);

        cpu.load_and_run_without_reset(vec![0xFE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1011), 0x02);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0xfe_inc_wraps_and_sets_zero_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.memory.write(0x1011, 0xFF);

        cpu.load_and_run_without_reset(vec![0xFE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1011), 0x00);
        assert_flag(&cpu, Flags::Zero);
    }

    #[test]
    fn test_0xfe_inc_sets_negative_flag_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.memory.write(0x1011, 0x7F);

        cpu.load_and_run_without_reset(vec![0xFE, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.memory.read(0x1011), 0x80);
        assert_flag(&cpu, Flags::Negative);
    }
}
