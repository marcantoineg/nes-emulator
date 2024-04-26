use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use common::{assert_flag, assert_no_flags};

#[test]
fn test_0x0a_asl_implied_shifts_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0000_0001;

    cpu.load_and_run_without_reset(vec![0x0A, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0010);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x0a_asl_implied_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0000_0000;

    cpu.load_and_run_without_reset(vec![0x0A, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0000);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x0a_asl_implied_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0100_0000;

    cpu.load_and_run_without_reset(vec![0x0A, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x0a_asl_implied_carry_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0001;

    cpu.load_and_run_without_reset(vec![0x0a, 0x00]);
    
    assert_eq!(cpu.register_a, 0b0000_0010);
    assert_flag(&cpu, Flags::Carry);
}

#[test]
fn test_0x06_asl_zero_page_shifts_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x0010, 0b0000_0001);

    cpu.load_and_run_without_reset(vec![0x06, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x10), 0b0000_0010);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x16_asl_zero_page_x_shifts_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;
    cpu.memory.write(0x0011, 0b0000_0001);

    cpu.load_and_run_without_reset(vec![0x16, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x0011), 0b0000_0010);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x0e_asl_absolute_shifts_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x1010, 0b0000_0001);

    cpu.load_and_run_without_reset(vec![0x0E, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x1010), 0b0000_0010);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x1e_asl_absolute_x_shifts_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;
    cpu.memory.write_u16(0x1010, 0x1010);
    cpu.memory.write(0x1011, 0b0000_0001);

    cpu.load_and_run_without_reset(vec![0x1E, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x1011), 0b0000_0010);
    assert_no_flags(&cpu);
}
