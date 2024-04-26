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
    #[derive(PartialEq, Eq, Clone, Copy)]
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
            (0x65, Operation::new(ADC, ZeroPage, 2)),
            (0x75, Operation::new(ADC, ZeroPageX, 2)),
            (0x6D, Operation::new(ADC, Absolute, 3)),
            (0x7D, Operation::new(ADC, AbsoluteX, 3)),
            (0x79, Operation::new(ADC, AbsoluteY, 3)),
            (0x61, Operation::new(ADC, IndirectX, 2)),
            (0x71, Operation::new(ADC, IndirectY, 2)),

            (0x29, Operation::new(AND, Immediate, 2)),
            (0x25, Operation::new(AND, ZeroPage, 2)),
            (0x35, Operation::new(AND, ZeroPageX, 2)),
            (0x2D, Operation::new(AND, Absolute, 3)),
            (0x3D, Operation::new(AND, AbsoluteX, 3)),
            (0x39, Operation::new(AND, AbsoluteY, 3)),
            (0x21, Operation::new(AND, IndirectX, 2)),
            (0x31, Operation::new(AND, IndirectY, 2)),

            (0x0A, Operation::new(ASL, Implied, 1)),
            (0x06, Operation::new(ASL, ZeroPage, 2)),
            (0x16, Operation::new(ASL, ZeroPageX, 2)),
            (0x0E, Operation::new(ASL, Absolute, 3)),
            (0x1E, Operation::new(ASL, AbsoluteX, 3)),

            (0x90, Operation::new(BCC, Immediate, 2)),
            (0xB0, Operation::new(BCS, Immediate, 2)),

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
        self.memory.set_debug();
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
                let zero_page_addr = self.memory.read(self.program_counter);
                let addr = zero_page_addr.wrapping_add(self.register_x);
                return self.memory.read_u16(addr as u16);
            }
            IndirectY => {
                let zero_page_addr = self.memory.read(self.program_counter);
                let addr = self.memory.read_u16(zero_page_addr as u16);
                return addr.wrapping_add(self.register_y as u16);
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
                AND => self.and(op.addressing_mode),
                ASL => self.asl(op.addressing_mode),
                BCC => self.bcc(),
                BCS => self.bcs(),
                BRK => return,
                LDA => self.lda(op.addressing_mode),
                LDX => self.ldx(op.addressing_mode),
                LDY => self.ldy(op.addressing_mode),
                TAX => self.tax(),
                TAY => self.tay(),
                INX => self.inx(),

                _ => todo!("op not implemented"),
            }

            self.program_counter += (op.bytes - 1) as u16;
        }
    }

    fn adc(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);

        let mem_value = self.memory.read(addr);
        let carry_in: u8 = if self.carry_flag() { 1 } else { 0 };

        let wrapped_sum = self.register_a.wrapping_add(mem_value + carry_in);

        let carry_out = wrapped_sum < self.register_a;
        let overflow = ((self.register_a ^ wrapped_sum) & (mem_value ^ wrapped_sum) & 0x80) != 0;

        self.set_carry_flag(carry_out);
        self.set_overflow_flag(overflow);
        self.set_register_a(wrapped_sum);
    }

    fn and(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);
        self.set_register_a(self.register_a & mem_value)
    }

    fn asl(&mut self, mode: AddressingMode) {
        if mode == AddressingMode::Implied {
            let left_shifted_value = self.register_a << 1;
            
            self.set_carry_flag(self.register_a & 0x80 != 0);
            self.set_register_a(left_shifted_value);
        } else {
            let addr = self.get_op_target_addr(mode);
            let mem_value = self.memory.read(addr);

            let left_shifted_value = mem_value << 1;

            self.set_carry_flag(mem_value & 0x80 != 0);
            self.memory.write(addr, left_shifted_value);
        }
    }

    fn bcc(&mut self) {
        let condition = !self.carry_flag();
        self.branch(condition);
    }

    fn bcs(&mut self) {
        let condition = self.carry_flag();
        self.branch(condition);
    }

    fn branch(&mut self, branching_condition: bool) {
        if !branching_condition {
            return;
        }

        let addr = self.get_op_target_addr(AddressingMode::Immediate);
        let offset = self.memory.read(addr);
        if offset == 0 { return; }

        let usigned_offset = (offset & 0b0111_1111) as u16;
        if offset & 0b1000_0000 != 0 {
            self.program_counter -= usigned_offset;
        } else {
            self.program_counter += usigned_offset;
        }
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

    fn inx(&mut self) {
        let v = self.register_x.wrapping_add(1);
        self.set_register_x(v);
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

    fn carry_flag(&mut self) -> bool {
        return self.status.contains(Flags::Carry);
    }

    fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.status.insert(Flags::Carry);
        } else {
            self.status.remove(Flags::Carry);
        }
    }

    fn set_overflow_flag(&mut self, value: bool) {
        if value {
            self.status.insert(Flags::Overflow);
        } else {
            self.status.remove(Flags::Overflow);
        }
    }
}
