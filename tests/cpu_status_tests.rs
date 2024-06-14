use nes_emulator::cpu::{Flags, CPU};

mod common;
use crate::common::{assert_flag, assert_no_flags};

#[test]
fn test_cpu_flag_carry() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Carry);

    assert_eq!(cpu.status.bits(), 0b0011_0001);
    assert_flag(&cpu, Flags::Carry);
}

#[test]
fn test_0x18_clc_implied_clears_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Carry);

    cpu.load_and_run_without_reset(vec![0x18, 0x00]);

    assert_eq!(cpu.status.bits(), 0b0011_0000);
    assert_no_flags(&cpu)
}

#[test]
fn test_cpu_flag_zero() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Zero);

    assert_eq!(cpu.status.bits(), 0b0011_0010);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_cpu_flag_interupt_disable() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::InteruptDisable);

    assert_eq!(cpu.status.bits(), 0b0011_0100);
    assert_flag(&cpu, Flags::InteruptDisable);
}

#[test]
fn test_0x58_cli_implied_clears_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::InteruptDisable);

    cpu.load_and_run_without_reset(vec![0x58, 0x00]);

    assert_eq!(cpu.status.bits(), 0b0011_0000);
    assert_no_flags(&cpu)
}

#[test]
fn test_cpu_flag_decimal_mode() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Decimal);

    assert_eq!(cpu.status.bits(), 0b0011_1000);
    assert_flag(&cpu, Flags::Decimal);
}

#[test]
fn test_0xd8_cld_implied_clears_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Decimal);

    cpu.load_and_run_without_reset(vec![0xD8, 0x00]);

    assert_eq!(cpu.status.bits(), 0b0011_0000);
    assert_no_flags(&cpu)
}

#[test]
fn test_cpu_flag_break() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Break);

    assert_eq!(cpu.status.bits(), 0b0011_0000);
    assert_flag(&cpu, Flags::Break);
}

#[test]
fn test_cpu_flag_overflow() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Overflow);

    assert_eq!(cpu.status.bits(), 0b0111_0000);
    assert_flag(&cpu, Flags::Overflow);
}

#[test]
fn test_0xb8_clv_implied_clears_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Overflow);

    cpu.load_and_run_without_reset(vec![0xB8, 0x00]);

    assert_eq!(cpu.status.bits(), 0b0011_0000);
    assert_no_flags(&cpu)
}

#[test]
fn test_cpu_flag_negative() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Negative);

    assert_eq!(cpu.status.bits(), 0b1011_0000);
    assert_flag(&cpu, Flags::Negative);
}
