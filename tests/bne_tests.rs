use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use common::{assert_no_flags, assert_flags};

#[test]
fn test_0xd0_bne_relative_branches_forward_correctly() {
    let mut cpu = CPU::new();
    
    cpu.load_and_run_without_reset(vec![
        /*BNE+2*/ 0xD0, 0x02,
        /*LDA*/ 0xA9, 0x02,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x00);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xd0_bne_relative_branches_backward_correctly() {
    let mut cpu = CPU::new();
    
    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0xFE,
        /*ADC*/ 0x69, 0x01,
        /*BNE-4*/ 0xD0, 0x84,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x00);
    assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
}

#[test]
fn test_0xd0_bne_relative_ignores_branching_when_offset_is_zero() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0x00,
        /*BNE+-0*/ 0xD0, 0x00,
        /*ADC*/ 0x69, 0x01,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x01);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xd0_bne_relative_ignores_branching_when_condition_is_not_met() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0xFF,
        /*ADC*/ 0x69, 0x01,
        /*BNE-4*/ 0xD0, 0x84,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x00);
    assert_flags(&cpu, vec![Flags::Zero, Flags::Carry]);
}
