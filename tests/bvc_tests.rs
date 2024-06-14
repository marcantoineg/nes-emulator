use nes_emulator::cpu::{Flags, CPU};
use std::vec;

mod common;
use common::{assert_flags, assert_no_flags};

#[test]
fn test_0x50_bvc_relative_branches_forward_correctly() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*BVC+2*/ 0x50, 0x02, /*LDA*/ 0xA9, 0x02, 0x00,
    ]);

    assert_eq!(cpu.register_a, 0x00);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x50_bvc_relative_branches_backward_correctly() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0x7E, /*ADC*/ 0x69, 0x01, /*BVC-4*/ 0x50, 0x84, 0x00,
    ]);

    assert_eq!(cpu.register_a, 0x80);
    assert_flags(&cpu, vec![Flags::Negative, Flags::Overflow]);
}

#[test]
fn test_0x50_bvc_relative_ignores_branching_when_offset_is_zero() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0x7F, /*ADC*/ 0x69, 0x01, /*BVC+-0*/ 0x50, 0x00, 0x00,
    ]);

    assert_eq!(cpu.register_a, 0x80);
    assert_flags(&cpu, vec![Flags::Negative, Flags::Overflow]);
}

#[test]
fn test_0x50_bvc_relative_ignores_branching_when_condition_is_not_met() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0x7F, /*ADC*/ 0x69, 0x01, /*BVC-4*/ 0x50, 0x84,
    ]);

    assert_eq!(cpu.register_a, 0x80);
    assert_flags(&cpu, vec![Flags::Negative, Flags::Overflow]);
}
