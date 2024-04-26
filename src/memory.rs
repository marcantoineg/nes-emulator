pub struct Memory {
    memory: [u8; 0xFFFF],
    debug: bool,
    hex_dump: Vec<u8>
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; 0xFFFF],
            debug: false,
            hex_dump: vec![]
        } 
    }

    pub fn set_debug(&mut self) {
        self.debug = true;
    }

    pub fn load_program(&mut self, data: Vec<u8>) {
        self.memory[0x8000 .. 0x8000 + data.len()].copy_from_slice(&data[..]);
    }

    pub fn read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr.wrapping_add(1)) as u16;

        return (hi << 8) | (lo as u16);
    }

    pub fn write_u16(&mut self, addr: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;

        self.write(addr, lo);
        self.write(addr.wrapping_add(1), hi);
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let value = self.memory[addr as usize];
        if self.debug {
            self.hex_dump.push(value)
        }
        return value;
    }

    pub fn dump(&self) -> &Vec<u8> {
        return &self.hex_dump;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_u16() {
        let mut mem = Memory::new();
        mem.write_u16(0, 0x0001);

        assert_eq!(mem.memory[0], 0x01);
        assert_eq!(mem.memory[1], 0x00);
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
    fn test_load_program() {
        let program = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];

        let mut mem = Memory::new();
        mem.load_program(program.clone());

        assert_eq!(mem.memory[0x8000..0x8005], program);
    }
}