use std::vec;
use nes_emulator::cpu::CPU;

#[test]
fn test_0x69_adc_immediate_adds_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;
    
    cpu.load_and_run_without_reset(vec![0x69, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0x02);
    assert_eq!(cpu.status.bits(), 0b0011_0000);
}

#[test]
fn test_0x69_adc_immediate_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x00;

    cpu.load_and_run_without_reset(vec![0x69, 0x00, 0x00]);
    
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.status.bits(), 0b0011_0010);
}

#[test]
fn test_0x69_adc_immediate_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0;

    cpu.load_and_run_without_reset(vec![0x69, 0b1000_0001, 0x00]);
    
    assert_eq!(cpu.register_a, 0b1000_0001);
    assert_eq!(cpu.status.bits(), 0b1011_0000);
}

#[test]
fn test_0x69_adc_immediate_carry_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0xFF;

    cpu.load_and_run_without_reset(vec![0x69, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.status.bits(), 0b0011_0011);
}

#[test]
fn test_0x69_adc_immediate_overflow() {
    let mut cpu = CPU::new();
    cpu.register_a = 0xFF;

    cpu.load_and_run_without_reset(vec![0x69, 0xFF, 0x00]);

    assert_eq!(cpu.register_a, 0xFE);
    assert_eq!(cpu.status.bits(), 0b1011_0001);
}

#[test]
fn test_0x65_adc_zero_page_add_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 3;
    cpu.memory.write(200, 0x05);

    cpu.load_and_run_without_reset(vec![0x65, 0x20, 0x00]);

    assert_eq!(cpu.register_a, 8);
    assert_eq!(cpu.status.bits(), 0b0000_0000);
    
}
