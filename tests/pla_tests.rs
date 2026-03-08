use nes_emulator::cpu::{Flags, CPU};

mod common;
use common::{assert_flags, only_break_flag_set};

#[test]
fn test_0x68_pla_implied_pulls_correctly_from_stack() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x01FF, 0b0111_0101);
    cpu.stack_pointer -= 1;

    cpu.load_and_run_without_reset(vec![0x68, 0x00]);

    only_break_flag_set(&cpu);
    assert_eq!(cpu.register_a, 0b0111_0101);

    assert_eq!(cpu.stack_pointer, 0xFC);
    assert_ne!(cpu.memory.read(0x01FF), 0b0111_0101);
}

#[test]
fn test_0x68_pla_implied_sets_zero_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x01FF, 0x00);
    cpu.stack_pointer -= 1;

    cpu.load_and_run_without_reset(vec![0x68, 0x00]);

    assert_flags(&cpu, vec![Flags::Zero]);
    assert_eq!(cpu.register_a, 0x00);

    assert_eq!(cpu.stack_pointer, 0xFC);
    assert_ne!(cpu.memory.read(0x01FF), 0x00);
}

#[test]
fn test_0x68_pla_implied_sets_negative_flag_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x01FF, 0x81); // 0x81 has bit 7 set, so it's negative
    cpu.stack_pointer -= 1;

    cpu.load_and_run_without_reset(vec![0x68, 0x00]);

    assert_flags(&cpu, vec![Flags::Negative]);
    assert_eq!(cpu.register_a, 0x81);

    assert_eq!(cpu.stack_pointer, 0xFC);
    assert_ne!(cpu.memory.read(0x01FF), 0x81);
    assert_eq!(cpu.stack_pointer, 0xFC);
}
