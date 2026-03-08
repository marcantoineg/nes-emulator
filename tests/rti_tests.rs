mod common;

use common::{assert_flags, assert_flags_without_break, push_to_stack};
use nes_emulator::cpu::{Flags, CPU};

#[test]
fn test_0x40_rti_implied_pulls_from_stack_correctly() {
    let mut cpu = CPU::new();
    // PC
    push_to_stack(&mut cpu, 0x81);
    push_to_stack(&mut cpu, 0x81);
    // cpu status
    push_to_stack(&mut cpu, 0b1111_1111);

    assert_flags_without_break(&cpu, vec![]);
    assert_eq!(cpu.stack_pointer, 0xFC);
    cpu.load_and_run_without_reset(vec![0x40]);

    // All flags should be set: RTI restored the status from stack (0xFF), then BRK at 0x8181 ran
    assert_flags(
        &cpu,
        vec![
            Flags::Carry,
            Flags::Zero,
            Flags::InteruptDisable,
            Flags::Decimal,
            Flags::Overflow,
            Flags::Negative,
        ],
    );
    assert_eq!(cpu.program_counter, (0x8181 + 1)); // add one here since it tries to read op at 0x8181 that has 0x00 (BRK) then adds 1 after running BRK before stopping.
    assert_eq!(cpu.stack_pointer, 0xFC);
}
