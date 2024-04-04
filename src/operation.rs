#[derive(Copy, Clone)]
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Implied,
}

#[derive(Copy, Clone)]
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