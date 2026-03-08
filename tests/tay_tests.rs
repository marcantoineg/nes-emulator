use std::vec;

use nes_emulator::cpu::{Flags, CPU};

mod common;
use common::{assert_flags, only_break_flag_set};

#[test]
fn test_0xa8_tay_implied_copy_data() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;

    cpu.load_and_run_n_without_reset(vec![0xA8], 1);

    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.register_y, 0x01);
    only_break_flag_set(&cpu);
}

#[test]
fn test_0xa8_tay_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x00;

    cpu.load_and_run_n_without_reset(vec![0xA8], 1);

    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.register_y, 0x00);
    assert_flags(&cpu, vec![Flags::Zero]);
}

#[test]
fn test_0xa8_tay_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;

    cpu.load_and_run_n_without_reset(vec![0xA8], 1);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_eq!(cpu.register_y, 0b1000_0000);
    assert_flags(&cpu, vec![Flags::Negative]);
}
