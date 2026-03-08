use nes_emulator::cpu::{Flags, CPU};
use std::vec;

mod common;
use common::{assert_flags, only_break_flag_set};

#[test]
fn test_0x30_bmi_relative_branches_forward_correctly() {
    let mut cpu = CPU::new();

    cpu.load_and_run_n_without_reset(
        vec![
            /*LDA*/ 0xA9, 0x81, /*BMI+2*/ 0x30, 0x02, /*LDA*/ 0xA9, 0x05, 0x00,
        ],
        2,
    );

    assert_eq!(cpu.register_a, 0x81);
    assert_flags(&cpu, vec![Flags::Negative]);
}

#[test]
fn test_0x30_bmi_relative_branches_backward_correctly() {
    let mut cpu = CPU::new();

    cpu.load_and_run_n_without_reset(
        vec![
            /*LDA*/ 0xA9, 0xFE, /*ADC*/ 0x69, 0x01, /*BMI-4*/ 0x30, 0x84,
        ],
        5,
    );

    assert_eq!(cpu.register_a, 0x00);
    assert_flags(&cpu, vec![Flags::Zero, Flags::Carry]);
}

#[test]
fn test_0x30_bmi_relative_ignores_branching_when_offset_is_zero() {
    let mut cpu = CPU::new();

    cpu.load_and_run_n_without_reset(
        vec![
            /*LDA*/ 0xA9, 0xFE, /*BMI+-0*/ 0x30, 0x00, /*ADC*/ 0x69, 0x01, 0x00,
        ],
        3,
    );

    assert_eq!(cpu.register_a, 0xFF);
    assert_flags(&cpu, vec![Flags::Negative]);
}

#[test]
fn test_0x30_bmi_relative_ignores_branching_when_condition_is_not_met() {
    let mut cpu = CPU::new();

    cpu.load_and_run_n_without_reset(
        vec![
            /*LDA*/ 0xA9, 0x01, /*ADC*/ 0x69, 0x01, /*BMI-4*/ 0x30, 0x84,
        ],
        3,
    );

    assert_eq!(cpu.register_a, 0x02);
    assert_flags(&cpu, vec![]);
}
