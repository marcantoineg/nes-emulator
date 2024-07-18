use bitflags::bitflags;

use crate::{
    memory::Memory,
    operation::{AddressingMode, OpName, OPERATIONS_MAP},
};

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: Flags,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub memory: Memory,
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

impl Flags {
    fn from_u8(status_bits: u8) -> Self {
        return Flags::from_bits(status_bits)
            .unwrap_or_else(|| panic!("unable to parse cpu status: {:#010b}", status_bits));
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: Flags::Init,
            program_counter: 0,
            stack_pointer: 0xFF,
            memory: Memory::new(),
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

    fn get_op_target_addr(&mut self, mode: &AddressingMode) -> u16 {
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
            let op = OPERATIONS_MAP
                .get(&op_code)
                .unwrap_or_else(|| panic!("unrecognized operation: 0x{:02X?}", op_code));

            self.program_counter += 1;

            match op.mnemonic_name {
                ADC => self.adc(&op.addressing_mode),
                AND => self.and(&op.addressing_mode),
                ASL => self.asl(&op.addressing_mode),
                BCC => self.bcc(),
                BCS => self.bcs(),
                BEQ => self.beq(),
                BMI => self.bmi(),
                BNE => self.bne(),
                BPL => self.bpl(),
                BVC => self.bvc(),
                BVS => self.bvs(),
                BIT => self.bit(&op.addressing_mode),
                BRK => return, // todo: actually implement this
                CLC => self.set_carry_flag(false),
                CLD => self.set_decimal_flag(false),
                CLI => self.set_interupt_flag(false),
                CLV => self.set_overflow_flag(false),
                CMP => self.cmp(&op.addressing_mode),
                CPX => self.cpx(&op.addressing_mode),
                CPY => self.cpy(&op.addressing_mode),
                DEC => self.dec(&op.addressing_mode),
                DEX => self.dex(),
                DEY => self.dey(),
                EOR => self.eor(&op.addressing_mode),
                INC => self.inc(&op.addressing_mode),
                INX => self.inx(),
                INY => self.iny(),
                JMP => self.jmp(&op.addressing_mode),
                JSR => self.jsr(),
                LDA => self.lda(&op.addressing_mode),
                LDX => self.ldx(&op.addressing_mode),
                LDY => self.ldy(&op.addressing_mode),
                LSR => self.lsr(&op.addressing_mode),
                NOP => {}
                ORA => self.ora(&op.addressing_mode),
                PHA => self.pha(),
                PHP => self.php(),
                PLA => self.pla(),
                PLP => self.plp(),
                ROL => self.rol(&op.addressing_mode),
                ROR => self.ror(&op.addressing_mode),
                RTI => self.rti(),
                TAX => self.tax(),
                TAY => self.tay(),

                _ => todo!("op not implemented"),
            }

            match op.mnemonic_name {
                JMP | JSR => {
                    // no-op
                }
                _ => {
                    self.program_counter += (op.bytes - 1) as u16;
                }
            }
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
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

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);
        self.set_register_a(self.register_a & mem_value)
    }

    fn asl(&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::Implied {
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

        let addr = self.get_op_target_addr(&AddressingMode::Immediate);
        let offset = self.memory.read(addr);
        if offset == 0 {
            return;
        }

        let usigned_offset = (offset & 0b0111_1111) as u16;
        if offset & 0b1000_0000 != 0 {
            self.program_counter -= usigned_offset;
        } else {
            self.program_counter += usigned_offset;
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);

        self.set_zero_flag(self.register_a & mem_value);
        self.set_negative_flag(mem_value & Flags::Negative.bits());
        self.set_overflow_flag(mem_value & Flags::Overflow.bits() != 0);
    }

    fn cmp(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.register_a);
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.register_x);
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.register_y);
    }

    fn compare(&mut self, mode: &AddressingMode, register_value: u8) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);

        let result = register_value.wrapping_sub(mem_value);
        self.set_carry_flag(register_value >= mem_value);
        self.set_zero_flag(result);
        self.set_negative_flag(result);
    }

    fn dec(&mut self, mode: &AddressingMode) {
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

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);
        let result = self.register_a ^ mem_value;
        self.set_register_a(result);
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);
        self.set_register_a(mem_value);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);
        self.set_register_x(mem_value);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
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

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);

        self.set_memory(addr, mem_value.wrapping_add(1));
    }

    fn inx(&mut self) {
        let v = self.register_x.wrapping_add(1);
        self.set_register_x(v);
    }

    fn iny(&mut self) {
        let v = self.register_y.wrapping_add(1);
        self.set_register_y(v);
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_target_addr(mode);

        let mut v = addr;
        if *mode == AddressingMode::Indirect {
            v = self.memory.read_u16(addr);
        }

        self.program_counter = v;
    }

    fn jsr(&mut self) {
        let subroutine_addr = self.get_op_target_addr(&AddressingMode::Absolute);

        // the subroutine return address on the stack
        // points to the second byte of data for JSR (ie.: 0x20, 0x00, ->0xFF<-)
        self.push_u16_to_stack(self.program_counter + 1);
        self.program_counter = subroutine_addr;
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::Implied {
            self.set_carry_flag((self.register_a & 0b0000_0001) != 0);
            self.set_register_a(self.register_a >> 1);
        } else {
            let addr = self.get_op_target_addr(mode);
            let mem_value = self.memory.read(addr);

            self.set_carry_flag((mem_value & 0b0000_0001) != 0);
            self.set_memory(addr, mem_value >> 1);
        }
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_target_addr(mode);
        let mem_value = self.memory.read(addr);

        self.set_register_a(self.register_a | mem_value);
    }

    fn pha(&mut self) {
        self.push_to_stack(self.register_a);
    }

    fn php(&mut self) {
        self.push_to_stack(self.status.bits());
    }

    fn pla(&mut self) {
        let v = self.pop_stack();
        self.set_register_a(v);
    }

    fn plp(&mut self) {
        self.status = Flags::from_u8(self.pop_stack());
    }

    fn rol(&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::Implied {
            self.rol_acc();
        } else {
            self.rol_mem(mode);
        }
    }

    fn rol_acc(&mut self) {
        let carry_in = self.carry_flag();
        let mut value = self.register_a;
        let carry_out = value & 0b1000_0000 != 0;

        value = value << 1;
        if carry_in {
            value += 1;
        }

        self.set_register_a(value);
        self.set_carry_flag(carry_out);
    }

    fn rol_mem(&mut self, mode: &AddressingMode) {
        let carry_in = self.carry_flag();
        let addr = self.get_op_target_addr(mode);
        let mut value = self.memory.read(addr);
        let carry_out = value & 0b1000_0000 != 0;

        value = value << 1;
        if carry_in {
            value += 1;
        }

        self.set_memory(addr, value);
        self.set_carry_flag(carry_out);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::Implied {
            self.ror_acc();
        } else {
            self.ror_mem(mode);
        }
    }

    fn ror_acc(&mut self) {
        let carry_in = self.carry_flag();
        let mut value = self.register_a;
        let carry_out = value & 0b0000_0001 != 0;

        value = value >> 1;
        if carry_in {
            value = value | 0b1000_0000;
        }

        self.set_register_a(value);
        self.set_carry_flag(carry_out);
    }

    fn ror_mem(&mut self, mode: &AddressingMode) {
        let carry_in = self.carry_flag();
        let addr = self.get_op_target_addr(mode);
        let mut value = self.memory.read(addr);
        let carry_out = value & 0b0000_0001 != 0;

        value = value >> 1;
        if carry_in {
            value = value | 0b1000_0000;
        }

        self.set_memory(addr, value);
        self.set_carry_flag(carry_out);
    }

    fn rti(&mut self) {
        self.status = Flags::from_u8(self.pop_stack());
        self.program_counter = self.pop_u16_from_stack();
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

    fn push_u16_to_stack(&mut self, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;

        self.push_to_stack(hi);
        self.push_to_stack(lo);
    }

    fn push_to_stack(&mut self, value: u8) {
        self.memory.write(self.stack_pointer_u16(), value);
        self.stack_pointer -= 1;
    }

    fn pop_u16_from_stack(&mut self) -> u16 {
        let lo = self.pop_stack() as u16;
        let hi = self.pop_stack() as u16;

        return (hi << 8) | lo;
    }

    fn pop_stack(&mut self) -> u8 {
        let stack_pointer_u16 = self.stack_pointer_u16() + 1;
        let value = self.memory.read(stack_pointer_u16);
        self.memory.write(stack_pointer_u16, 0x00);
        self.stack_pointer += 1;
        return value;
    }

    fn stack_pointer_u16(&self) -> u16 {
        return 0x0100 + self.stack_pointer as u16;
    }
}

#[cfg(test)]
mod tests {
    use super::CPU;

    #[test]
    fn test_stack_pushes_and_pops_correctly() {
        let mut cpu = CPU::new();
        assert_eq!(cpu.stack_pointer, 0xFF);

        cpu.push_to_stack(0x55);
        assert_eq!(cpu.memory.read(0x01FF), 0x55);
        assert_eq!(cpu.stack_pointer, 0xFE);

        cpu.push_to_stack(0x02);
        assert_eq!(cpu.memory.read(0x01FE), 0x02);
        assert_eq!(cpu.stack_pointer, 0xFD);

        cpu.pop_stack();
        assert_eq!(cpu.memory.read(0x01FE), 0x00);
        assert_eq!(cpu.stack_pointer, 0xFE);
    }

    #[test]
    fn test_stack_pushes_and_pops_u16_correctly() {
        let mut cpu = CPU::new();
        assert_eq!(cpu.stack_pointer, 0xFF);

        cpu.push_to_stack(0x55);
        assert_eq!(cpu.memory.read(0x01FF), 0x55);
        assert_eq!(cpu.stack_pointer, 0xFE);

        cpu.push_u16_to_stack(0x1011);
        assert_eq!(cpu.memory.read(0x01FE), 0x10);
        assert_eq!(cpu.memory.read(0x01FD), 0x11);
        assert_eq!(cpu.stack_pointer, 0xFC);

        cpu.pop_u16_from_stack();
        assert_eq!(cpu.memory.read(0x01FE), 0x00);
        assert_eq!(cpu.memory.read(0x01FD), 0x00);
        assert_eq!(cpu.stack_pointer, 0xFE);

        assert_eq!(cpu.memory.read(0x01FF), 0x55);
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
            cpu.push_to_stack(n as u8);
        }
    }
}
