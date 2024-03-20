use crate::memory::Memory;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8, // NVB_DIZC
    pub program_counter: u16,
    pub memory: Memory,
}

enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    None,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b0000_0000,
            program_counter: 0,
            memory: Memory::new()
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset(true);
        self.run();
    }

    pub fn load_and_run_without_reset(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset(false);
        self.run();
    }

    fn load(&mut self, program: Vec<u8>) {
        self.memory.load_program(program);
        self.memory.write_u16(0xFFFC, 0x8000);
    }

    fn reset(&mut self, reset_registers_and_status: bool) {
        if reset_registers_and_status {
            self.register_a = 0;
            self.register_x = 0;
            self.register_y = 0;
            self.status = 0b0000_0000;
        }

        self.program_counter = self.memory.read_u16(0xFFFC);
    }

    fn get_op_target_addr(&mut self, mode: AddressingMode) -> u16 {
        use AddressingMode::*;
        match mode {
            Immediate => {
                return self.program_counter;
            },
            ZeroPage => {
                return self.memory.read(self.program_counter) as u16;
            },
            ZeroPageX => {
                let addr = self.memory.read(self.program_counter) as u16;
                return addr.wrapping_add(self.register_x as u16);
            },
            ZeroPageY => {
                let addr = self.memory.read(self.program_counter) as u16;
                return addr.wrapping_add(self.register_y as u16);
            },
            Absolute => {
                return self.memory.read_u16(self.program_counter);
            },
            AbsoluteX => {
                let addr = self.memory.read_u16(self.program_counter);
                return addr.wrapping_add(self.register_x as u16);
            },
            AbsoluteY => {
                let addr = self.memory.read_u16(self.program_counter);
                return addr.wrapping_add(self.register_y as u16);
            },
            IndirectX => {
                let mut addr = self.memory.read(self.program_counter);
                addr = addr.wrapping_add(self.register_x);
                return self.memory.read_u16(addr as u16);
            },
            IndirectY => {
                let mut addr = self.memory.read(self.program_counter);
                addr = addr.wrapping_add(self.register_y);
                return self.memory.read_u16(addr as u16);
            },
            None => {
                panic!("type not supported");
            }
        }
    }

    fn run(&mut self) {
        loop {
            let operation_code = self.memory.read(self.program_counter);
            self.program_counter += 1;

            match operation_code {
                // LDA
                0xA9 => {
                    self.lda(AddressingMode::Immediate);
                    self.program_counter += 1;
                },
                0xA5 => {
                    self.lda(AddressingMode::ZeroPage);
                    self.program_counter += 1;
                },
                0xB5 => {
                    self.lda(AddressingMode::ZeroPageX);
                    self.program_counter += 2;
                },
                0xAD => {
                    self.lda(AddressingMode::Absolute);
                    self.program_counter += 2;
                },
                0xBD => {
                    self.lda(AddressingMode::AbsoluteX);
                    self.program_counter += 2;
                },
                0xB9 => {
                    self.lda(AddressingMode::AbsoluteY);
                    self.program_counter += 2;
                },
                0xA1 => {
                    self.lda(AddressingMode::IndirectX);
                    self.program_counter += 1;
                },
                0xB1 => {
                    self.lda(AddressingMode::IndirectY);
                    self.program_counter += 1;
                },

                // TAX
                0xAA => {
                    self.register_x = self.register_a;

                    self.set_zero_flag(self.register_x);
                    self.set_negative_flag(self.register_x);
                },

                // INX
                0xE8 => {
                    if self.register_x == u8::MAX {
                        self.register_x = 0
                    } else {
                        self.register_x += 1;
                    }


                    self.set_zero_flag(self.register_x);
                    self.set_negative_flag(self.register_x);
                },

                // BRK
                0x00 => return,
                
                _ => todo!()
            }
        }
    }

    fn set_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status = self.status | 0b0000_0010; 
        } else {
            self.status = self.status & 0b1111_1101;
        }
    }

    fn set_negative_flag(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000
        } else {
            self.status = self.status & 0b0111_1111
        }
    }

    fn lda(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        self.register_a = self.memory.read(addr);

        self.set_zero_flag(self.register_a);
        self.set_negative_flag(self.register_a);
    }

    pub fn zero_flag(&mut self) -> bool {
        return self.status & 0b0000_0010 != 0
    }
    
    pub fn negative_flag(&mut self) -> bool {
        return self.status & 0b1000_0000 != 0
    }
}