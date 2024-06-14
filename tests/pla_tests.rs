use nes_emulator::cpu::{Flags, CPU};

mod common;
use common::{assert_flag, assert_no_flags};

#[test]
fn test_0x68_pla_implied_pulls_correctly_from_stack() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x01FF, 0b0111_0101);
    cpu.stack_pointer -= 1;

    cpu.load_and_run_without_reset(vec![0x68, 0x00]);

    assert_no_flags(&cpu);
    assert_eq!(cpu.register_a, 0b0111_0101);

    assert_eq!(cpu.stack_pointer, 0xFF);
    assert_eq!(cpu.memory.read(0x01FF), 0x00);
}

#[test]
fn test_0x68_pla_implied_sets_zero_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x01FF, 0x00);
    cpu.stack_pointer -= 1;

    cpu.load_and_run_without_reset(vec![0x68, 0x00]);

    assert_flag(&cpu, Flags::Zero);
    assert_eq!(cpu.register_a, 0x00);

    assert_eq!(cpu.stack_pointer, 0xFF);
    assert_eq!(cpu.memory.read(0x01FF), 0x00);
}

#[test]
fn test_0x68_pla_implied_sets_negative_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x01FF, 0x80);
    cpu.stack_pointer -= 1;

    cpu.load_and_run_without_reset(vec![0x68, 0x00]);

    assert_flag(&cpu, Flags::Negative);
    assert_eq!(cpu.register_a, 0x80);

    assert_eq!(cpu.stack_pointer, 0xFF);
    assert_eq!(cpu.memory.read(0x01FF), 0x00);
}
