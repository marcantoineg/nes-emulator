use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use crate::common::{assert_flag, assert_no_flags};

#[test]
fn test_0xca_dex_implied_decrement_register_x_correcly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x02;

    cpu.load_and_run_without_reset(vec![0xCA, 0x00]);

    assert_eq!(cpu.register_x, 0x01);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xca_dex_implied_sets_zero_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;

    cpu.load_and_run_without_reset(vec![0xCA, 0x00]);

    assert_eq!(cpu.register_x, 0x00);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0xca_dex_implied_sets_negative_flag_and_wraps_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x00;

    cpu.load_and_run_without_reset(vec![0xCA, 0x00]);

    assert_eq!(cpu.register_x, 0xFF);
    assert_flag(&cpu, Flags::Negative);
}
