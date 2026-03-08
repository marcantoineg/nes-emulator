use nes_emulator::cpu::{Flags, CPU};
use std::vec;

mod common;
use crate::common::{assert_flags, only_break_flag_set};

#[test]
fn test_0xca_dex_implied_decrement_register_x_correcly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x02;

    cpu.load_and_run_without_reset(vec![0xCA, 0x00]);

    assert_eq!(cpu.register_x, 0x01);
    only_break_flag_set(&cpu);
}

#[test]
fn test_0xca_dex_implied_sets_zero_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;

    cpu.load_and_run_without_reset(vec![0xCA, 0x00]);

    assert_eq!(cpu.register_x, 0x00);
    assert_flags(&cpu, vec![Flags::Zero]);
}

#[test]
fn test_0xca_dex_implied_sets_negative_flag_and_wraps_correctly() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x00;

    cpu.load_and_run_without_reset(vec![0xCA, 0x00]);

    assert_eq!(cpu.register_x, 0xFF);
    assert_flags(&cpu, vec![Flags::Negative]);
}
