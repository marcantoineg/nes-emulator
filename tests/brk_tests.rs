use nes_emulator::cpu::{Flags, CPU};

mod common;

#[test]
fn test_0x00_brk_implied_pushes_pc_and_status_to_stack() {
    let mut cpu = CPU::new();
    let initial_sp = cpu.stack_pointer;

    cpu.load_and_run_without_reset(vec![0x00, 0xEA]);

    // The stack should have 0x8002:
    assert_eq!(cpu.memory.read(0x01FF), 0x80); // high byte
    assert_eq!(cpu.memory.read(0x01FE), 0x02); // low byte = 0x02, high = 0x80, so 0x8002

    // Status should be at 0x01FD
    let pushed_status = cpu.memory.read(0x01FD);
    assert_eq!(pushed_status & Flags::Break.bits(), Flags::Break.bits());

    // Stack pointer should have moved down by 3
    assert_eq!(cpu.stack_pointer, initial_sp - 3);
}

#[test]
fn test_0x00_brk_sets_break_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![0x00, 0xEA]);

    assert!(cpu.status.contains(Flags::Break));
}

#[test]
fn test_0x00_brk_sets_interrupt_disable_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![0x00, 0xEA]);

    assert!(cpu.status.contains(Flags::InteruptDisable));
}

#[test]
fn test_0x00_brk_preserves_other_flags() {
    let mut cpu = CPU::new();

    // Set some other flags before BRK
    cpu.status.insert(Flags::Carry);
    cpu.status.insert(Flags::Zero);
    cpu.status.insert(Flags::Negative);

    cpu.load_and_run_without_reset(vec![0x00, 0xEA]);

    // All flags should still be set
    assert!(cpu.status.contains(Flags::Carry));
    assert!(cpu.status.contains(Flags::Zero));
    assert!(cpu.status.contains(Flags::Negative));
    assert!(cpu.status.contains(Flags::Break));
    assert!(cpu.status.contains(Flags::InteruptDisable));
}

#[test]
fn test_0x00_brk_pushed_status_contains_break_and_interrupt_flags() {
    let mut cpu = CPU::new();

    // Set other flags before BRK
    cpu.status.insert(Flags::Carry);
    cpu.status.insert(Flags::Zero);

    cpu.load_and_run_without_reset(vec![0x00, 0xEA]);

    // Get the pushed status
    let pushed_status = cpu.memory.read(0x01FD);

    // Verify all relevant flags were pushed correctly
    assert_eq!(pushed_status & Flags::Carry.bits(), Flags::Carry.bits());
    assert_eq!(pushed_status & Flags::Zero.bits(), Flags::Zero.bits());
    assert_eq!(pushed_status & Flags::Break.bits(), Flags::Break.bits());
    assert_eq!(
        pushed_status & Flags::InteruptDisable.bits(),
        Flags::InteruptDisable.bits()
    );
}

#[test]
fn test_0x00_brk_stack_operations_correct_order() {
    let mut cpu = CPU::new();

    // Get initial SP before any pushes
    let initial_sp = cpu.stack_pointer;

    cpu.load_and_run_without_reset(vec![0x00, 0xEA]);

    // BRK pushes: 2 bytes for PC + 1 byte for status = 3 bytes
    // So final SP should be initial_sp - 3
    assert_eq!(cpu.stack_pointer, initial_sp - 3);

    // Verify the return address was pushed (0x8002)
    assert_eq!(cpu.memory.read(0x01FF), 0x80); // high byte of return address
    assert_eq!(cpu.memory.read(0x01FE), 0x02); // low byte of return address

    // Verify status was pushed with Break and Interrupt Disable flags
    let pushed_status = cpu.memory.read(0x01FD);
    assert_eq!(pushed_status & Flags::Break.bits(), Flags::Break.bits());
    assert_eq!(
        pushed_status & Flags::InteruptDisable.bits(),
        Flags::InteruptDisable.bits()
    );
}

#[test]
fn test_0x00_brk_stops_cpu_execution() {
    let mut cpu = CPU::new();

    // Program with BRK followed by many NOPs
    let program = vec![
        0x00, // BRK at 0x8000
        0xEA, // NOP at 0x8001 (should not be executed)
        0xEA, // NOP at 0x8002 (should not be executed)
        0xEA, // NOP at 0x8003 (should not be executed)
    ];

    cpu.load_and_run_without_reset(program);

    // After BRK, PC should be at 0x8001 (incremented before BRK executes)
    // Then BRK returns, stopping the CPU
    // So CPU shouldn't execute the following NOPs
    assert_eq!(cpu.program_counter, 0x8001);
}

#[test]
fn test_0x00_brk_with_prior_flags_all_preserved() {
    let mut cpu = CPU::new();

    // Set ALL individual flags except Break
    cpu.status.insert(Flags::Carry);
    cpu.status.insert(Flags::Zero);
    cpu.status.insert(Flags::InteruptDisable);
    cpu.status.insert(Flags::Decimal);
    cpu.status.insert(Flags::Overflow);
    cpu.status.insert(Flags::Negative);

    cpu.load_and_run_without_reset(vec![0x00, 0xEA]);

    // Check that all originally set flags are still set
    assert!(cpu.status.contains(Flags::Carry));
    assert!(cpu.status.contains(Flags::Zero));
    assert!(cpu.status.contains(Flags::Overflow));
    assert!(cpu.status.contains(Flags::Negative));
    assert!(cpu.status.contains(Flags::Break));
    assert!(cpu.status.contains(Flags::InteruptDisable));
    assert!(cpu.status.contains(Flags::Decimal));
}
