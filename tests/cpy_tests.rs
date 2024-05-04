use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use crate::common::{assert_flag, assert_flags};

mod cpy_immediate {
    use super::*;

    #[test]
    fn test_0xc0_cpy_sets_flags_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x01;

        cpu.load_and_run_without_reset(vec![0xC0, 0x05, 0x00]);
        assert_eq!(cpu.register_y, 0x01);
        assert_flag(&cpu, Flags::Negative);
    }

    #[test]
    fn test_0xc0_cpy_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x26;

        cpu.load_and_run_without_reset(vec![0xC0, 0x26, 0x00]);
        assert_eq!(cpu.register_y, 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xc0_cpy_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x05;

        cpu.load_and_run_without_reset(vec![0xC0, 0x01, 0x00]);
        assert_eq!(cpu.register_y, 0x05);
        assert_flag(&cpu, Flags::Carry);
    }
}

mod cpy_zero_page {
    use super::*;

    #[test]
    fn test_0xc4_cpy_sets_flags_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x01;
        cpu.memory.write(0x01, 0x05);

        cpu.load_and_run_without_reset(vec![0xC4, 0x01, 0x00]);

        assert_eq!(cpu.register_y, 0x01);
        assert_eq!(cpu.memory.read(0x01), 0x05);
        assert_flag(&cpu, Flags::Negative);
    }

    #[test]
    fn test_0xc4_cpy_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x26;
        cpu.memory.write(0x01, 0x26);

        cpu.load_and_run_without_reset(vec![0xC4, 0x01, 0x00]);

        assert_eq!(cpu.register_y, 0x26);
        assert_eq!(cpu.memory.read(0x01), 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xc4_cpy_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x05;
        cpu.memory.write(0x01, 0x01);

        cpu.load_and_run_without_reset(vec![0xC4, 0x01, 0x00]);

        assert_eq!(cpu.register_y, 0x05);
        assert_eq!(cpu.memory.read(0x01), 0x01);
        assert_flag(&cpu, Flags::Carry);
    }
}

mod cpy_absolute {
    use super::*;

    #[test]
    fn test_0xcc_cpy_sets_flags_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x01;
        cpu.memory.write(0x1010, 0x05);

        cpu.load_and_run_without_reset(vec![0xCC, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_y, 0x01);
        assert_eq!(cpu.memory.read(0x1010), 0x05);
        assert_flag(&cpu, Flags::Negative);
    }

    #[test]
    fn test_0xcc_cpy_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x26;
        cpu.memory.write(0x1010, 0x26);

        cpu.load_and_run_without_reset(vec![0xCC, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_y, 0x26);
        assert_eq!(cpu.memory.read(0x1010), 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xcc_cpy_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x05;
        cpu.memory.write(0x1010, 0x01);

        cpu.load_and_run_without_reset(vec![0xCC, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_y, 0x05);
        assert_eq!(cpu.memory.read(0x1010), 0x01);
        assert_flag(&cpu, Flags::Carry);
    }
}