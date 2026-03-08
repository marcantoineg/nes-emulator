use std::vec;

use nes_emulator::cpu::{Flags, CPU};

mod common;
use common::{assert_flags, only_break_flag_set};

#[test]
fn test_0xe8_inx_implied_increment_x() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;

    cpu.load_and_run_n_without_reset(vec![0xE8], 1);

    assert_eq!(cpu.register_x, 0x02);
    assert_flags(&cpu, vec![]);
}

#[test]
fn test_0xe8_inx_implied_overflow() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;

    cpu.load_and_run_n_without_reset(vec![0xE8], 1);

    assert_eq!(cpu.register_x, 0);
    assert_flags(&cpu, vec![Flags::Zero])
}

#[test]
fn test_0xe8_inx_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xFF;

    cpu.load_and_run_n_without_reset(vec![0xE8], 1);

    assert_eq!(cpu.register_x, 0x00);
    assert_flags(&cpu, vec![Flags::Zero])
}

#[test]
fn test_0xe8_inx_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 0b0111_1111;

    cpu.load_and_run_n_without_reset(vec![0xE8], 1);

    assert_eq!(cpu.register_x, 0b1000_0000);
    assert_flags(&cpu, vec![Flags::Negative]);
}
