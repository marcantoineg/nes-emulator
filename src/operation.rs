use std::collections::HashMap;

lazy_static! {
    pub static ref OPERATIONS_MAP: HashMap<u8, Operation> = {
        use OpName::*;
        use AddressingMode::*;

        let mut m = HashMap::new();
        m.insert(0x69/*noice*/, Operation::new(ADC, Immediate, 2));
        m.insert(0x65, Operation::new(ADC, ZeroPage, 2));
        m.insert(0x75, Operation::new(ADC, ZeroPageX, 2));
        m.insert(0x6D, Operation::new(ADC, Absolute, 3));
        m.insert(0x7D, Operation::new(ADC, AbsoluteX, 3));
        m.insert(0x79, Operation::new(ADC, AbsoluteY, 3));
        m.insert(0x61, Operation::new(ADC, IndirectX, 2));
        m.insert(0x71, Operation::new(ADC, IndirectY, 2));

        m.insert(0x29, Operation::new(AND, Immediate, 2));
        m.insert(0x25, Operation::new(AND, ZeroPage, 2));
        m.insert(0x35, Operation::new(AND, ZeroPageX, 2));
        m.insert(0x2D, Operation::new(AND, Absolute, 3));
        m.insert(0x3D, Operation::new(AND, AbsoluteX, 3));
        m.insert(0x39, Operation::new(AND, AbsoluteY, 3));
        m.insert(0x21, Operation::new(AND, IndirectX, 2));
        m.insert(0x31, Operation::new(AND, IndirectY, 2));

        m.insert(0x0A, Operation::new(ASL, Implied, 1));
        m.insert(0x06, Operation::new(ASL, ZeroPage, 2));
        m.insert(0x16, Operation::new(ASL, ZeroPageX, 2));
        m.insert(0x0E, Operation::new(ASL, Absolute, 3));
        m.insert(0x1E, Operation::new(ASL, AbsoluteX, 3));

        m.insert(0x90, Operation::new(BCC, Relative, 2));
        m.insert(0xB0, Operation::new(BCS, Relative, 2));
        m.insert(0xF0, Operation::new(BEQ, Relative, 2));
        m.insert(0x30, Operation::new(BMI, Relative, 2));
        m.insert(0xD0, Operation::new(BNE, Relative, 2));
        m.insert(0x10, Operation::new(BPL, Relative, 2));
        m.insert(0x50, Operation::new(BVC, Relative, 2));
        m.insert(0x70, Operation::new(BVS, Relative, 2));

        m.insert(0x24, Operation::new(BIT, ZeroPage, 2));
        m.insert(0x2C, Operation::new(BIT, Absolute, 3));

        m.insert(0x00, Operation::new(BRK, Implied, 1));

        m.insert(0x18, Operation::new(CLC, Implied, 1));
        m.insert(0xD8, Operation::new(CLD, Implied, 1));
        m.insert(0x58, Operation::new(CLI, Implied, 1));
        m.insert(0xB8, Operation::new(CLV, Implied, 1));

        m.insert(0xC9, Operation::new(CMP, Immediate, 2));
        m.insert(0xC5, Operation::new(CMP, ZeroPage, 2));
        m.insert(0xD5, Operation::new(CMP, ZeroPageX, 2));
        m.insert(0xCD, Operation::new(CMP, Absolute, 3));
        m.insert(0xDD, Operation::new(CMP, AbsoluteX, 3));
        m.insert(0xD9, Operation::new(CMP, AbsoluteY, 3));
        m.insert(0xC1, Operation::new(CMP, IndirectX, 2));
        m.insert(0xD1, Operation::new(CMP, IndirectY, 2));

        m.insert(0xE0, Operation::new(CPX, Immediate, 2));
        m.insert(0xE4, Operation::new(CPX, ZeroPage, 2));
        m.insert(0xEC, Operation::new(CPX, Absolute, 3));

        m.insert(0xC0, Operation::new(CPY, Immediate, 2));
        m.insert(0xC4, Operation::new(CPY, ZeroPage, 2));
        m.insert(0xCC, Operation::new(CPY, Absolute, 3));

        m.insert(0xC6, Operation::new(DEC, ZeroPage, 2));
        m.insert(0xD6, Operation::new(DEC, ZeroPageX, 2));
        m.insert(0xCE, Operation::new(DEC, Absolute, 3));
        m.insert(0xDE, Operation::new(DEC, AbsoluteX, 3));

        m.insert(0xCA, Operation::new(DEX, Implied, 1));

        m.insert(0x88, Operation::new(DEY, Implied, 1));

        m.insert(0x49, Operation::new(EOR, Immediate, 2));
        m.insert(0x45, Operation::new(EOR, ZeroPage, 2));
        m.insert(0x55, Operation::new(EOR, ZeroPageX, 2));
        m.insert(0x4D, Operation::new(EOR, Absolute, 3));
        m.insert(0x5D, Operation::new(EOR, AbsoluteX, 3));
        m.insert(0x59, Operation::new(EOR, AbsoluteY, 3));
        m.insert(0x41, Operation::new(EOR, IndirectX, 2));
        m.insert(0x51, Operation::new(EOR, IndirectY, 2));

        m.insert(0xE6, Operation::new(INC, ZeroPage, 2));
        m.insert(0xF6, Operation::new(INC, ZeroPageX, 2));
        m.insert(0xEE, Operation::new(INC, Absolute, 3));
        m.insert(0xFE, Operation::new(INC, AbsoluteX, 3));

        m.insert(0xE8, Operation::new(INX, Implied, 2));

        m.insert(0xC8, Operation::new(INY, Implied, 2));

        m.insert(0x4C, Operation::new(JMP, Absolute, 3));
        m.insert(0x6C, Operation::new(JMP, Indirect, 3));

        m.insert(0x20, Operation::new(JSR, Absolute, 3));
            
        m.insert(0xA9, Operation::new(LDA, Immediate, 2));
        m.insert(0xA5, Operation::new(LDA, ZeroPage, 2));
        m.insert(0xB5, Operation::new(LDA, ZeroPageX, 2));
        m.insert(0xAD, Operation::new(LDA, Absolute, 3));
        m.insert(0xBD, Operation::new(LDA, AbsoluteX, 3));
        m.insert(0xB9, Operation::new(LDA, AbsoluteY, 3));
        m.insert(0xA1, Operation::new(LDA, IndirectX, 2));
        m.insert(0xB1, Operation::new(LDA, IndirectY, 2));

        m.insert(0xA2, Operation::new(LDX, Immediate, 2));
        m.insert(0xA6, Operation::new(LDX, ZeroPage, 2));
        m.insert(0xB6, Operation::new(LDX, ZeroPageY, 2));
        m.insert(0xAE, Operation::new(LDX, Absolute, 3));
        m.insert(0xBE, Operation::new(LDX, AbsoluteY, 3));

        m.insert(0xA0, Operation::new(LDY, Immediate, 2));
        m.insert(0xA4, Operation::new(LDY, ZeroPage, 2));
        m.insert(0xB4, Operation::new(LDY, ZeroPageX, 2));
        m.insert(0xAC, Operation::new(LDY, Absolute, 3));
        m.insert(0xBC, Operation::new(LDY, AbsoluteX, 3));

        m.insert(0x4A, Operation::new(LSR, Implied, 1));
        m.insert(0x46, Operation::new(LSR, ZeroPage, 2));
        m.insert(0x56, Operation::new(LSR, ZeroPageX, 2));
        m.insert(0x4E, Operation::new(LSR, Absolute, 3));
        m.insert(0x5E, Operation::new(LSR, AbsoluteX, 3));

        m.insert(0xEA, Operation::new(NOP, Implied, 1));

        m.insert(0x09, Operation::new(ORA, Immediate, 2));
        m.insert(0x05, Operation::new(ORA, ZeroPage, 2));
        m.insert(0x15, Operation::new(ORA, ZeroPageX, 2));
        m.insert(0x0D, Operation::new(ORA, Absolute, 3));
        m.insert(0x1D, Operation::new(ORA, AbsoluteX, 3));
        m.insert(0x19, Operation::new(ORA, AbsoluteY, 3));
        m.insert(0x01, Operation::new(ORA, IndirectX, 2));
        m.insert(0x11, Operation::new(ORA, IndirectY, 2));

        m.insert(0x48, Operation::new(PHA, Implied, 1));

        m.insert(0xAA, Operation::new(TAX, Implied, 1));
        m.insert(0xA8, Operation::new(TAY, Implied, 1));
        m
    };
}

pub enum OpName {
    ADC,AND,ASL,
    BCC,BCS,BEQ,BIT,BMI,BNE,BPL,BRK,BVC,BVS,
    CLC,CLD,CLI,CLV,CMP,CPX,CPY,
    DEC,DEX,DEY,
    EOR,
    INC,INX,INY,
    JMP,JSR,
    LDA,LDX,LDY,LSR,
    NOP,
    ORA,
    PHA,PHP,PLA,PLP,
    ROL,ROR,RTI,RTS,
    SBC,SEC,SED,SEI,STA,STX,STY,
    TAX,TAY,TSX,TXA,TXS,TYA,
}

#[derive(PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Implied,
}

pub struct Operation {
    pub mnemonic_name: OpName,
    pub addressing_mode: AddressingMode,
    pub bytes: u8,
}

impl Operation {
    pub fn new(mnemonic_name: OpName, addressing_mode: AddressingMode, bytes: u8) -> Self {
        Operation {
            mnemonic_name: mnemonic_name,
            addressing_mode: addressing_mode,
            bytes: bytes,
        }
    }
}