use std::vec;
use nes_emulator::cpu::CPU;

#[test]
fn test_0xa2_ldx_immediate_load_data() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![0xa2, 0x05, 0x00]);

    assert_eq!(cpu.register_x, 0x05);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xa2_ldx_immediate_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run_without_reset(vec![0xa2, 0x00, 0x00]);
    assert_eq!(cpu.zero_flag(), true);
}

#[test]
fn test_0xa2_ldx_immediate_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run_without_reset(vec![0xa2, 0b1000_0000, 0x00]);
    assert_eq!(cpu.negative_flag(), true);
}

#[test]
fn test_0xa6_ldx_zero_page_load_data() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x05, 0x01);

    cpu.load_and_run_without_reset(vec![0xa6, 0x05, 0x00]);

    assert_eq!(cpu.register_x, 0x01);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xa6_ldx_zero_page_zero_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![0xa6, 0x05, 0x00]);

    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.zero_flag(), true);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xa6_ldx_zero_page_negative_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x05, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xa6, 0x05, 0x00]);

    assert_eq!(cpu.register_x, 0b1000_0000);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), true);
}

#[test]
fn test_0xb6_ldx_zero_page_y_load_data() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;
    cpu.memory.write(0x02, 2);

    cpu.load_and_run_without_reset(vec![0xB6, 0x01, 0x00]);

    assert_eq!(cpu.register_x, 2);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xb6_ldx_zero_page_y_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;
    cpu.memory.write(0x02, 0);

    cpu.load_and_run_without_reset(vec![0xB6, 0x01, 0x00]);

    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.zero_flag(), true);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xb6_ldx_zero_page_y_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;
    cpu.memory.write(0x02, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xB6, 0x01, 0x00]);

    assert_eq!(cpu.register_x, 0b1000_0000);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), true);
}

#[test]
fn test_0xae_ldx_absolute_load_data() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x00FF, 0x05);

    cpu.load_and_run_without_reset(vec![0xae, 0x00FF, 0x00]);

    assert_eq!(cpu.register_x, 0x05);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xae_ldx_absolute_zero_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x00FF, 0);

    cpu.load_and_run_without_reset(vec![0xae, 0x00FF, 0x00]);

    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.zero_flag(), true);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xae_ldx_absolute_negative_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x00FF, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xae, 0x00FF, 0x00]);

    assert_eq!(cpu.register_x, 0b1000_0000);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), true);
}

#[test]
fn test_0xbe_ldx_absolute_y_load_data() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;
    cpu.memory.write(0x00FF, 0x05);

    cpu.load_and_run_without_reset(vec![0xbe, 0x00FE, 0x00]);

    assert_eq!(cpu.register_x, 0x05);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xbe_ldx_absolute_y_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;
    cpu.memory.write(0x00FF, 0);

    cpu.load_and_run_without_reset(vec![0xbe, 0x00FE, 0x00]);

    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.zero_flag(), true);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xbe_ldx_absolute_y_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;
    cpu.memory.write(0x00FF, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xbe, 0x00FE, 0x00]);

    assert_eq!(cpu.register_x, 0b1000_0000);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), true);
}
