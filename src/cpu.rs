use std::collections::HashMap;

use crate::{memory::Memory, operation::{AddressingMode, OpName, Operation}};

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8, // NVB_DIZC
    pub program_counter: u16,
    pub memory: Memory,

    operations_map: HashMap<u8, Operation>,
}

impl CPU {
    pub fn new() -> Self {
        use AddressingMode::*;
        use OpName::*;

        let operations: HashMap<u8, Operation> = HashMap::from([
            (0x00, Operation::new(BRK, Implied, 1)),
            
            (0xA9, Operation::new(LDA, Immediate, 2)),
            (0xA5, Operation::new(LDA, ZeroPage, 2)),
            (0xB5, Operation::new(LDA, ZeroPageX, 2)),
            (0xAD, Operation::new(LDA, Absolute, 3)),
            (0xBD, Operation::new(LDA, AbsoluteX, 3)),
            (0xB9, Operation::new(LDA, AbsoluteY, 3)),
            (0xA1, Operation::new(LDA, IndirectX, 2)),
            (0xB1, Operation::new(LDA, IndirectY, 2)),

            (0xAA, Operation::new(TAX, Implied, 1)),
            
            (0xE8, Operation::new(INX, Implied, 2)),
        ]);

        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b0000_0000,
            program_counter: 0,
            memory: Memory::new(),
            operations_map: operations,
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
            }
            ZeroPage => {
                return self.memory.read(self.program_counter) as u16;
            }
            ZeroPageX => {
                let addr = self.memory.read(self.program_counter) as u16;
                return addr.wrapping_add(self.register_x as u16);
            }
            ZeroPageY => {
                let addr = self.memory.read(self.program_counter) as u16;
                return addr.wrapping_add(self.register_y as u16);
            }
            Absolute => {
                return self.memory.read_u16(self.program_counter);
            }
            AbsoluteX => {
                let addr = self.memory.read_u16(self.program_counter);
                return addr.wrapping_add(self.register_x as u16);
            }
            AbsoluteY => {
                let addr = self.memory.read_u16(self.program_counter);
                return addr.wrapping_add(self.register_y as u16);
            }
            IndirectX => {
                let mut addr = self.memory.read(self.program_counter);
                addr = addr.wrapping_add(self.register_x);
                return self.memory.read_u16(addr as u16);
            }
            IndirectY => {
                let mut addr = self.memory.read(self.program_counter);
                addr = addr.wrapping_add(self.register_y);
                return self.memory.read_u16(addr as u16);
            }
            Implied => {
                panic!("operation does not require target address");
            }
        }
    }

    fn run(&mut self) {
        use OpName::*;

        loop {
            let op_code = self.memory.read(self.program_counter);
            let op = self.operations_map[&op_code];

            self.program_counter += 1;

            match op.mnemonic_name {
                LDA => {
                    let op = self.operations_map[&op_code];
                    self.lda(op.addressing_mode);
                }

                // TAX
                TAX => {
                    self.register_x = self.register_a;

                    self.set_zero_flag(self.register_x);
                    self.set_negative_flag(self.register_x);
                }

                // INX
                INX => {
                    self.register_x = self.register_x.wrapping_add(1);

                    self.set_zero_flag(self.register_x);
                    self.set_negative_flag(self.register_x);
                }

                // BRK
                BRK => return,

                _ => todo!(),
            }

            self.program_counter += (op.bytes - 1) as u16;
        }
    }

    fn lda(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        self.register_a = self.memory.read(addr);

        self.set_zero_flag(self.register_a);
        self.set_negative_flag(self.register_a);
    }

    fn set_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }
    }

    pub fn zero_flag(&mut self) -> bool {
        return self.status & 0b0000_0010 != 0;
    }

    fn set_negative_flag(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000
        } else {
            self.status = self.status & 0b0111_1111
        }
    }

    pub fn negative_flag(&mut self) -> bool {
        return self.status & 0b1000_0000 != 0;
    }
}
