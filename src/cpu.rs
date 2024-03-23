use std::collections::HashMap;

use bitflags::bitflags;

use crate::{memory::Memory, operation::{AddressingMode, OpName, Operation}};

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: Flags,
    pub program_counter: u16,
    pub memory: Memory,

    operations_map: HashMap<u8, Operation>,
}

bitflags! {
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    pub struct Flags: u8 {
        const Init = 0b0011_0000;
        const Carry = 0b0000_0001;
        const Zero = 0b0000_0010;
        const InteruptDisable = 0b0000_0100;
        const Decimal = 0b0000_1000;
        const Break = 0b0001_0000;
        const Break2 = 0b0010_0000;
        const Overflow = 0b0100_0000;
        const Negative = 0b1000_0000;
    }
}

impl CPU {
    pub fn new() -> Self {
        use AddressingMode::*;
        use OpName::*;

        let operations: HashMap<u8, Operation> = HashMap::from([
            (0x69/*noice*/, Operation::new(ADC, Immediate, 2)),

            (0x00, Operation::new(BRK, Implied, 1)),
            
            (0xA9, Operation::new(LDA, Immediate, 2)),
            (0xA5, Operation::new(LDA, ZeroPage, 2)),
            (0xB5, Operation::new(LDA, ZeroPageX, 2)),
            (0xAD, Operation::new(LDA, Absolute, 3)),
            (0xBD, Operation::new(LDA, AbsoluteX, 3)),
            (0xB9, Operation::new(LDA, AbsoluteY, 3)),
            (0xA1, Operation::new(LDA, IndirectX, 2)),
            (0xB1, Operation::new(LDA, IndirectY, 2)),

            (0xA2, Operation::new(LDX, Immediate, 2)),
            (0xA6, Operation::new(LDX, ZeroPage, 2)),
            (0xB6, Operation::new(LDX, ZeroPageY, 2)),
            (0xAE, Operation::new(LDX, Absolute, 3)),
            (0xBE, Operation::new(LDX, AbsoluteY, 3)),

            (0xA0, Operation::new(LDY, Immediate, 2)),
            (0xA4, Operation::new(LDY, ZeroPage, 2)),
            (0xB4, Operation::new(LDY, ZeroPageX, 2)),
            (0xAC, Operation::new(LDY, Absolute, 3)),
            (0xBC, Operation::new(LDY, AbsoluteX, 3)),

            (0xAA, Operation::new(TAX, Implied, 1)),
            (0xA8, Operation::new(TAY, Implied, 1)),
            
            (0xE8, Operation::new(INX, Implied, 2)),
        ]);

        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: Flags::Init,
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
            self.status = Flags::Init;
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
                let addr = self.memory.read(self.program_counter);
                return addr.wrapping_add(self.register_x) as u16;
            }
            ZeroPageY => {
                let addr = self.memory.read(self.program_counter);
                return addr.wrapping_add(self.register_y) as u16;
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
                ADC => self.adc(op.addressing_mode),

                LDA => self.lda(op.addressing_mode),
                LDX => self.ldx(op.addressing_mode),
                LDY => self.ldy(op.addressing_mode),

                TAX => self.tax(),
                TAY => self.tay(),

                INX => {
                    let v = self.register_x.wrapping_add(1);
                    self.set_register_x(v);
                }

                // BRK
                BRK => return,

                _ => todo!("op not implemented"),
            }

            self.program_counter += (op.bytes - 1) as u16;
        }
    }

    fn adc(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);

        let wrapped_sum = self.register_a.wrapping_add(mem_value);
        
        self.set_carry_flag(wrapped_sum < self.register_a);
        self.set_register_a(wrapped_sum);
    }

    fn lda(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);
        self.set_register_a(mem_value);
    }

    fn ldx(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);
        self.set_register_x(mem_value);
    }

    fn ldy(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);
        self.set_register_y(mem_value);
    }

    fn tax(&mut self) {
        self.set_register_x(self.register_a);
    }

    fn tay(&mut self) {
        self.set_register_y(self.register_a);
    }

    fn set_register_a(&mut self, value: u8) {
        self.register_a = value;
        self.set_zero_flag(self.register_a);
        self.set_negative_flag(self.register_a)
    }

    fn set_register_x(&mut self, value: u8) {
        self.register_x = value;
        self.set_zero_flag(self.register_x);
        self.set_negative_flag(self.register_x)
    }

    fn set_register_y(&mut self, value: u8) {
        self.register_y = value;
        self.set_zero_flag(self.register_y);
        self.set_negative_flag(self.register_y)
    }

    pub fn zero_flag(&mut self) -> bool {
        return self.status.contains(Flags::Zero);
    }

    fn set_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status.insert(Flags::Zero);
        } else {
            self.status.remove(Flags::Zero);
        }
    }

    fn set_negative_flag(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.status.insert(Flags::Negative);
        } else {
            self.status.remove(Flags::Negative);
        }
    }

    pub fn negative_flag(&mut self) -> bool {
        return self.status.contains(Flags::Negative);
    }

    fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.status.insert(Flags::Carry);
        } else {
            self.status.remove(Flags::Carry);
        }
    }
}
