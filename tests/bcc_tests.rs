use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use common::{assert_no_flags, assert_flags};

#[test]
fn test_0x90_bcc_immediate_branches_forward_correctly() {
    let mut cpu = CPU::new();
    
    cpu.load_and_run_without_reset(vec![
        /*BCC+2*/ 0x90, 0x02,
        /*LDA*/ 0xA9, 0x02,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x00);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x90_bcc_immediate_ignore_branching_correctly() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Carry);
    
    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0xFE,
        /*ADC*/ 0x69, 0x01,
        /*BCC-1*/ 0x90, 0x81,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x00);
    assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
}

#[test]
fn test_0x90_bcc_immediate_ignore_branching_when_offset_is_zero() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0x01,
        /*ADC*/ 0x69, 0x01,
        /*BCC+-0*/ 0x90, 0x00,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x02);
    assert_no_flags(&cpu);
}
