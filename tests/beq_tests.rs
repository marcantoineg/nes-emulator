use nes_emulator::cpu::{Flags, CPU};
use std::vec;

mod common;
use common::{assert_flag, assert_no_flags};

#[test]
fn test_0xf0_beq_relative_branches_forward_correctly() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0x00, /*BEQ+2*/ 0xF0, 0x02, /*LDA*/ 0xA9, 0x05, 0x00,
    ]);

    assert_eq!(cpu.register_a, 0x00);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0xf0_beq_relative_branches_backward_correctly() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0xFF, /*ADC*/ 0x69, 0x01, /*BEQ-4*/ 0xF0, 0x84, 0x00,
    ]);

    assert_eq!(cpu.register_a, 0x02);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xf0_beq_relative_ignores_branching_when_offset_is_zero() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0x00, /*BEQ+-0*/ 0xF0, 0x00, /*LDA*/ 0xA9, 0x05,
    ]);

    assert_eq!(cpu.register_a, 0x05);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xf0_beq_relative_ignores_branching_when_condition_is_not_met() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0x01, /*BEQ+2*/ 0xF0, 0x00, /*LDA*/ 0xA9, 0x05,
    ]);

    assert_eq!(cpu.register_a, 0x05);
    assert_no_flags(&cpu);
}
