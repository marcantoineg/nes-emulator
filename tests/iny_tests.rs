use std::vec;

use nes_emulator::cpu::{Flags, CPU};

mod common;
use common::{assert_flag, assert_flags, assert_no_flags};

#[test]
fn test_0xc8_iny_implied_increment_x() {
    let mut cpu = CPU::new();
    cpu.register_y = 0x01;

    cpu.load_and_run_without_reset(vec![0xC8, 0x00]);

    assert_eq!(cpu.register_y, 0x02);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xc8_iny_implied_overflow() {
    let mut cpu = CPU::new();
    cpu.register_y = 0xff;

    cpu.load_and_run_without_reset(vec![0xC8, 0x00]);

    assert_eq!(cpu.register_y, 0);
    assert_flags(&cpu, vec![Flags::Zero])
}

#[test]
fn test_0xc8_iny_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 0xFF;

    cpu.load_and_run_without_reset(vec![0xC8, 0x00]);

    assert_eq!(cpu.register_y, 0x00);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0xc8_iny_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 0b0111_1111;

    cpu.load_and_run_without_reset(vec![0xC8, 0x00]);

    assert_eq!(cpu.register_y, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}
