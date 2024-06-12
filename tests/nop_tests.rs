use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use common::assert_flags;

#[test]
fn test_0xea_nop_implied_does_nothing_and_updates_pc() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;
    cpu.register_x = 0x02;
    cpu.register_y = 0x03;
    cpu.status.insert(Flags::Zero | Flags::Overflow | Flags::Carry);
    
    cpu.load_and_run_without_reset(vec![0xEA]);

    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.register_x, 0x02);
    assert_eq!(cpu.register_y, 0x03);
    assert_flags(&cpu, vec![Flags::Zero, Flags::Overflow, Flags::Carry]);
    assert_eq!(cpu.program_counter, 0x8002);
}