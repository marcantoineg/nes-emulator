use std::vec;

use nes_emulator::cpu::CPU;

#[test]
fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = CPU::new();

    cpu.interpret(vec![0xa9, 0x05, 0x00]);

    assert_eq!(cpu.register_a, 0x05);
    assert_eq!(cpu.zero_flag_set(), false);
    assert_eq!(cpu.negative_flag_set(), false);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0x00, 0x00]);
    assert_eq!(cpu.zero_flag_set(), true);
}

#[test]
fn test_0xa9_lda_negative_flag() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xA9, 0b1000_0000, 0x00]);
    assert_eq!(cpu.negative_flag_set(), true);
}

#[test]
fn test_0xaa_tax_implied_copy_data() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;

    cpu.interpret(vec![0xAA, 0x00]);

    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.register_x, 0x01);
    assert_eq!(cpu.zero_flag_set(), false);
    assert_eq!(cpu.negative_flag_set(), false);
}

#[test]
fn test_0xaa_tax_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x00;

    cpu.interpret(vec![0xAA, 0x00]);

    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.zero_flag_set(), true);
}

#[test]
fn test_0xaa_tax_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;

    cpu.interpret(vec![0xAA, 0x00]);
    
    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_eq!(cpu.register_x, 0b1000_0000);
    assert_eq!(cpu.negative_flag_set(), true);
}