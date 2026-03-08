use nes_emulator::cpu::CPU;

mod common;
use common::only_break_flag_set;

#[test]
fn test_0x48_pha_implied_pushed_to_stack_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x80;

    cpu.load_and_run_n_without_reset(vec![0x48], 1);

    // BRK pushes return address (2 bytes) + status (1 byte) + PHA (1 byte) = 4 bytes (0xFF -> 0xFB)
    assert_eq!(cpu.stack_pointer, 0xFE);
    assert_eq!(cpu.memory.read(0x01FF), 0x80);
    only_break_flag_set(&cpu);
}
