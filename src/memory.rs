pub struct Memory {
    pub memory: [u8; 0xFFFF]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; 0xFFFF]
        } 
    }

    pub fn load_program(&mut self, data: Vec<u8>) {
        self.memory[0x8000 .. 0x8000 + data.len()].copy_from_slice(&data[..]);
    }

    pub fn read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr + 1) as u16;

        return (hi << 8) | (lo as u16)
    }

    pub fn write_u16(&mut self, addr: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;

        self.write(addr, lo);
        self.write(addr + 1, hi);
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }
}