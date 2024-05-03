use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use crate::common::{assert_flag, assert_flags};

mod cmp_immediate {
    use super::*;

    #[test]
    fn test_0xc9_cmp_sets_flags_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;

        cpu.load_and_run_without_reset(vec![0xC9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x01);
        assert_flag(&cpu, Flags::Negative);
    }

    #[test]
    fn test_0xc9_cmp_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x26;

        cpu.load_and_run_without_reset(vec![0xC9, 0x26, 0x00]);
        assert_eq!(cpu.register_a, 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xc9_cmp_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;

        cpu.load_and_run_without_reset(vec![0xC9, 0x01, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_flag(&cpu, Flags::Carry);
    }
}

mod cmp_zero_page {
    use super::*;

    #[test]
    fn test_0xc5_cmp_sets_flag_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.memory.write(0x01, 0x05);

        cpu.load_and_run_without_reset(vec![0xC5, 0x01, 0x00]);

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.memory.read(0x01), 0x05);
        assert_flag(&cpu, Flags::Negative)
    }

    #[test]
    fn test_0xc5_cmp_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x26;
        cpu.memory.write(0x01, 0x26);

        cpu.load_and_run_without_reset(vec![0xC5, 0x01, 0x00]);

        assert_eq!(cpu.register_a, 0x26);
        assert_eq!(cpu.memory.read(0x01), 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xc5_cmp_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.memory.write(0x01, 0x01);

        cpu.load_and_run_without_reset(vec![0xC5, 0x01, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.memory.read(0x01), 0x01);
        assert_flag(&cpu, Flags::Carry);
    }
}

mod cmp_zero_page_x {
    use super::*;

    #[test]
    fn test_0xd5_cmp_sets_flag_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.register_x = 0x01;
        cpu.memory.write(0x02, 0x05);

        cpu.load_and_run_without_reset(vec![0xD5, 0x01, 0x00]);

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.memory.read(0x02), 0x05);
        assert_flag(&cpu, Flags::Negative)
    }

    #[test]
    fn test_0xd5_cmp_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x26;
        cpu.register_x = 0x01;
        cpu.memory.write(0x02, 0x26);

        cpu.load_and_run_without_reset(vec![0xD5, 0x01, 0x00]);

        assert_eq!(cpu.register_a, 0x26);
        assert_eq!(cpu.memory.read(0x02), 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xd5_cmp_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.register_x = 0x01;
        cpu.memory.write(0x02, 0x01);

        cpu.load_and_run_without_reset(vec![0xD5, 0x01, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.memory.read(0x02), 0x01);
        assert_flag(&cpu, Flags::Carry);
    }
}

mod cmp_absolute {
    use super::*;

    #[test]
    fn test_0xcd_cmp_sets_flag_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.memory.write(0x1010, 0x05);

        cpu.load_and_run_without_reset(vec![0xCD, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.memory.read(0x1010), 0x05);
        assert_flag(&cpu, Flags::Negative)
    }

    #[test]
    fn test_0xcd_cmp_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x26;
        cpu.memory.write(0x1010, 0x26);

        cpu.load_and_run_without_reset(vec![0xCD, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x26);
        assert_eq!(cpu.memory.read(0x1010), 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xcd_cmp_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.register_x = 0x01;
        cpu.memory.write(0x1010, 0x01);

        cpu.load_and_run_without_reset(vec![0xCD, 0x10, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.memory.read(0x1010), 0x01);
        assert_flag(&cpu, Flags::Carry);
    }
}

mod cmp_absolute_x {
    use super::*;

    #[test]
    fn test_0xdd_cmp_sets_flag_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.register_x = 0x01;
        cpu.memory.write(0x1011, 0x05);

        cpu.load_and_run_without_reset(vec![0xDD, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.memory.read(0x1011), 0x05);
        assert_flag(&cpu, Flags::Negative)
    }

    #[test]
    fn test_0xdd_cmp_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x26;
        cpu.register_x = 0x01;
        cpu.memory.write(0x1011, 0x26);

        cpu.load_and_run_without_reset(vec![0xDD, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x26);
        assert_eq!(cpu.memory.read(0x1011), 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xdd_cmp_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.register_x = 0x01;
        cpu.memory.write(0x1011, 0x01);

        cpu.load_and_run_without_reset(vec![0xDD, 0x10, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.memory.read(0x1011), 0x01);
        assert_flag(&cpu, Flags::Carry);
    }
}

mod cmp_absolute_y {
    use super::*;

    #[test]
    fn test_0xd9_cmp_sets_flag_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.register_y = 0x01;
        cpu.memory.write(0x1011, 0x05);

        cpu.load_and_run_without_reset(vec![0xD9, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.memory.read(0x1011), 0x05);
        assert_flag(&cpu, Flags::Negative)
    }

    #[test]
    fn test_0xd9_cmp_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x26;
        cpu.register_y = 0x01;
        cpu.memory.write(0x1011, 0x26);

        cpu.load_and_run_without_reset(vec![0xD9, 0x10, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x26);
        assert_eq!(cpu.memory.read(0x1011), 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xd9_cmp_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.register_y = 0x01;
        cpu.memory.write(0x1011, 0x01);

        cpu.load_and_run_without_reset(vec![0xD9, 0x10, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.memory.read(0x1011), 0x01);
        assert_flag(&cpu, Flags::Carry);
    }
}

mod cmp_indirect_x {
    use super::*;

    #[test]
    fn test_0xc1_cmp_sets_flag_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.register_x = 0x01;
        cpu.memory.write_u16(0x11, 0x1010);
        cpu.memory.write(0x1010, 0x05);

        cpu.load_and_run_without_reset(vec![0xC1, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.memory.read(0x1010), 0x05);
        assert_flag(&cpu, Flags::Negative)
    }

    #[test]
    fn test_0xc1_cmp_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x26;
        cpu.register_x = 0x01;
        cpu.memory.write_u16(0x11, 0x1010);
        cpu.memory.write(0x1010, 0x26);

        cpu.load_and_run_without_reset(vec![0xC1, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x26);
        assert_eq!(cpu.memory.read(0x1010), 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xc1_cmp_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.register_x = 0x01;
        cpu.memory.write_u16(0x11, 0x1010);
        cpu.memory.write(0x1010, 0x01);

        cpu.load_and_run_without_reset(vec![0xC1, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.memory.read(0x1010), 0x01);
        assert_flag(&cpu, Flags::Carry);
    }
}

mod cmp_indirect_y {
    use super::*;

    #[test]
    fn test_0xd1_cmp_sets_flag_correctly_when_a_lt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.register_y = 0x01;
        cpu.memory.write_u16(0x10, 0x1010);
        cpu.memory.write(0x1011, 0x05);

        cpu.load_and_run_without_reset(vec![0xD1, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.memory.read(0x1011), 0x05);
        assert_flag(&cpu, Flags::Negative)
    }

    #[test]
    fn test_0xd1_cmp_sets_carry_and_zero_flag_when_a_eq_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x26;
        cpu.register_y = 0x01;
        cpu.memory.write_u16(0x10, 0x1010);
        cpu.memory.write(0x1011, 0x26);

        cpu.load_and_run_without_reset(vec![0xD1, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x26);
        assert_eq!(cpu.memory.read(0x1011), 0x26);
        assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
    }

    #[test]
    fn test_0xd1_cmp_sets_carry_flag_when_a_gt_mem() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.register_y = 0x01;
        cpu.memory.write_u16(0x10, 0x1010);
        cpu.memory.write(0x1011, 0x01);

        cpu.load_and_run_without_reset(vec![0xD1, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.memory.read(0x1011), 0x01);
        assert_flag(&cpu, Flags::Carry);
    }
}
