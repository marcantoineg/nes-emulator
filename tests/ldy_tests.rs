use std::vec;
use nes_emulator::cpu::CPU;

#[test]
fn test_0xa0_ldy_immediate_load_data() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![0xa0, 0x05, 0x00]);

    assert_eq!(cpu.register_y, 0x05);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xa0_ldy_immediate_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run_without_reset(vec![0xa0, 0x00, 0x00]);
    assert_eq!(cpu.zero_flag(), true);
}

#[test]
fn test_0xa0_ldy_immediate_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run_without_reset(vec![0xa0, 0b1000_0000, 0x00]);
    assert_eq!(cpu.negative_flag(), true);
}

#[test]
fn test_0xa4_ldy_zero_page_load_data() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x05, 0x01);

    cpu.load_and_run_without_reset(vec![0xa4, 0x05, 0x00]);

    assert_eq!(cpu.register_y, 0x01);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xa4_ldy_zero_page_zero_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![0xa4, 0x05, 0x00]);

    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.zero_flag(), true);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xa4_ldy_zero_page_negative_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x05, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xa4, 0x05, 0x00]);

    assert_eq!(cpu.register_y, 0b1000_0000);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), true);
}

#[test]
fn test_0xb4_ldy_zero_page_x_load_data() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x02, 2);

    cpu.load_and_run_without_reset(vec![0xb4, 0x01, 0x00]);

    assert_eq!(cpu.register_y, 2);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xb4_ldy_zero_page_x_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x02, 0);

    cpu.load_and_run_without_reset(vec![0xb4, 0x01, 0x00]);

    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.zero_flag(), true);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xb4_ldy_zero_page_x_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x02, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xb4, 0x01, 0x00]);

    assert_eq!(cpu.register_y, 0b1000_0000);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), true);
}

#[test]
fn test_0xac_ldy_absolute_load_data() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x00FF, 0x05);

    cpu.load_and_run_without_reset(vec![0xac, 0x00FF, 0x00]);

    assert_eq!(cpu.register_y, 0x05);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xac_ldy_absolute_zero_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x00FF, 0);

    cpu.load_and_run_without_reset(vec![0xac, 0x00FF, 0x00]);

    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.zero_flag(), true);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xac_ldy_absolute_negative_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x00FF, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xac, 0x00FF, 0x00]);

    assert_eq!(cpu.register_y, 0b1000_0000);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), true);
}

#[test]
fn test_0xbc_ldy_absolute_x_load_data() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x00FF, 0x05);

    cpu.load_and_run_without_reset(vec![0xbc, 0x00FE, 0x00]);

    assert_eq!(cpu.register_y, 0x05);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xbc_ldy_absolute_x_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x00FF, 0);

    cpu.load_and_run_without_reset(vec![0xbc, 0x00FE, 0x00]);

    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.zero_flag(), true);
    assert_eq!(cpu.negative_flag(), false);
}

#[test]
fn test_0xbc_ldy_absolute_x_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x00FF, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xbc, 0x00FE, 0x00]);

    assert_eq!(cpu.register_y, 0b1000_0000);
    assert_eq!(cpu.zero_flag(), false);
    assert_eq!(cpu.negative_flag(), true);
}
