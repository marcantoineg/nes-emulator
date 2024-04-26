use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use common::{assert_flag, assert_no_flags, assert_flags};

#[test]
fn test_0xb0_bcs_relative_branches_forward_correctly() {
    let mut cpu = CPU::new();
    cpu.status.insert(Flags::Carry);
    
    cpu.load_and_run_without_reset(vec![
        /*BCS+2*/ 0xB0, 0x02,
        /*LDA*/ 0xA9, 0x02,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x00);
    assert_flag(&cpu, Flags::Carry);
}

#[test]
fn test_0xb0_bcs_relative_branches_backward_correctly() {
    let mut cpu = CPU::new();
    
    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0xFF,
        /*ADC*/ 0x69, 0x01,
        /*BCS-4*/ 0xB0, 0x84,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x02); // 0x02 here because of adc+1 with carry (so +2)
    assert_no_flags(&cpu);
}

#[test]
fn test_0xb0_bcs_relative_ignores_branching_when_offset_is_zero() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0xFF,
        /*ADC*/ 0x69, 0x01,
        /*BCS+-0*/ 0xB0, 0x00,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x00);
    assert_flags(&cpu, vec![Flags::Carry, Flags::Zero]);
}

#[test]
fn test_0xb0_bcs_relative_ignores_branching_when_condition_is_not_met() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![
        /*LDA*/ 0xA9, 0x01,
        /*ADC*/ 0x69, 0x01,
        /*BCS-4*/ 0xB0, 0x84,
        0x00
    ]);

    assert_eq!(cpu.register_a, 0x02);
    assert_no_flags(&cpu);
}
