use std::collections::HashMap;

use bitflags::bitflags;

use crate::{memory::Memory, operation::{AddressingMode, OpName, Operation}};

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: Flags,
    pub program_counter: u16,
    pub stack_pointer: u8,
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

            (0x90, Operation::new(BCC, Relative, 2)),
            (0xB0, Operation::new(BCS, Relative, 2)),
            (0xF0, Operation::new(BEQ, Relative, 2)),
            (0x30, Operation::new(BMI, Relative, 2)),
            (0xD0, Operation::new(BNE, Relative, 2)),
            (0x10, Operation::new(BPL, Relative, 2)),
            (0x50, Operation::new(BVC, Relative, 2)),
            (0x70, Operation::new(BVS, Relative, 2)),

            (0x24, Operation::new(BIT, ZeroPage, 2)),
            (0x2C, Operation::new(BIT, Absolute, 3)),

            (0x00, Operation::new(BRK, Implied, 1)),

            (0x18, Operation::new(CLC, Implied, 1)),
            (0xD8, Operation::new(CLD, Implied, 1)),
            (0x58, Operation::new(CLI, Implied, 1)),
            (0xB8, Operation::new(CLV, Implied, 1)),

            (0xC9, Operation::new(CMP, Immediate, 2)),
            (0xC5, Operation::new(CMP, ZeroPage, 2)),
            (0xD5, Operation::new(CMP, ZeroPageX, 2)),
            (0xCD, Operation::new(CMP, Absolute, 3)),
            (0xDD, Operation::new(CMP, AbsoluteX, 3)),
            (0xD9, Operation::new(CMP, AbsoluteY, 3)),
            (0xC1, Operation::new(CMP, IndirectX, 2)),
            (0xD1, Operation::new(CMP, IndirectY, 2)),

            (0xE0, Operation::new(CPX, Immediate, 2)),
            (0xE4, Operation::new(CPX, ZeroPage, 2)),
            (0xEC, Operation::new(CPX, Absolute, 3)),

            (0xC0, Operation::new(CPY, Immediate, 2)),
            (0xC4, Operation::new(CPY, ZeroPage, 2)),
            (0xCC, Operation::new(CPY, Absolute, 3)),

            (0xC6, Operation::new(DEC, ZeroPage, 2)),
            (0xD6, Operation::new(DEC, ZeroPageX, 2)),
            (0xCE, Operation::new(DEC, Absolute, 3)),
            (0xDE, Operation::new(DEC, AbsoluteX, 3)),

            (0xCA, Operation::new(DEX, Implied, 1)),

            (0x88, Operation::new(DEY, Implied, 1)),

            (0x49, Operation::new(EOR, Immediate, 2)),
            (0x45, Operation::new(EOR, ZeroPage, 2)),
            (0x55, Operation::new(EOR, ZeroPageX, 2)),
            (0x4D, Operation::new(EOR, Absolute, 3)),
            (0x5D, Operation::new(EOR, AbsoluteX, 3)),
            (0x59, Operation::new(EOR, AbsoluteY, 3)),
            (0x41, Operation::new(EOR, IndirectX, 2)),
            (0x51, Operation::new(EOR, IndirectY, 2)),

            (0xE6, Operation::new(INC, ZeroPage, 2)),
            (0xF6, Operation::new(INC, ZeroPageX, 2)),
            (0xEE, Operation::new(INC, Absolute, 3)),
            (0xFE, Operation::new(INC, AbsoluteX, 3)),

            (0xE8, Operation::new(INX, Implied, 2)),

            (0xC8, Operation::new(INY, Implied, 2)),

            (0x4C, Operation::new(JMP, Absolute, 3)),
            (0x6C, Operation::new(JMP, Indirect, 3)),
            
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
        ]);

        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: Flags::Init,
            program_counter: 0,
            stack_pointer: 0xFF,
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
            Immediate | Relative => {
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
            Indirect => {
                return self.memory.read_u16(self.program_counter);
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
                BEQ => self.beq(),
                BMI => self.bmi(),
                BNE => self.bne(),
                BPL => self.bpl(),
                BVC => self.bvc(),
                BVS => self.bvs(),
                BIT => self.bit(op.addressing_mode),
                BRK => return,
                CLC => self.set_carry_flag(false),
                CLD => self.set_decimal_flag(false),
                CLI => self.set_interupt_flag(false),
                CLV => self.set_overflow_flag(false),
                CMP => self.cmp(op.addressing_mode),
                CPX => self.cpx(op.addressing_mode),
                CPY => self.cpy(op.addressing_mode),
                DEC => self.dec(op.addressing_mode),
                DEX => self.dex(),
                DEY => self.dey(),
                EOR => self.eor(op.addressing_mode),
                INC => self.inc(op.addressing_mode),
                INX => self.inx(),
                INY => self.iny(),
                JMP => self.jmp(op.addressing_mode),
                LDA => self.lda(op.addressing_mode),
                LDX => self.ldx(op.addressing_mode),
                LDY => self.ldy(op.addressing_mode),
                TAX => self.tax(),
                TAY => self.tay(),

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

    fn beq(&mut self) {
        let condition = self.zero_flag();
        self.branch(condition);
    }

    fn bmi(&mut self) {
        let condition = self.negative_flag();
        self.branch(condition);
    }

    fn bne(&mut self) {
        let condition = !self.zero_flag();
        self.branch(condition);
    }
    
    fn bpl(&mut self) {
        let condition = !self.negative_flag();
        self.branch(condition);
    }

    fn bvc(&mut self) {
        let condition = !self.overflow_flag();
        self.branch(condition);
    }

    fn bvs(&mut self) {
        let condition = self.overflow_flag();
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

    fn bit(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);

        self.set_zero_flag(self.register_a & mem_value);
        self.set_negative_flag(mem_value & Flags::Negative.bits());
        self.set_overflow_flag(mem_value & Flags::Overflow.bits() != 0);
    }

    fn cmp(&mut self, mode: AddressingMode) {
        self.compare(mode, self.register_a);
    }

    fn cpx(&mut self, mode: AddressingMode) {
        self.compare(mode, self.register_x);
    }

    fn cpy(&mut self, mode: AddressingMode) {
        self.compare(mode, self.register_y);
    }

    fn compare(&mut self, mode: AddressingMode, register_value: u8) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);

        let result = register_value.wrapping_sub(mem_value);
        self.set_carry_flag(register_value >= mem_value);
        self.set_zero_flag(result);
        self.set_negative_flag(result);
    }

    fn dec(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);

        let result = mem_value.wrapping_sub(1);
        self.set_memory(addr, result);
    }

    fn dex(&mut self) {
        let result = self.register_x.wrapping_sub(1);
        self.set_register_x(result);
    }

    fn dey(&mut self) {
        let result = self.register_y.wrapping_sub(1);
        self.set_register_y(result);
    }

    fn eor(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);
        let result = self.register_a ^ mem_value;
        self.set_register_a(result);
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

    fn inc(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);

        self.set_memory(addr, mem_value.wrapping_add(1));
    }

    fn inx(&mut self) {
        let v = self.register_x.wrapping_add(1);
        self.set_register_x(v);
    }

    fn iny(&mut self) {
        let  v = self.register_y.wrapping_add(1);
        self.set_register_y(v);
    }

    fn jmp(&mut self, mode: AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        self.program_counter = addr;
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

    fn set_memory(&mut self, addr: u16, value: u8) {
        self.memory.write(addr, value);
        self.set_zero_flag(value);
        self.set_negative_flag(value);
    }

    fn zero_flag(&mut self) -> bool {
        return self.status.contains(Flags::Zero);
    }

    fn set_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status.insert(Flags::Zero);
        } else {
            self.status.remove(Flags::Zero);
        }
    }

    fn negative_flag(&mut self) -> bool {
        return self.status.contains(Flags::Negative);
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

    fn overflow_flag(&mut self) -> bool {
        return self.status.contains(Flags::Overflow);
    }

    fn set_overflow_flag(&mut self, value: bool) {
        if value {
            self.status.insert(Flags::Overflow);
        } else {
            self.status.remove(Flags::Overflow);
        }
    }

    fn set_interupt_flag(&mut self, value: bool) {
        if value {
            self.status.insert(Flags::InteruptDisable);
        } else {
            self.status.remove(Flags::InteruptDisable);
        }
    }

    fn set_decimal_flag(&mut self, value: bool) {
        if value {
            self.status.insert(Flags::Decimal);
        } else {
            self.status.remove(Flags::Decimal);
        }
    }

    #[allow(dead_code)] //todo: remove when used by operations
    fn push_stack(&mut self, value: u8) {
        self.memory.write(self.stack_pointer_u16(), value);
        self.stack_pointer -= 1;
    }

    #[allow(dead_code)] //todo: remove when used by operations
    fn pop_stack(&mut self) -> u8 {
        let stack_pointer_u16 = self.stack_pointer_u16() + 1;
        let value = self.memory.read(stack_pointer_u16);
        self.memory.write(stack_pointer_u16, 0x00);
        self.stack_pointer += 1;
        return value;
    }

    #[allow(dead_code)] //todo: remove when used by operations
    fn stack_pointer_u16(&self) -> u16 {
        return 0x0100 + self.stack_pointer as u16;
    }
}

#[cfg(test)]
mod tests {
    use super::CPU;

    #[test]
    fn test_stack_pushes_correctly() {
        let mut cpu = CPU::new();
        assert_eq!(cpu.stack_pointer, 0xFF);
        
        cpu.push_stack(0x55);
        assert_eq!(cpu.memory.read(0x01FF), 0x55);
        assert_eq!(cpu.stack_pointer, 0xFE);

        cpu.push_stack(0x02);
        assert_eq!(cpu.memory.read(0x01FE), 0x02);
        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    #[test]
    fn test_stack_pops_correctly() {
        let mut cpu = CPU::new();
        assert_eq!(cpu.stack_pointer, 0xFF);
        
        cpu.push_stack(0x55);
        assert_eq!(cpu.memory.read(0x01FF), 0x55);
        assert_eq!(cpu.stack_pointer, 0xFE);

        cpu.pop_stack();
        assert_eq!(cpu.memory.read(0x01FF), 0x00);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    #[should_panic]
    fn test_stack_panic_when_pointer_overflows() {
        let mut cpu = CPU::new();
        cpu.pop_stack();
    }

    #[test]
    #[should_panic]
    fn test_stack_panic_when_pointer_underflow() {
        let mut cpu = CPU::new();
        for n in 1..257 {
            cpu.push_stack(n as u8);
        }
    }
}