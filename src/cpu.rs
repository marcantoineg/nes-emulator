pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8, // NVB_DIZC
    pub program_counter: u16,
}

    impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let operation_code = program[self.program_counter as usize];
            self.program_counter += 1;

            match operation_code {
                // LDA
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
            
                    self.register_a = param;

                    self.set_zero_flag(self.register_a == 0);
                    self.set_negative_flag(self.register_a);
                },

                // TAX
                0xAA => {
                    self.register_x = self.register_a;

                    self.set_zero_flag(self.register_x == 0);
                    self.set_negative_flag(self.register_x);
                }

                // BRK
                0x00 => return,
                
                _ => todo!()
            }
        }
    }

    fn set_zero_flag(&mut self, flag_status: bool) {
        if flag_status {
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

    pub fn zero_flag_set(&mut self) -> bool {
        return self.status & 0b0000_0010 != 0
    }
    
    pub fn negative_flag_set(&mut self) -> bool {
        return self.status & 0b1000_0000 != 0
    }
}
 
 