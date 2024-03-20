use nes_emulator::memory::Memory;

#[test]
fn test_write_u16() {
    let mut mem = Memory::new();
    mem.write_u16(0, 0x01);

    assert_eq!(mem.memory[0], 0x1);
    assert_eq!(mem.memory[1], 0x0);
}