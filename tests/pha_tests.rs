use nes_emulator::cpu::CPU;

mod common;
use common::assert_no_flags;

#[test]
fn test_0x48_pha_implied_pushed_to_stack_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x80;

    cpu.load_and_run_without_reset(vec![0x48, 0x00]);

    assert_eq!(cpu.stack_pointer, 0xFE);
    assert_eq!(cpu.memory.read(0x01FF), 0x80);
    assert_no_flags(&cpu);
}
