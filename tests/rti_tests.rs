mod common;

use common::{assert_no_flags, push_to_stack};
use nes_emulator::cpu::CPU;

#[test]
fn test_0x60_rti_implied_pulls_from_stack_correctly() {
    let mut cpu = CPU::new();
    // PC
    push_to_stack(&mut cpu, 0x81);
    push_to_stack(&mut cpu, 0x81);
    // cpu status
    push_to_stack(&mut cpu, 0b1111_1111);

    assert_no_flags(&cpu);
    assert_eq!(cpu.stack_pointer, 0xFC);
    cpu.load_and_run_without_reset(vec![0x60]);

    assert_eq!(cpu.status.bits(), 0b1111_1111);
    assert_eq!(cpu.program_counter, (0x8181 + 1)); // add one here since it tries to read op at 0x8181 that has 0x00 (BRK) then adds 1 after running BRK before stopping.
    assert_eq!(cpu.stack_pointer, 0xFF);
}
