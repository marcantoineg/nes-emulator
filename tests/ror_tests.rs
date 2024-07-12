use nes_emulator::cpu::{Flags, CPU};

mod common;
use common::{assert_flag, assert_flags, assert_no_flags};

mod tests_0x6a_ror_implied {
    use super::*;

    #[test]
    fn test_0x6a_ror_implied_shifts_accumulator_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0010_1010;

        cpu.load_and_run_without_reset(vec![0x6A, 0x00]);

        assert_eq!(cpu.register_a, 0b0001_0101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x6a_ror_implied_shifts_accumulator_with_carry_in_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0010_1010;
        cpu.status.insert(Flags::Carry);

        cpu.load_and_run_without_reset(vec![0x6A, 0x00]);

        assert_eq!(cpu.register_a, 0b1001_0101);
        assert_flag(&cpu, Flags::Negative);
    }

    #[test]
    fn test_0x6a_ror_implied_shifts_accumulator_with_carry_out_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1010_1011;

        cpu.load_and_run_without_reset(vec![0x6A, 0x00]);

        assert_eq!(cpu.register_a, 0b0101_0101);
        assert_flag(&cpu, Flags::Carry);
    }

    #[test]
    fn test_0x6a_ror_implied_shifts_accumulator_with_zero_flag_out_correctly() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_0001;

        cpu.load_and_run_without_reset(vec![0x6A, 0x00]);

        assert_eq!(cpu.register_a, 0b0000_0000);
        assert_flags(&cpu, vec![Flags::Zero, Flags::Carry]);
    }
}

mod tests_0x66_ror_zero_page {
    use super::*;

    #[test]
    fn test_0x66_ror_zero_page_shifts_mem_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x05, 0b0010_1010);

        cpu.load_and_run_without_reset(vec![0x66, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x05), 0b0001_0101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x66_ror_zero_page_shifts_mem_with_carry_in_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x05, 0b0010_1010);
        cpu.status.insert(Flags::Carry);

        cpu.load_and_run_without_reset(vec![0x66, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x05), 0b1001_0101);
        assert_flag(&cpu, Flags::Negative);
    }

    #[test]
    fn test_0x66_ror_zero_page_shifts_mem_with_carry_out_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x05, 0b1010_1011);

        cpu.load_and_run_without_reset(vec![0x66, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x05), 0b0101_0101);
        assert_flag(&cpu, Flags::Carry);
    }

    #[test]
    fn test_0x66_ror_zero_page_shifts_mem_with_zero_flag_out_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x05, 0b0000_0001);

        cpu.load_and_run_without_reset(vec![0x66, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x05), 0b0000_0000);
        assert_flags(&cpu, vec![Flags::Zero, Flags::Carry]);
    }
}

mod tests_0x76_ror_zero_page_x {
    use super::*;

    #[test]
    fn test_0x76_ror_zero_page_x_shifts_mem_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.memory.write(0x06, 0b0010_1010);

        cpu.load_and_run_without_reset(vec![0x76, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x06), 0b0001_0101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x76_ror_zero_page_x_shifts_mem_with_carry_in_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.memory.write(0x06, 0b0010_1010);
        cpu.status.insert(Flags::Carry);

        cpu.load_and_run_without_reset(vec![0x76, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x06), 0b1001_0101);
        assert_flag(&cpu, Flags::Negative);
    }

    #[test]
    fn test_0x76_ror_zero_page_x_shifts_mem_with_carry_out_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.memory.write(0x06, 0b1010_1011);

        cpu.load_and_run_without_reset(vec![0x76, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x06), 0b0101_0101);
        assert_flag(&cpu, Flags::Carry);
    }

    #[test]
    fn test_0x76_ror_zero_page_x_shifts_mem_with_zero_flag_out_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.memory.write(0x06, 0b0000_0001);

        cpu.load_and_run_without_reset(vec![0x76, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x06), 0b0000_0000);
        assert_flags(&cpu, vec![Flags::Zero, Flags::Carry]);
    }
}

mod tests_0x6e_ror_absolute {
    use super::*;

    #[test]
    fn test_0x6e_ror_absolute_shifts_mem_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x0505, 0b0010_1010);

        cpu.load_and_run_without_reset(vec![0x6E, 0x05, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x0505), 0b0001_0101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x6e_ror_absolute_shifts_mem_with_carry_in_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x0505, 0b0010_1010);
        cpu.status.insert(Flags::Carry);

        cpu.load_and_run_without_reset(vec![0x6E, 0x05, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x0505), 0b1001_0101);
        assert_flag(&cpu, Flags::Negative);
    }

    #[test]
    fn test_0x6e_ror_absolute_shifts_mem_with_carry_out_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x0505, 0b1010_1011);

        cpu.load_and_run_without_reset(vec![0x6E, 0x05, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x0505), 0b0101_0101);
        assert_flag(&cpu, Flags::Carry);
    }

    #[test]
    fn test_0x6e_ror_absolute_shifts_mem_with_zero_flag_out_correctly() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x0505, 0b0000_0001);

        cpu.load_and_run_without_reset(vec![0x6E, 0x05, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x0505), 0b0000_0000);
        assert_flags(&cpu, vec![Flags::Zero, Flags::Carry]);
    }
}

mod tests_0x7e_ror_absolute_x {
    use super::*;

    #[test]
    fn test_0x7e_ror_absolute_x_shifts_mem_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x02;
        cpu.memory.write(0x0507, 0b0010_1010);

        cpu.load_and_run_without_reset(vec![0x7E, 0x05, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x0507), 0b0001_0101);
        assert_no_flags(&cpu);
    }

    #[test]
    fn test_0x7e_ror_absolute_x_shifts_mem_with_carry_in_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x02;
        cpu.memory.write(0x0507, 0b0010_1010);
        cpu.status.insert(Flags::Carry);

        cpu.load_and_run_without_reset(vec![0x7E, 0x05, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x0507), 0b1001_0101);
        assert_flag(&cpu, Flags::Negative);
    }

    #[test]
    fn test_0x7e_ror_absolute_x_shifts_mem_with_carry_out_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x02;
        cpu.memory.write(0x0507, 0b1010_1011);

        cpu.load_and_run_without_reset(vec![0x7E, 0x05, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x0507), 0b0101_0101);
        assert_flag(&cpu, Flags::Carry);
    }

    #[test]
    fn test_0x7e_ror_absolute_x_shifts_mem_with_zero_flag_out_correctly() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x02;
        cpu.memory.write(0x0507, 0b0000_0001);

        cpu.load_and_run_without_reset(vec![0x7E, 0x05, 0x05, 0x00]);

        assert_eq!(cpu.memory.read(0x0507), 0b0000_0000);
        assert_flags(&cpu, vec![Flags::Zero, Flags::Carry]);
    }
}
