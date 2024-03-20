use nes_emulator::memory::Memory;

#[test]
fn test_write_u16() {
    let mut mem = Memory::new();
    mem.write_u16(0, 0x01);

    assert_eq!(mem.memory[0], 0x1);
    assert_eq!(mem.memory[1], 0x0);
}

#[test]
fn test_read_u16() {
    let mut mem = Memory::new();
    mem.memory[0] = 0x01;
    mem.memory[1] = 0x00;
    mem.memory[2] = 0x02;

    let result_1 = mem.read_u16(0);
    let result_2 = mem.read_u16(1);

    assert_eq!(result_1, 0x0001);
    assert_eq!(result_2, 0x0200);
}

#[test]
fn test_lead_program() {
    let program = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];

    let mut mem = Memory::new();
    mem.load_program(program.clone());

    assert_eq!(mem.memory[0x8000..0x8005], program);
}