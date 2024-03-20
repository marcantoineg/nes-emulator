use crate::memory::Memory;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8, // NVB_DIZC
    pub program_counter: u16,

    memory: Memory,
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

    fn run(&mut self) {
        loop {
            let operation_code = self.memory.read(self.program_counter);
            self.program_counter += 1;

            match operation_code {
                // LDA
                0xA9 => {
                    let param = self.memory.read(self.program_counter);
                    self.program_counter += 1;
            
                    self.register_a = param;

                    self.set_zero_flag(self.register_a);
                    self.set_negative_flag(self.register_a);
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

    pub fn zero_flag(&mut self) -> bool {
        return self.status & 0b0000_0010 != 0
    }
    
    pub fn negative_flag(&mut self) -> bool {
        return self.status & 0b1000_0000 != 0
    }
}
 
 