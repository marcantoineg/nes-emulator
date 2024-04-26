use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use common::{assert_flag, assert_no_flags};

#[test]
fn test_0x24_bit_zero_page_sets_zero_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x01, 0b0000_1111);
    cpu.register_a = 0b0011_0000;

    cpu.load_and_run_without_reset(vec![0x24, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b0011_0000);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x24_bit_zero_page_sets_negative_and_overflow_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x01, 0b1000_0000);
    cpu.register_a = 0b1000_0000;

    cpu.load_and_run_without_reset(vec![0x24, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x24_bit_zero_page_sets_overflow_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x01, 0b0100_0000);
    cpu.register_a = 0b0100_0000;

    cpu.load_and_run_without_reset(vec![0x24, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b0100_0000);
    assert_flag(&cpu, Flags::Overflow)
}

#[test]
fn test_0x24_bit_zero_page_does_not_set_zero_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x01, 0b0000_0011);
    cpu.register_a = 0b0000_0001;

    cpu.load_and_run_without_reset(vec![0x24, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0001);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x2c_bit_absolute_sets_zero_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x0101, 0b0000_1100);
    cpu.register_a = 0b0000_0011;

    cpu.load_and_run_without_reset(vec![0x2C, 0x01, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0011);
    assert_flag(&cpu, Flags::Zero)
}

#[test]
fn test_0x2c_bit_absolute_sets_negative_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x0101, 0b1000_0000);
    cpu.register_a = 0b1000_0000;

    cpu.load_and_run_without_reset(vec![0x2C, 0x01, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x2c_bit_absolute_sets_overflow_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x0101, 0b0100_0000);
    cpu.register_a = 0b0100_0000;

    cpu.load_and_run_without_reset(vec![0x2C, 0x01, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b0100_0000);
    assert_flag(&cpu, Flags::Overflow);
}

#[test]
fn test_0x2c_bit_absolute_does_not_set_zero_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x0101, 0b0000_0011);
    cpu.register_a = 0b0000_0001;

    cpu.load_and_run_without_reset(vec![0x2C, 0x01, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0001);
    assert_no_flags(&cpu)
}
