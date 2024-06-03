use nes_emulator::cpu::CPU;

mod common;
use crate::common::assert_no_flags;

#[test]
fn test_0x4c_jmp_immediate_jumps_correctly() {
    let mut cpu = CPU::new();
    
    cpu.load_and_run_without_reset(vec![
        /*JMP*/ 0x4C, 0x05, 0x80,
        /*LDX*/ 0xA2, 0x01,
        /*LDA*/ 0xA9, 0x02,
        0x00
    ]);

    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.register_a, 0x02);
    assert_no_flags(&cpu);
}

#[test]
fn test_0x6c_jmp_immediate_jumps_correctly() {
    let mut cpu = CPU::new();
    cpu.memory.write_u16(0x1010, 0x8005);
    
    cpu.load_and_run_without_reset(vec![
        /*JMP*/ 0x6C, 0x10, 0x10,
        /*LDX*/ 0xA2, 0x01,
        /*LDA*/ 0xA9, 0x02,
        0x00
    ]);

    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.register_a, 0x02);
    assert_no_flags(&cpu);
}
