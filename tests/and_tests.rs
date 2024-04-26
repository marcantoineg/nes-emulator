use nes_emulator::cpu::{CPU, Flags};

mod common;
use common::{assert_flag, assert_no_flags, assert_flags};

#[test]
fn test_0x29_and_immediate_calculates_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_1111;

    cpu.load_and_run_without_reset(vec![0x29, 0b0111_0000, 0x00]);

    assert_eq!(cpu.register_a, 0b0111_0000);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x29_and_immediate_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1111_1111;

    cpu.load_and_run_without_reset(vec![0x29, 0b0000_0000, 0x00]);

    assert_eq!(cpu.register_a, 0);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x29_and_immediate_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;

    cpu.load_and_run_without_reset(vec![0x29, 0b1000_0000, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x25_and_zero_page_calculates_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.memory.write(0x0024, 0b0111_0000);

    cpu.load_and_run_without_reset(vec![0x25, 0x24, 0x00]);


    assert_eq!(cpu.register_a, 0b0111_0000);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x25_and_zero_page_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.memory.write(0x0024, 0b0000_0000);

    cpu.load_and_run_without_reset(vec![0x25, 0x24, 0x00]);


    assert_eq!(cpu.register_a, 0b0000_0000);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x25_and_zero_page_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;
    cpu.memory.write(0x0024, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0x25, 0x24, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x35_and_zero_page_x_calculates_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.register_x = 0x01;
    cpu.memory.write(0x0025, 0b0111_0000);

    cpu.load_and_run_without_reset(vec![0x35, 0x24, 0x00]);

    assert_eq!(cpu.register_a, 0b0111_0000);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x35_and_zero_page_x_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.register_x = 0x01;
    cpu.memory.write(0x0025, 0b0000_0000);

    cpu.load_and_run_without_reset(vec![0x35, 0x24, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0000);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x35_and_zero_page_x_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;
    cpu.register_x = 0x01;
    cpu.memory.write(0x0025, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0x35, 0x24, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x2d_and_absolute_calculates_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.memory.write(0x1010, 0b0111_0000);

    cpu.load_and_run_without_reset(vec![0x2D, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0b0111_0000);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x2d_and_absolute_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.memory.write(0x1010, 0b0000_0000);

    cpu.load_and_run_without_reset(vec![0x2D, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0000);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x2d_and_absolute_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;
    cpu.memory.write(0x1010, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0x2D, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x3d_and_absolute_x_calculates_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.register_x = 0x01;
    cpu.memory.write(0x1011, 0b0111_0000);

    cpu.load_and_run_without_reset(vec![0x3D, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0b0111_0000);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x3d_and_absolute_x_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.register_x = 0x01;
    cpu.memory.write(0x1011, 0b0000_0000);

    cpu.load_and_run_without_reset(vec![0x3D, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0000);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x3d_and_absolute_x_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;
    cpu.register_x = 0x01;
    cpu.memory.write(0x1011, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0x3D, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x39_and_absolute_y_calculates_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.register_y = 0x01;
    cpu.memory.write(0x1011, 0b0111_0000);

    cpu.load_and_run_without_reset(vec![0x39, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0b0111_0000);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x39_and_absolute_y_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.register_y = 0x01;
    cpu.memory.write(0x1011, 0b0000_0000);

    cpu.load_and_run_without_reset(vec![0x39, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0000);
    assert_flags(&cpu, vec![Flags::Zero])
}

#[test]
fn test_0x39_and_absolute_y_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;
    cpu.register_y = 0x01;
    cpu.memory.write(0x1011, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0x39, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x21_and_indirect_x_calculates_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.register_x = 0x01;
    cpu.memory.write_u16(0x02, 0x1010);
    cpu.memory.write_u16(0x1010, 0b0111_0000);

    cpu.load_and_run_without_reset(vec![0x21, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b0111_0000);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x21_and_indirect_x_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.register_x = 0x01;
    cpu.memory.write_u16(0x02, 0x1010);
    cpu.memory.write_u16(0x1010, 0b0000_0000);

    cpu.load_and_run_without_reset(vec![0x21, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0000);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x21_and_indirect_x_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;
    cpu.register_x = 0x01;
    cpu.memory.write_u16(0x02, 0x1010);
    cpu.memory.write_u16(0x1010, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0x21, 0x01, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}

#[test]
fn test_0x31_and_indirect_y_calculates_correctly() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.register_y = 0x01;
    cpu.memory.write_u16(0x02, 0x1010);
    cpu.memory.write(0x1011, 0b0111_0000);

    cpu.load_and_run_without_reset(vec![0x31, 0x02, 0x00]);

    assert_eq!(cpu.register_a, 0b0111_0000);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x31_and_indirect_y_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b0111_0000;
    cpu.register_y = 0x01;
    cpu.memory.write_u16(0x02, 0x1010);
    cpu.memory.write(0x1011, 0b0000_0000);

    cpu.load_and_run_without_reset(vec![0x31, 0x02, 0x00]);

    assert_eq!(cpu.register_a, 0b0000_0000);
    assert_flag(&cpu, Flags::Zero);
}

#[test]
fn test_0x31_and_indirect_y_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1111_0000;
    cpu.register_y = 0x01;
    cpu.memory.write_u16(0x02, 0x1010);
    cpu.memory.write(0x1011, 0b1000_0000);

    cpu.load_and_run_without_reset(vec![0x31, 0x02, 0x00]);

    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_flag(&cpu, Flags::Negative);
}
