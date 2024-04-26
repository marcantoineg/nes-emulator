use std::vec;

use nes_emulator::cpu::{Flags, CPU};

mod common;
use common::{assert_flag, assert_no_flags};

#[test]
fn test_0xa8_tay_implied_copy_data() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;

    cpu.load_and_run_without_reset(vec![0xA8, 0x00]);

    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.register_y, 0x01);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xa8_tay_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x00;

    cpu.load_and_run_without_reset(vec![0xA8, 0x00]);

    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.register_y, 0x00);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0xa8_tay_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;

    cpu.load_and_run_without_reset(vec![0xA8, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_eq!(cpu.register_y, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}
