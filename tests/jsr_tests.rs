use nes_emulator::cpu::CPU;

mod common;
use crate::common::only_break_flag_set;

#[test]
fn test_0x20_jsr_absolute_jumps_to_subroutine_correctly() {
    let mut cpu = CPU::new();

    cpu.load_and_run_n_without_reset(
        vec![
            /*JSR*/ 0x20, 0x05, 0x80, /*LDX*/ 0xA2, 0x01, /*LDA*/ 0xA9, 0x02, 0x00,
        ],
        2,
    );

    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.register_a, 0x02);
    // assert address 0x8002 was push on stack (little-endian)
    assert_eq!(cpu.memory.read(0x01FE), 0x02);
    assert_eq!(cpu.memory.read(0x01FF), 0x80);

    assert_eq!(cpu.program_counter, 0x8007);
    only_break_flag_set(&cpu);
}
