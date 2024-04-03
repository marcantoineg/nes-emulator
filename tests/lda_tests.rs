use std::vec;
use nes_emulator::cpu::{CPU, Flags};

mod common;
use common::{assert_flags, assert_no_flags};

#[test]
fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![0xa9, 0x05, 0x00]);

    assert_eq!(cpu.register_a, 0x05);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xa9_lda_immediate_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run_without_reset(vec![0xa9, 0x00, 0x00]);
    assert_flags(&cpu, vec![Flags::Zero]);
}

#[test]
fn test_0xa9_lda_immediate_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run_without_reset(vec![0xA9, 0b1000_0000, 0x00]);
    assert_flags(&cpu, vec![Flags::Negative]);
}

#[test]
fn test_0xa5_lda_zero_page_load_data() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x05, 0x01);

    cpu.load_and_run_without_reset(vec![0xA5, 0x05, 0x00]);

    assert_eq!(cpu.register_a, 0x01);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xa5_lda_zero_page_zero_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run_without_reset(vec![0xA5, 0x05, 0x00]);

    assert_eq!(cpu.register_a, 0);
    assert_flags(&cpu, vec![Flags::Zero]);
}

#[test]
fn test_0xa5_lda_zero_page_negative_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x05, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xA5, 0x05, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flags(&cpu, vec![Flags::Negative]);
}

#[test]
fn test_0xb5_lda_zero_page_x_load_data() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x02, 2);

    cpu.load_and_run_without_reset(vec![0xB5, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 2);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xb5_lda_zero_page_x_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x02, 0);

    cpu.load_and_run_without_reset(vec![0xB5, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0);
    assert_flags(&cpu, vec![Flags::Zero]);
}

#[test]
fn test_0xb5_lda_zero_page_x_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x02, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xB5, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flags(&cpu, vec![Flags::Negative]);
}

#[test]
fn test_0xad_lda_absolute_load_data() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x1110, 0x05);

    cpu.load_and_run_without_reset(vec![0xAD, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0x05);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xad_lda_absolute_zero_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x1110, 0);

    cpu.load_and_run_without_reset(vec![0xAD, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0);
    assert_flags(&cpu, vec![Flags::Zero]);
}

#[test]
fn test_0xad_lda_absolute_negative_flag() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x1110, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xAD, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flags(&cpu, vec![Flags::Negative]);
}

#[test]
fn test_0xbd_lda_absolute_x_load_data() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x1111, 0x05);

    cpu.load_and_run_without_reset(vec![0xBD, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0x05);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xbd_lda_absolute_x_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x1111, 0);

    cpu.load_and_run_without_reset(vec![0xBD, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0);
    assert_flags(&cpu, vec![Flags::Zero]);
}

#[test]
fn test_0xbd_lda_absolute_x_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write(0x1111, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xBD, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flags(&cpu, vec![Flags::Negative]);
}

#[test]
fn test_0xb9_lda_absolute_y_load_data() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;
    cpu.memory.write(0x1111, 0x05);

    cpu.load_and_run_without_reset(vec![0xB9, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0x05);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xb9_lda_absolute_y_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;
    cpu.memory.write(0x1111, 0);

    cpu.load_and_run_without_reset(vec![0xB9, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0);
    assert_flags(&cpu, vec![Flags::Zero]);
}

#[test]
fn test_0xb9_lda_absolute_y_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;
    cpu.memory.write(0x1111, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xB9, 0x10, 0x11, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flags(&cpu, vec![Flags::Negative]);
}

#[test]
fn test_0xa1_lda_indirect_x_load_data() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;
    cpu.memory.write_u16(0x02, 0x1000);
    cpu.memory.write(0x1000, 0x05);

    cpu.load_and_run_without_reset(vec![0xA1, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0x05);
    assert_no_flags(&cpu);
}

#[test]
fn test_0xa1_lda_indirect_x_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write_u16(0x02, 0x1000);
    cpu.memory.write(0x1000, 0);

    cpu.load_and_run_without_reset(vec![0xA1, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0);
    assert_flags(&cpu, vec![Flags::Zero]);
}

#[test]
fn test_0xa1_lda_indirect_x_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 1;
    cpu.memory.write_u16(0x02, 0x1000);
    cpu.memory.write(0x1000, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0xA1, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flags(&cpu, vec![Flags::Negative]);
}
