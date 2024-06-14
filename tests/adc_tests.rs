use nes_emulator::cpu::{Flags, CPU};
use std::vec;

mod common;
use common::{assert_flag, assert_flags, assert_no_flags};

#[test]
fn test_0x69_adc_immediate_adds_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;

    cpu.load_and_run_without_reset(vec![0x69, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0x02);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x69_adc_immediate_carry_in() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Carry);
    cpu.register_a = 0x01;

    cpu.load_and_run_without_reset(vec![0x69, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0x03);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x69_adc_immediate_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x00;

    cpu.load_and_run_without_reset(vec![0x69, 0x00, 0x00]);

    assert_eq!(cpu.register_a, 0x00);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x69_adc_immediate_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0;

    cpu.load_and_run_without_reset(vec![0x69, 0b1000_0001, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0001);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x69_adc_immediate_carry_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0xFF;

    cpu.load_and_run_without_reset(vec![0x69, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0x00);
    assert_flags(&cpu, vec![Flags::Zero, Flags::Carry]);
}

#[cfg(test)]
mod adc_overflow_flag_tests {
    use super::*;
    // reproducing the tests table at:
    // http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html

    #[test]
    fn test_0x69_adc_immediate_overflow_1() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x50;

        cpu.load_and_run_without_reset(vec![0x69, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x60);
        assert_no_flags(&cpu)
    }

    #[test]
    fn test_0x69_adc_immediate_overflow_2() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x50;

        cpu.load_and_run_without_reset(vec![0x69, 0x50, 0x00]);

        assert_eq!(cpu.register_a, 0xA0);
        assert_flags(&cpu, vec![Flags::Negative, Flags::Overflow]);
    }

    #[test]
    fn test_0x69_adc_immediate_overflow_3() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x50;

        cpu.load_and_run_without_reset(vec![0x69, 0x90, 0x00]);

        assert_eq!(cpu.register_a, 0xE0);
        assert_flags(&cpu, vec![Flags::Negative])
    }

    #[test]
    fn test_0x69_adc_immediate_overflow_4() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x50;

        cpu.load_and_run_without_reset(vec![0x69, 0xD0, 0x00]);

        assert_eq!(cpu.register_a, 0x20);
        assert_flag(&cpu, Flags::Carry);
    }

    #[test]
    fn test_0x69_adc_immediate_overflow_5() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xD0;

        cpu.load_and_run_without_reset(vec![0x69, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0xE0);
        assert_flag(&cpu, Flags::Negative);
    }

    #[test]
    fn test_0x69_adc_immediate_overflow_6() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xD0;

        cpu.load_and_run_without_reset(vec![0x69, 0x50, 0x00]);

        assert_eq!(cpu.register_a, 0x20);
        assert_flag(&cpu, Flags::Carry);
    }

    #[test]
    fn test_0x69_adc_immediate_overflow_7() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xD0;

        cpu.load_and_run_without_reset(vec![0x69, 0x90, 0x00]);

        assert_eq!(cpu.register_a, 0x60);
        assert_flags(&cpu, vec![Flags::Overflow, Flags::Carry]);
    }

    #[test]
    fn test_0x69_adc_immediate_overflow_8() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xD0;

        cpu.load_and_run_without_reset(vec![0x69, 0xD0, 0x00]);

        assert_eq!(cpu.register_a, 0xA0);
        assert_flags(&cpu, vec![Flags::Negative, Flags::Carry]);
    }
}

#[test]
fn test_0x65_adc_zero_page_adds_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;
    cpu.memory.write(0x0001, 0x05);

    cpu.load_and_run_without_reset(vec![0x65, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0x06);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x75_adc_zero_page_x_adds_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory.write(0x02, 0x05);

    cpu.load_and_run_without_reset(vec![0x75, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0x06);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x6d_adc_absolute_adds_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;
    cpu.memory.write(0x1110, 0x05);

    cpu.load_and_run_without_reset(vec![0x6D, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0x06);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x7d_adc_absolute_x_adds_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory.write(0x1111, 0x01);

    cpu.load_and_run_without_reset(vec![0x7D, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0x02);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x79_adc_absolute_y_adds_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;
    cpu.register_y = 0x01;
    cpu.memory.write(0x1111, 0x01);

    cpu.load_and_run_without_reset(vec![0x79, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0x02);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x61_adc_indirect_x_adds_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory.write(0x0002, 0x11);
    cpu.memory.write_u16(0x0011, 0x0011);

    cpu.load_and_run_without_reset(vec![0x61, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0x0012);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x71_adc_indirect_y_adds_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;
    cpu.register_y = 0x01;
    cpu.memory.write_u16(0x0001, 0x0111);
    cpu.memory.write_u16(0x0112, 0x02);

    cpu.load_and_run_without_reset(vec![0x71, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0x03);
    assert_no_flags(&cpu);
}
