use common::assert_flag;
use nes_emulator::cpu::{Flags, CPU};

mod common;
use crate::common::assert_no_flags;

#[test]
fn test_0x4a_lsr_implied_shifts_right_without_carry_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1010_1010;

    cpu.load_and_run_without_reset(vec![0x4A, 0x00]);

    assert_eq!(cpu.register_a, 0b0101_0101);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x4a_lsr_implied_shifts_right_with_carry_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1010_1011;

    cpu.load_and_run_without_reset(vec![0x4A, 0x00]);

    assert_eq!(cpu.register_a, 0b0101_0101);
    assert_flag(&cpu, Flags::Carry);
}

#[test]
fn test_0x4a_lsr_implied_shifts_right_with_zero_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x00;

    cpu.load_and_run_without_reset(vec![0x4A, 0x00]);

    assert_eq!(cpu.register_a, 0x00);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x46_lsr_zero_page_shifts_right_without_carry_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x0010, 0b1000_1110);

    cpu.load_and_run_without_reset(vec![0x46, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x0010), 0b0100_0111);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x46_lsr_zero_page_shifts_right_with_carry_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x0010, 0b1000_1111);

    cpu.load_and_run_without_reset(vec![0x46, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x0010), 0b0100_0111);
    assert_flag(&cpu, Flags::Carry)
}

#[test]
fn test_0x46_lsr_zero_page_shifts_right_with_zero_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x0010, 0x00);

    cpu.load_and_run_without_reset(vec![0x46, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x0010), 0x00);
    assert_flag(&cpu, Flags::Zero)
}

#[test]
fn test_0x56_lsr_zero_page_x_shifts_right_without_carry_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;
    cpu.memory.write(0x0011, 0b1000_1110);

    cpu.load_and_run_without_reset(vec![0x56, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x0011), 0b0100_0111);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x56_lsr_zero_page_x_shifts_right_with_carry_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;
    cpu.memory.write(0x0011, 0b1000_1111);

    cpu.load_and_run_without_reset(vec![0x56, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x0011), 0b0100_0111);
    assert_flag(&cpu, Flags::Carry);
}

#[test]
fn test_0x56_lsr_zero_page_x_shifts_right_with_zero_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;
    cpu.memory.write(0x0011, 0x00);

    cpu.load_and_run_without_reset(vec![0x56, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x0011), 0x00);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x4e_lsr_absolute_shifts_right_without_carry_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x1011, 0b0100_0110);

    cpu.load_and_run_without_reset(vec![0x4E, 0x11, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x1011), 0b0010_0011);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x4e_lsr_absolute_shifts_right_with_carry_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x1011, 0b0011_1111);

    cpu.load_and_run_without_reset(vec![0x4E, 0x11, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x1011), 0b0001_1111);
    assert_flag(&cpu, Flags::Carry);
}

#[test]
fn test_0x4e_lsr_absolute_shifts_right_with_zero_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x1011, 0x00);

    cpu.load_and_run_without_reset(vec![0x4E, 0x11, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x1011), 0x00);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x5e_lsr_absolute_x_shifts_right_without_carry_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;
    cpu.memory.write(0x1012, 0b1010_1010);

    cpu.load_and_run_without_reset(vec![0x5E, 0x11, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x1012), 0b0101_0101);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x5e_lsr_absolute_x_shifts_right_with_carry_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;
    cpu.memory.write(0x1012, 0b1010_1011);

    cpu.load_and_run_without_reset(vec![0x5E, 0x11, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x1012), 0b0101_0101);
    assert_flag(&cpu, Flags::Carry);
}

#[test]
fn test_0x5e_lsr_absolute_x_shifts_right_with_zero_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;
    cpu.memory.write(0x1012, 0x00);

    cpu.load_and_run_without_reset(vec![0x5E, 0x11, 0x10, 0x00]);

    assert_eq!(cpu.memory.read(0x1012), 0x00);
    assert_flag(&cpu, Flags::Zero);
}
