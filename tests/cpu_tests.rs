use std::vec;

use nes_emulator::cpu::{Flags, CPU};

#[test]
fn test_load_and_run_reset_registeries() {
    let mut cpu = CPU::new();
    cpu.register_a = 1;
    cpu.register_x = 2;
    cpu.register_y = 3;
    cpu.status = Flags::all();

    cpu.load_and_run(vec![0x00]);

    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.status.bits(), 0b0011_0000);
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run_without_reset(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}
