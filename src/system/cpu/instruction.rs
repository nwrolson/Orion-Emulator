use std::fmt;

// Following are definitions used for CPU instructions

pub struct Instruction {
    pub op: Opcode,
    pub instr_type: InstructionType,
    pub cycle_len: u8,
}

#[derive(PartialEq, Debug)]
pub enum Opcode {
    DAA,
    CCF,
    SCF,
    CPL,
    NOP,
    ADD,
    ADC,
    INC,
    SUB,
    SBC,
    DEC,
    AND,
    OR,
    XOR,
    CP,
    JP,
    JR,
    LD,
    PUSH,
    POP,
    CALL,
    RET,
    RETI,
    RST,
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SRL,
    SWAP,
    BIT,
    RES,
    SET,
    DI,
    EI,
}

#[derive(Debug)]
pub enum InstructionType {
    Misc,
    Arithmetic(ArithmeticArg),
    Load(LoadTarget, LoadSource),
    Jump(JumpCond),
    JumpHL,
    Push(RegisterPair),
    Pop(RegisterPair),
    Call(JumpCond),
    Return(JumpCond),
    RST(u8),
    Arithmetic16(RegisterPair),
    Unary16(Word16),
    Load16(Word16),
    LoadMemory16,
    Add16(Word16),
    AddSP,
    Rotate(Word8),
    Bit(Word8, u8)
}

#[derive(Debug)]
pub enum ArithmeticArg {
    A, B, C, D, E, H, L, HL, D8, None
}

#[derive(Debug)]
pub enum Word8 {
    A, B, C, D, E, H, L, HL
}

#[derive(Debug)]
pub enum LoadTarget {
    A, B, C, D, E, H, L, HL, HLI, HLD, BC, DE, A8, A16, CA
}

#[derive(Debug)]
pub enum LoadSource {
    A, B, C, D, E, H, L, HL, HLI, HLD, BC, DE, D8, A8, A16, CA
}

#[derive(Debug)]
pub enum RegisterPair {
    BC, DE, HL, AF
}

#[derive(Debug)]
pub enum Word16 {
    BC, DE, HL, SP
}

#[derive(Debug)]
pub enum JumpCond {
    Zero,
    NotZero,
    Carry,
    NotCarry,
    Always,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode: {:?}\n", self.op);
        write!(f, "Instruction Type: {:?}", self.instr_type)
    }
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Instruction {
        // raw bytes are matched to yield the correctly formatted instruction
        match byte {
            0x00 => Instruction { 
                    op: Opcode::NOP, 
                    instr_type: InstructionType::Misc,
                    cycle_len: 1 
                },
            0x01 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load16(Word16::BC),
                    cycle_len: 3 
                },
            0x02 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::BC, LoadSource::A),
                    cycle_len: 2 
                },
            0x03 => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Unary16(Word16::BC),
                    cycle_len: 2 
                },
            0x04 => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::B),
                    cycle_len: 1 
                },
            0x05 => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::B),
                    cycle_len: 1 
                },
            0x06 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::B, LoadSource::D8),
                    cycle_len: 2 
                },
            0x07 => Instruction { 
                    op: Opcode::RLCA, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 1 
                },
            0x08 => Instruction {
                    op: Opcode::LD,
                    instr_type: InstructionType::LoadMemory16,
                    cycle_len: 5
                },
            0x09 => Instruction {
                    op: Opcode::ADD,
                    instr_type: InstructionType::Add16(Word16::BC),
                    cycle_len: 2
                },
            0x0A => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::BC),
                    cycle_len: 2 
                },
            0x0B => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Unary16(Word16::BC),
                    cycle_len: 2 
                },
            0x0C => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::C),
                    cycle_len: 1 
                },
            0x0D => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::C),
                    cycle_len: 1 
                },
            0x0E => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::C, LoadSource::D8),
                    cycle_len: 2 
                },
            0x0F => Instruction { 
                    op: Opcode::RRCA, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 1 
                },
            0x11 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load16(Word16::DE),
                    cycle_len: 3 
                },
            0x12 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::DE, LoadSource::A),
                    cycle_len: 2 
                },
            0x13 => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Unary16(Word16::DE),
                    cycle_len: 2 
                },
            0x14 => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D),
                    cycle_len: 1 
                },
            0x15 => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D),
                    cycle_len: 1 
                },
            0x16 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::D, LoadSource::D8),
                    cycle_len: 2 
                },
            0x17 => Instruction { 
                    op: Opcode::RLA, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 1 
                },
            0x18 => Instruction {
                    op: Opcode::JR,
                    instr_type: InstructionType::Jump(JumpCond::Always),
                    cycle_len: 3
                },
            0x19 => Instruction {
                    op: Opcode::ADD,
                    instr_type: InstructionType::Add16(Word16::DE),
                    cycle_len: 2
                },
            0x1A => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::DE),
                    cycle_len: 2 
                },
            0x1B => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Unary16(Word16::DE),
                    cycle_len: 2 
                },
            0x1C => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::E),
                    cycle_len: 1 
                },
            0x1D => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::E),
                    cycle_len: 1 
                },
            0x1E => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::E, LoadSource::D8),
                    cycle_len: 2 
                },
            0x1F => Instruction { 
                    op: Opcode::RRA, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 1 
                },
            0x20 => Instruction {
                    op: Opcode::JR,
                    instr_type: InstructionType::Jump(JumpCond::NotZero),
                    cycle_len: 3
                },
            0x21 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load16(Word16::HL),
                    cycle_len: 3 
                },
            0x22 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::HLI, LoadSource::A),
                    cycle_len: 2 
                },
            0x23 => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Unary16(Word16::HL),
                    cycle_len: 2 
                },
            0x24 => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::H),
                    cycle_len: 1 
                },
            0x25 => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::H),
                    cycle_len: 1 
                },
            0x26 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::H, LoadSource::D8),
                    cycle_len: 2 
                },
            0x27 => Instruction { 
                    op: Opcode::DAA, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::None),
                    cycle_len: 1 
                },
            0x28 => Instruction {
                    op: Opcode::JR,
                    instr_type: InstructionType::Jump(JumpCond::Zero),
                    cycle_len: 3
                },
            0x29 => Instruction {
                    op: Opcode::ADD,
                    instr_type: InstructionType::Add16(Word16::HL),
                    cycle_len: 2
                },
            0x2A => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::HLI),
                    cycle_len: 2 
                },
            0x2B => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Unary16(Word16::HL),
                    cycle_len: 2 
                },
            0x2C => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::L),
                    cycle_len: 1 
                },
            0x2D => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::L),
                    cycle_len: 1 
                },
            0x2E => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::L, LoadSource::D8),
                    cycle_len: 2 
                },
            0x2F => Instruction { 
                    op: Opcode::CPL, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::None),
                    cycle_len: 1 
                },
            0x30 => Instruction {
                    op: Opcode::JR,
                    instr_type: InstructionType::Jump(JumpCond::NotCarry),
                    cycle_len: 3
                },
            0x31 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load16(Word16::SP),
                    cycle_len: 3 
                },
            0x32 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::HLD, LoadSource::A),
                    cycle_len: 2 
                },
            0x33 => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Unary16(Word16::SP),
                    cycle_len: 2 
                },
            0x34 => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::HL),
                    cycle_len: 3 
                },
            0x35 => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::HL),
                    cycle_len: 3 
                },
            0x36 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::HL, LoadSource::D8),
                    cycle_len: 3 
                },
            0x37 => Instruction { 
                    op: Opcode::SCF, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::None),
                    cycle_len: 1 
                },
            0x38 => Instruction {
                    op: Opcode::JR,
                    instr_type: InstructionType::Jump(JumpCond::Carry),
                    cycle_len: 3
                },
            0x39 => Instruction {
                    op: Opcode::ADD,
                    instr_type: InstructionType::Add16(Word16::SP),
                    cycle_len: 2
                },
            0x3A => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::HLD),
                    cycle_len: 2 
                },
            0x3B => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Unary16(Word16::SP),
                    cycle_len: 2 
                },
            0x3C => Instruction { 
                    op: Opcode::INC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::A),
                    cycle_len: 1 
                },
            0x3D => Instruction { 
                    op: Opcode::DEC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::A),
                    cycle_len: 1 
                },
            0x3E => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::D8),
                    cycle_len: 2 
                },
            0x3F => Instruction { 
                    op: Opcode::CCF, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::None),
                    cycle_len: 1 
                },
            0x40 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::B, LoadSource::B),
                    cycle_len: 1 
                },
            0x41 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::B, LoadSource::C),
                    cycle_len: 1 
                },
            0x42 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::B, LoadSource::D),
                    cycle_len: 1 
                },
            0x43 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::B, LoadSource::E),
                    cycle_len: 1 
                },
            0x44 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::B, LoadSource::H),
                    cycle_len: 1 
                },
            0x45 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::B, LoadSource::L),
                    cycle_len: 1 
                },
            0x46 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::B, LoadSource::HL),
                    cycle_len: 2 
                },
            0x47 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::B, LoadSource::A),
                    cycle_len: 1 
                },
            0x48 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::C, LoadSource::B),
                    cycle_len: 1 
                },
            0x49 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::C, LoadSource::C),
                    cycle_len: 1 
                },
            0x4A => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::C, LoadSource::D),
                    cycle_len: 1 
                },
            0x4B => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::C, LoadSource::E),
                    cycle_len: 1 
                },
            0x4C => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::C, LoadSource::H),
                    cycle_len: 1 
                },
            0x4D => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::C, LoadSource::L),
                    cycle_len: 1 
                },
            0x4E => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::C, LoadSource::HL),
                    cycle_len: 2 
                },
            0x4F => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::C, LoadSource::A),
                    cycle_len: 1 
                },
            0x50 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::D, LoadSource::B),
                    cycle_len: 1 
                },
            0x51 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::D, LoadSource::C),
                    cycle_len: 1 
                },
            0x52 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::D, LoadSource::D),
                    cycle_len: 1 
                },
            0x53 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::D, LoadSource::E),
                    cycle_len: 1 
                },
            0x54 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::D, LoadSource::H),
                    cycle_len: 1 
                },
            0x55 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::D, LoadSource::L),
                    cycle_len: 1 
                },
            0x56 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::D, LoadSource::HL),
                    cycle_len: 2 
                },
            0x57 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::D, LoadSource::A),
                    cycle_len: 1 
                },
            0x58 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::E, LoadSource::B),
                    cycle_len: 1 
                },
            0x59 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::E, LoadSource::C),
                    cycle_len: 1 
                },
            0x5A => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::E, LoadSource::D),
                    cycle_len: 1 
                },
            0x5B => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::E, LoadSource::E),
                    cycle_len: 1 
                },
            0x5C => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::E, LoadSource::H),
                    cycle_len: 1 
                },
            0x5D => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::E, LoadSource::L),
                    cycle_len: 1 
                },
            0x5E => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::E, LoadSource::HL),
                    cycle_len: 2 
                },
            0x5F => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::E, LoadSource::A),
                    cycle_len: 1 
                },
            0x60 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::H, LoadSource::B),
                    cycle_len: 1 
                },
            0x61 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::H, LoadSource::C),
                    cycle_len: 1 
                },
            0x62 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::H, LoadSource::D),
                    cycle_len: 1 
                },
            0x63 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::H, LoadSource::E),
                    cycle_len: 1 
                },
            0x64 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::H, LoadSource::H),
                    cycle_len: 1 
                },
            0x65 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::H, LoadSource::L),
                    cycle_len: 1 
                },
            0x66 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::H, LoadSource::HL),
                    cycle_len: 2 
                },
            0x67 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::H, LoadSource::A),
                    cycle_len: 1 
                },
            0x68 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::L, LoadSource::B),
                    cycle_len: 1 
                },
            0x69 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::L, LoadSource::C),
                    cycle_len: 1 
                },
            0x6A => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::L, LoadSource::D),
                    cycle_len: 1 
                },
            0x6B => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::L, LoadSource::E),
                    cycle_len: 1 
                },
            0x6C => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::L, LoadSource::H),
                    cycle_len: 1 
                },
            0x6D => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::L, LoadSource::L),
                    cycle_len: 1 
                },
            0x6E => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::L, LoadSource::HL),
                    cycle_len: 2 
                },
            0x6F => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::L, LoadSource::A),
                    cycle_len: 1 
                },
            0x70 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::HL, LoadSource::B),
                    cycle_len: 2 
                },
            0x71 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::HL, LoadSource::C),
                    cycle_len: 2 
                },
            0x72 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::HL, LoadSource::D),
                    cycle_len: 2 
                },
            0x73 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::HL, LoadSource::E),
                    cycle_len: 2 
                },
            0x74 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::HL, LoadSource::H),
                    cycle_len: 2 
                },
            0x75 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::HL, LoadSource::L),
                    cycle_len: 2 
                },
            0x77 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::HL, LoadSource::A),
                    cycle_len: 2 
                },
            0x78 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::B),
                    cycle_len: 1 
                },
            0x79 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::C),
                    cycle_len: 1 
                },
            0x7A => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::D),
                    cycle_len: 1 
                },
            0x7B => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::E),
                    cycle_len: 1 
                },
            0x7C => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::H),
                    cycle_len: 1 
                },
            0x7D => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::L),
                    cycle_len: 1 
                },
            0x7E => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::HL),
                    cycle_len: 2 
                },
            0x7F => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::A),
                    cycle_len: 1 
                },
            0x80 => Instruction { 
                    op: Opcode::ADD, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::B),
                    cycle_len: 1 
                },
            0x81 => Instruction { 
                    op: Opcode::ADD, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::C),
                    cycle_len: 1 
                },
            0x82 => Instruction { 
                    op: Opcode::ADD, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D),
                    cycle_len: 1 
                },
            0x83 => Instruction { 
                    op: Opcode::ADD, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::E),
                    cycle_len: 1 
                },
            0x84 => Instruction { 
                    op: Opcode::ADD, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::H),
                    cycle_len: 1 
                },
            0x85 => Instruction { 
                    op: Opcode::ADD, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::L),
                    cycle_len: 1 
                },
            0x86 => Instruction { 
                    op: Opcode::ADD, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::HL),
                    cycle_len: 2 
                },
            0x87 => Instruction { 
                    op: Opcode::ADD, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::A),
                    cycle_len: 1 
                },
            // ADC
            0x88 => Instruction { 
                    op: Opcode::ADC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::B),
                    cycle_len: 1 
                },
            0x89 => Instruction { 
                    op: Opcode::ADC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::C),
                    cycle_len: 1 
                },
            0x8A => Instruction { 
                    op: Opcode::ADC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D),
                    cycle_len: 1 
                },
            0x8B => Instruction { 
                    op: Opcode::ADC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::E),
                    cycle_len: 1 
                },
            0x8C => Instruction { 
                    op: Opcode::ADC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::H),
                    cycle_len: 1 
                },
            0x8D => Instruction { 
                    op: Opcode::ADC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::L),
                    cycle_len: 1 
                },
            0x8E => Instruction { 
                    op: Opcode::ADC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::HL),
                    cycle_len: 2
                },
            0x8F => Instruction { 
                    op: Opcode::ADC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::A),
                    cycle_len: 1 
                },
            // SUB
            0x90 => Instruction { 
                    op: Opcode::SUB, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::B),
                    cycle_len: 1 
                },
            0x91 => Instruction { 
                    op: Opcode::SUB, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::C),
                    cycle_len: 1 
                },
            0x92 => Instruction { 
                    op: Opcode::SUB, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D),
                    cycle_len: 1 
                },
            0x93 => Instruction { 
                    op: Opcode::SUB, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::E),
                    cycle_len: 1 
                },
            0x94 => Instruction { 
                    op: Opcode::SUB, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::H),
                    cycle_len: 1 
                },
            0x95 => Instruction { 
                    op: Opcode::SUB, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::L),
                    cycle_len: 1 
                },
            0x96 => Instruction { 
                    op: Opcode::SUB, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::HL),
                    cycle_len: 2 
                },
            0x97 => Instruction { 
                    op: Opcode::SUB, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::A),
                    cycle_len: 1 
                },
            // SBC
            0x98 => Instruction { 
                    op: Opcode::SBC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::B),
                    cycle_len: 1 
                },
            0x99 => Instruction { 
                    op: Opcode::SBC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::C),
                    cycle_len: 1 
                },
            0x9A => Instruction { 
                    op: Opcode::SBC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D),
                    cycle_len: 1 
                },
            0x9B => Instruction { 
                    op: Opcode::SBC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::E),
                    cycle_len: 1 
                },
            0x9C => Instruction { 
                    op: Opcode::SBC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::H),
                    cycle_len: 1 
                },
            0x9D => Instruction { 
                    op: Opcode::SBC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::L),
                    cycle_len: 1 
                },
            0x9E => Instruction { 
                    op: Opcode::SBC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::HL),
                    cycle_len: 2
                },
            0x9F => Instruction { 
                    op: Opcode::SBC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::A),
                    cycle_len: 1 
                },
            // AND
            0xA0 => Instruction { 
                    op: Opcode::AND, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::B),
                    cycle_len: 1 
                },
            0xA1 => Instruction { 
                    op: Opcode::AND, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::C),
                    cycle_len: 1 
                },
            0xA2 => Instruction { 
                    op: Opcode::AND, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D),
                    cycle_len: 1 
                },
            0xA3 => Instruction { 
                    op: Opcode::AND, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::E),
                    cycle_len: 1 
                },
            0xA4 => Instruction { 
                    op: Opcode::AND, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::H),
                    cycle_len: 1 
                },
            0xA5 => Instruction { 
                    op: Opcode::AND, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::L),
                    cycle_len: 1 
                },
            0xA6 => Instruction { 
                    op: Opcode::AND, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::HL),
                    cycle_len: 2 
                },
            0xA7 => Instruction { 
                    op: Opcode::AND, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::A),
                    cycle_len: 1 
                },
            // XOR
            0xA8 => Instruction { 
                    op: Opcode::XOR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::B),
                    cycle_len: 1 
                },
            0xA9 => Instruction { 
                    op: Opcode::XOR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::C),
                    cycle_len: 1 
                },
            0xAA => Instruction { 
                    op: Opcode::XOR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D),
                    cycle_len: 1 
                },
            0xAB => Instruction { 
                    op: Opcode::XOR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::E),
                    cycle_len: 1 
                },
            0xAC => Instruction { 
                    op: Opcode::XOR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::H),
                    cycle_len: 1 
                },
            0xAD => Instruction { 
                    op: Opcode::XOR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::L),
                    cycle_len: 1 
                },
            0xAE => Instruction { 
                    op: Opcode::XOR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::HL),
                    cycle_len: 2
                },
            0xAF => Instruction { 
                    op: Opcode::XOR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::A),
                    cycle_len: 1 
                },
            // OR
            0xB0 => Instruction { 
                    op: Opcode::OR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::B),
                    cycle_len: 1 
                },
            0xB1 => Instruction { 
                    op: Opcode::OR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::C),
                    cycle_len: 1 
                },
            0xB2 => Instruction { 
                    op: Opcode::OR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D),
                    cycle_len: 1 
                },
            0xB3 => Instruction { 
                    op: Opcode::OR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::E),
                    cycle_len: 1 
                },
            0xB4 => Instruction { 
                    op: Opcode::OR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::H),
                    cycle_len: 1 
                },
            0xB5 => Instruction { 
                    op: Opcode::OR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::L),
                    cycle_len: 1 
                },
            0xB6 => Instruction { 
                    op: Opcode::OR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::HL),
                    cycle_len: 2 
                },
            0xB7 => Instruction { 
                    op: Opcode::OR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::A),
                    cycle_len: 1 
                },
            // CP
            0xB8 => Instruction { 
                    op: Opcode::CP, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::B),
                    cycle_len: 1 
                },
            0xB9 => Instruction { 
                    op: Opcode::CP, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::C),
                    cycle_len: 1 
                },
            0xBA => Instruction { 
                    op: Opcode::CP, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D),
                    cycle_len: 1 
                },
            0xBB => Instruction { 
                    op: Opcode::CP, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::E),
                    cycle_len: 1 
                },
            0xBC => Instruction { 
                    op: Opcode::CP, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::H),
                    cycle_len: 1 
                },
            0xBD => Instruction { 
                    op: Opcode::CP, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::L),
                    cycle_len: 1 
                },
            0xBE => Instruction { 
                    op: Opcode::CP, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::HL),
                    cycle_len: 2
                },
            0xBF => Instruction { 
                    op: Opcode::CP, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::A),
                    cycle_len: 1 
                },
            0xC0 => Instruction { 
                    op: Opcode::RET, 
                    instr_type: InstructionType::Return(JumpCond::NotZero),
                    cycle_len: 5 
                },
            0xC1 => Instruction { 
                    op: Opcode::POP, 
                    instr_type: InstructionType::Pop(RegisterPair::BC),
                    cycle_len: 3 
                },
            0xC2 => Instruction { 
                    op: Opcode::JP, 
                    instr_type: InstructionType::Jump(JumpCond::NotZero),
                    cycle_len: 4 
                },
            0xC3 => Instruction { 
                    op: Opcode::JP, 
                    instr_type: InstructionType::Jump(JumpCond::Always),
                    cycle_len: 4 
                },
            0xC4 => Instruction { 
                    op: Opcode::CALL, 
                    instr_type: InstructionType::Call(JumpCond::NotZero),
                    cycle_len: 6 
                },
            0xC5 => Instruction { 
                    op: Opcode::PUSH, 
                    instr_type: InstructionType::Push(RegisterPair::BC),
                    cycle_len: 4 
                },
            0xC6 => Instruction { 
                    op: Opcode::ADD, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D8),
                    cycle_len: 2 
                },
            0xC7 => Instruction {
                    op: Opcode::RST,
                    instr_type: InstructionType::RST(0x00),
                    cycle_len: 4
                },
            0xC8 => Instruction { 
                    op: Opcode::RET, 
                    instr_type: InstructionType::Return(JumpCond::Zero),
                    cycle_len: 5 
                },
            0xC9 => Instruction { 
                    op: Opcode::RET, 
                    instr_type: InstructionType::Return(JumpCond::Always),
                    cycle_len: 4 
                },
            0xCA => Instruction { 
                    op: Opcode::JP, 
                    instr_type: InstructionType::Jump(JumpCond::Zero),
                    cycle_len: 4 
                },
            // 0xCB reserved for denoting prefix instructions
            0xCC => Instruction { 
                    op: Opcode::CALL, 
                    instr_type: InstructionType::Call(JumpCond::Zero),
                    cycle_len: 6 
                },
            0xCD => Instruction { 
                    op: Opcode::CALL, 
                    instr_type: InstructionType::Call(JumpCond::Always),
                    cycle_len: 6 
                },
            0xCE => Instruction { 
                    op: Opcode::ADC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D8),
                    cycle_len: 2 
                },
            0xCF => Instruction {
                    op: Opcode::RST,
                    instr_type: InstructionType::RST(0x08),
                    cycle_len: 4
                },
            0xD0 => Instruction { 
                    op: Opcode::RET, 
                    instr_type: InstructionType::Return(JumpCond::NotCarry),
                    cycle_len: 5 
                },
            0xD1 => Instruction { 
                    op: Opcode::POP, 
                    instr_type: InstructionType::Pop(RegisterPair::DE),
                    cycle_len: 3 
                },
            0xD2 => Instruction { 
                    op: Opcode::JP, 
                    instr_type: InstructionType::Jump(JumpCond::NotCarry),
                    cycle_len: 4 
                },
            0xD4 => Instruction { 
                    op: Opcode::CALL, 
                    instr_type: InstructionType::Call(JumpCond::NotCarry),
                    cycle_len: 6 
                },
            0xD5 => Instruction { 
                    op: Opcode::PUSH, 
                    instr_type: InstructionType::Push(RegisterPair::DE),
                    cycle_len: 4 
                },
            0xD6 => Instruction { 
                    op: Opcode::SUB, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D8),
                    cycle_len: 2 
                },
            0xD7 => Instruction {
                    op: Opcode::RST,
                    instr_type: InstructionType::RST(0x10),
                    cycle_len: 4
                },
            0xD8 => Instruction { 
                    op: Opcode::RET, 
                    instr_type: InstructionType::Return(JumpCond::Carry),
                    cycle_len: 5 
                },
            0xD9 => Instruction { 
                    op: Opcode::RETI, 
                    instr_type: InstructionType::Return(JumpCond::NotZero),
                    cycle_len: 4 
                },
            0xDA => Instruction { 
                    op: Opcode::JP, 
                    instr_type: InstructionType::Jump(JumpCond::Carry),
                    cycle_len: 4 
                },
            0xDC => Instruction { 
                    op: Opcode::CALL, 
                    instr_type: InstructionType::Call(JumpCond::Carry),
                    cycle_len: 6 
                },
            0xDE => Instruction { 
                    op: Opcode::SBC, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D8),
                    cycle_len: 2 
                },
            0xDF => Instruction {
                    op: Opcode::RST,
                    instr_type: InstructionType::RST(0x18),
                    cycle_len: 4
                },
            0xE0 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A8, LoadSource::A),
                    cycle_len: 3 
                },
            0xE1 => Instruction { 
                    op: Opcode::POP, 
                    instr_type: InstructionType::Pop(RegisterPair::HL),
                    cycle_len: 3 
                },
            0xE2 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::CA, LoadSource::A),
                    cycle_len: 2 
                },
            0xE5 => Instruction { 
                    op: Opcode::PUSH, 
                    instr_type: InstructionType::Push(RegisterPair::HL),
                    cycle_len: 4 
                },
            0xE6 => Instruction { 
                    op: Opcode::AND, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D8),
                    cycle_len: 2 
                },
            0xE7 => Instruction {
                    op: Opcode::RST,
                    instr_type: InstructionType::RST(0x20),
                    cycle_len: 4
                },
            0xE8 => Instruction {
                    op: Opcode::ADD,
                    instr_type: InstructionType::AddSP,
                    cycle_len: 4
                },
            0xE9 => Instruction {
                    op: Opcode::JP,
                    instr_type: InstructionType::JumpHL,
                    cycle_len: 1
                }, 
            0xEA => Instruction {
                    op: Opcode::LD,
                    instr_type: InstructionType::Load(LoadTarget::A16, LoadSource::A),
                    cycle_len: 4
                },
            0xEE => Instruction { 
                    op: Opcode::XOR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D8),
                    cycle_len: 2 
                },
            0xEF => Instruction {
                    op: Opcode::RST,
                    instr_type: InstructionType::RST(0x28),
                    cycle_len: 4
                },
            0xF0 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::A8),
                    cycle_len: 3 
                },
            0xF1 => Instruction { 
                    op: Opcode::POP, 
                    instr_type: InstructionType::Pop(RegisterPair::AF),
                    cycle_len: 3 
                },
            0xF2 => Instruction { 
                    op: Opcode::LD, 
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::CA),
                    cycle_len: 2 
                },
            0xF3 => Instruction {
                    op: Opcode::DI,
                    instr_type: InstructionType::Misc,
                    cycle_len: 1
                },
            0xF5 => Instruction { 
                    op: Opcode::PUSH, 
                    instr_type: InstructionType::Push(RegisterPair::AF),
                    cycle_len: 4 
                },
            0xF6 => Instruction { 
                    op: Opcode::OR, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D8),
                    cycle_len: 2 
                },
            0xF7 => Instruction {
                    op: Opcode::RST,
                    instr_type: InstructionType::RST(0x30),
                    cycle_len: 4
                },
            0xFA => Instruction {
                    op: Opcode::LD,
                    instr_type: InstructionType::Load(LoadTarget::A, LoadSource::A16),
                    cycle_len: 4
                },
            0xFB => Instruction {
                    op: Opcode::EI,
                    instr_type: InstructionType::Misc,
                    cycle_len: 1
                },
            0xFE => Instruction { 
                    op: Opcode::CP, 
                    instr_type: InstructionType::Arithmetic(ArithmeticArg::D8),
                    cycle_len: 2 
                },
            0xFF => Instruction {
                    op: Opcode::RST,
                    instr_type: InstructionType::RST(0x38),
                    cycle_len: 4
                },
             _ => Instruction { 
                    op: Opcode::NOP, 
                    instr_type: InstructionType::Misc,
                    cycle_len: 1 
                },
        }
    }

// ==================================*
// Prefix Instructions:
// ==================================*

    pub fn from_byte_prefix(byte: u8) -> Instruction {
        // called instead of normal from_byte if instruction is prefixed
        // with 0xCB
        match byte {
            0x00 => Instruction { 
                    op: Opcode::RLC, 
                    instr_type: InstructionType::Rotate(Word8::B),
                    cycle_len: 2 
                },
            0x01 => Instruction { 
                    op: Opcode::RLC, 
                    instr_type: InstructionType::Rotate(Word8::C),
                    cycle_len: 2 
                },
            0x02 => Instruction { 
                    op: Opcode::RLC, 
                    instr_type: InstructionType::Rotate(Word8::D),
                    cycle_len: 2 
                },
            0x03 => Instruction { 
                    op: Opcode::RLC, 
                    instr_type: InstructionType::Rotate(Word8::E),
                    cycle_len: 2 
                },
            0x04 => Instruction { 
                    op: Opcode::RLC, 
                    instr_type: InstructionType::Rotate(Word8::H),
                    cycle_len: 2 
                },
            0x05 => Instruction { 
                    op: Opcode::RLC, 
                    instr_type: InstructionType::Rotate(Word8::L),
                    cycle_len: 2 
                },
            0x06 => Instruction { 
                    op: Opcode::RLC, 
                    instr_type: InstructionType::Rotate(Word8::HL),
                    cycle_len: 4 
                },
            0x07 => Instruction { 
                    op: Opcode::RLC, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 2 
                },
            0x08 => Instruction { 
                    op: Opcode::RRC, 
                    instr_type: InstructionType::Rotate(Word8::B),
                    cycle_len: 2 
                },
            0x09 => Instruction { 
                    op: Opcode::RRC, 
                    instr_type: InstructionType::Rotate(Word8::C),
                    cycle_len: 2 
                },
            0x0A => Instruction { 
                    op: Opcode::RRC, 
                    instr_type: InstructionType::Rotate(Word8::D),
                    cycle_len: 2 
                },
            0x0B => Instruction { 
                    op: Opcode::RRC, 
                    instr_type: InstructionType::Rotate(Word8::E),
                    cycle_len: 2 
                },
            0x0C => Instruction { 
                    op: Opcode::RRC, 
                    instr_type: InstructionType::Rotate(Word8::H),
                    cycle_len: 2 
                },
            0x0D => Instruction { 
                    op: Opcode::RRC, 
                    instr_type: InstructionType::Rotate(Word8::L),
                    cycle_len: 2 
                },
            0x0E => Instruction { 
                    op: Opcode::RRC, 
                    instr_type: InstructionType::Rotate(Word8::HL),
                    cycle_len: 4 
                },
            0x0F => Instruction { 
                    op: Opcode::RRC, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 2 
                },
            0x10 => Instruction { 
                    op: Opcode::RL, 
                    instr_type: InstructionType::Rotate(Word8::B),
                    cycle_len: 2 
                },
            0x11 => Instruction { 
                    op: Opcode::RL, 
                    instr_type: InstructionType::Rotate(Word8::C),
                    cycle_len: 2 
                },
            0x12 => Instruction { 
                    op: Opcode::RL, 
                    instr_type: InstructionType::Rotate(Word8::D),
                    cycle_len: 2 
                },
            0x13 => Instruction { 
                    op: Opcode::RL, 
                    instr_type: InstructionType::Rotate(Word8::E),
                    cycle_len: 2 
                },
            0x14 => Instruction { 
                    op: Opcode::RL, 
                    instr_type: InstructionType::Rotate(Word8::H),
                    cycle_len: 2 
                },
            0x15 => Instruction { 
                    op: Opcode::RL, 
                    instr_type: InstructionType::Rotate(Word8::L),
                    cycle_len: 2 
                },
            0x16 => Instruction { 
                    op: Opcode::RL, 
                    instr_type: InstructionType::Rotate(Word8::HL),
                    cycle_len: 4 
                },
            0x17 => Instruction { 
                    op: Opcode::RL, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 2 
                },
            0x18 => Instruction { 
                    op: Opcode::RR, 
                    instr_type: InstructionType::Rotate(Word8::B),
                    cycle_len: 2 
                },
            0x19 => Instruction { 
                    op: Opcode::RR, 
                    instr_type: InstructionType::Rotate(Word8::C),
                    cycle_len: 2 
                },
            0x1A => Instruction { 
                    op: Opcode::RR, 
                    instr_type: InstructionType::Rotate(Word8::D),
                    cycle_len: 2 
                },
            0x1B => Instruction { 
                    op: Opcode::RR, 
                    instr_type: InstructionType::Rotate(Word8::E),
                    cycle_len: 2 
                },
            0x1C => Instruction { 
                    op: Opcode::RR, 
                    instr_type: InstructionType::Rotate(Word8::H),
                    cycle_len: 2 
                },
            0x1D => Instruction { 
                    op: Opcode::RR, 
                    instr_type: InstructionType::Rotate(Word8::L),
                    cycle_len: 2 
                },
            0x1E => Instruction { 
                    op: Opcode::RR, 
                    instr_type: InstructionType::Rotate(Word8::HL),
                    cycle_len: 4
                },
            0x1F => Instruction { 
                    op: Opcode::RR, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 2 
                },
            0x20 => Instruction { 
                    op: Opcode::SLA, 
                    instr_type: InstructionType::Rotate(Word8::B),
                    cycle_len: 2 
                },
            0x21 => Instruction { 
                    op: Opcode::SLA, 
                    instr_type: InstructionType::Rotate(Word8::C),
                    cycle_len: 2 
                },
            0x22 => Instruction { 
                    op: Opcode::SLA, 
                    instr_type: InstructionType::Rotate(Word8::D),
                    cycle_len: 2 
                },
            0x23 => Instruction { 
                    op: Opcode::SLA, 
                    instr_type: InstructionType::Rotate(Word8::E),
                    cycle_len: 2 
                },
            0x24 => Instruction { 
                    op: Opcode::SLA, 
                    instr_type: InstructionType::Rotate(Word8::H),
                    cycle_len: 2 
                },
            0x25 => Instruction { 
                    op: Opcode::SLA, 
                    instr_type: InstructionType::Rotate(Word8::L),
                    cycle_len: 2 
                },
            0x26 => Instruction { 
                    op: Opcode::SLA, 
                    instr_type: InstructionType::Rotate(Word8::HL),
                    cycle_len: 4 
                },
            0x27 => Instruction { 
                    op: Opcode::SLA, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 2 
                },
            0x28 => Instruction { 
                    op: Opcode::SRA, 
                    instr_type: InstructionType::Rotate(Word8::B),
                    cycle_len: 2 
                },
            0x29 => Instruction { 
                    op: Opcode::SRA, 
                    instr_type: InstructionType::Rotate(Word8::C),
                    cycle_len: 2 
                },
            0x2A => Instruction { 
                    op: Opcode::SRA, 
                    instr_type: InstructionType::Rotate(Word8::D),
                    cycle_len: 2 
                },
            0x2B => Instruction { 
                    op: Opcode::SRA, 
                    instr_type: InstructionType::Rotate(Word8::E),
                    cycle_len: 2 
                },
            0x2C => Instruction { 
                    op: Opcode::SRA, 
                    instr_type: InstructionType::Rotate(Word8::H),
                    cycle_len: 2 
                },
            0x2D => Instruction { 
                    op: Opcode::SRA, 
                    instr_type: InstructionType::Rotate(Word8::L),
                    cycle_len: 2 
                },
            0x2E => Instruction { 
                    op: Opcode::SRA, 
                    instr_type: InstructionType::Rotate(Word8::HL),
                    cycle_len: 4 
                },
            0x2F => Instruction { 
                    op: Opcode::SRA, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 2 
                },
            0x30 => Instruction { 
                    op: Opcode::SWAP, 
                    instr_type: InstructionType::Rotate(Word8::B),
                    cycle_len: 2 
                },
            0x31 => Instruction { 
                    op: Opcode::SWAP, 
                    instr_type: InstructionType::Rotate(Word8::C),
                    cycle_len: 2 
                },
            0x32 => Instruction { 
                    op: Opcode::SWAP, 
                    instr_type: InstructionType::Rotate(Word8::D),
                    cycle_len: 2 
                },
            0x33 => Instruction { 
                    op: Opcode::SWAP, 
                    instr_type: InstructionType::Rotate(Word8::E),
                    cycle_len: 2 
                },
            0x34 => Instruction { 
                    op: Opcode::SWAP, 
                    instr_type: InstructionType::Rotate(Word8::H),
                    cycle_len: 2 
                },
            0x35 => Instruction { 
                    op: Opcode::SWAP, 
                    instr_type: InstructionType::Rotate(Word8::L),
                    cycle_len: 2 
                },
            0x36 => Instruction { 
                    op: Opcode::SWAP, 
                    instr_type: InstructionType::Rotate(Word8::HL),
                    cycle_len: 4 
                },
            0x37 => Instruction { 
                    op: Opcode::SWAP, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 2 
                },
            0x38 => Instruction { 
                    op: Opcode::SRL, 
                    instr_type: InstructionType::Rotate(Word8::B),
                    cycle_len: 2 
                },
            0x39 => Instruction { 
                    op: Opcode::SRL, 
                    instr_type: InstructionType::Rotate(Word8::C),
                    cycle_len: 2 
                },
            0x3A => Instruction { 
                    op: Opcode::SRL, 
                    instr_type: InstructionType::Rotate(Word8::D),
                    cycle_len: 2 
                },
            0x3B => Instruction { 
                    op: Opcode::SRL, 
                    instr_type: InstructionType::Rotate(Word8::E),
                    cycle_len: 2 
                },
            0x3C => Instruction { 
                    op: Opcode::SRL, 
                    instr_type: InstructionType::Rotate(Word8::H),
                    cycle_len: 2 
                },
            0x3D => Instruction { 
                    op: Opcode::SRL, 
                    instr_type: InstructionType::Rotate(Word8::L),
                    cycle_len: 2 
                },
            0x3E => Instruction { 
                    op: Opcode::SRL, 
                    instr_type: InstructionType::Rotate(Word8::HL),
                    cycle_len: 4 
                },
            0x3F => Instruction { 
                    op: Opcode::SRL, 
                    instr_type: InstructionType::Rotate(Word8::A),
                    cycle_len: 2 
                },
            0x40 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::B, 0),
                    cycle_len: 2 
                },
            0x41 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::C, 0),
                    cycle_len: 2 
                },
            0x42 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::D, 0),
                    cycle_len: 2 
                },
            0x43 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::E, 0),
                    cycle_len: 2 
                },
            0x44 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::H, 0),
                    cycle_len: 2 
                },
            0x45 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::L, 0),
                    cycle_len: 2 
                },
            0x46 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::HL, 0),
                    cycle_len: 4 
                },
            0x47 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::A, 0),
                    cycle_len: 2 
                },
            0x48 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::B, 1),
                    cycle_len: 2 
                },
            0x49 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::C, 1),
                    cycle_len: 2 
                },
            0x4A => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::D, 1),
                    cycle_len: 2 
                },
            0x4B => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::E, 1),
                    cycle_len: 2 
                },
            0x4C => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::H, 1),
                    cycle_len: 2 
                },
            0x4D => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::L, 1),
                    cycle_len: 2 
                },
            0x4E => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::HL, 1),
                    cycle_len: 4 
                },
            0x4F => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::A, 1),
                    cycle_len: 2 
                },
            0x50 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::B, 2),
                    cycle_len: 2 
                },
            0x51 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::C, 2),
                    cycle_len: 2 
                },
            0x52 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::D, 2),
                    cycle_len: 2 
                },
            0x53 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::E, 2),
                    cycle_len: 2 
                },
            0x54 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::H, 2),
                    cycle_len: 2 
                },
            0x55 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::L, 2),
                    cycle_len: 2 
                },
            0x56 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::HL, 2),
                    cycle_len: 4 
                },
            0x57 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::A, 2),
                    cycle_len: 2 
                },
            0x58 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::B, 3),
                    cycle_len: 2 
                },
            0x59 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::C, 3),
                    cycle_len: 2 
                },
            0x5A => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::D, 3),
                    cycle_len: 2 
                },
            0x5B => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::E, 3),
                    cycle_len: 2 
                },
            0x5C => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::H, 3),
                    cycle_len: 2 
                },
            0x5D => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::L, 3),
                    cycle_len: 2 
                },
            0x5E => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::HL, 3),
                    cycle_len: 4 
                },
            0x5F => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::A, 3),
                    cycle_len: 2 
                },
            0x60 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::B, 4),
                    cycle_len: 2 
                },
            0x61 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::C, 4),
                    cycle_len: 2 
                },
            0x62 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::D, 4),
                    cycle_len: 2 
                },
            0x63 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::E, 4),
                    cycle_len: 2 
                },
            0x64 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::H, 4),
                    cycle_len: 2 
                },
            0x65 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::L, 4),
                    cycle_len: 2 
                },
            0x66 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::HL, 4),
                    cycle_len: 4 
                },
            0x67 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::A, 4),
                    cycle_len: 2 
                },
            0x68 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::B, 5),
                    cycle_len: 2 
                },
            0x69 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::C, 5),
                    cycle_len: 2 
                },
            0x6A => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::D, 5),
                    cycle_len: 2 
                },
            0x6B => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::E, 5),
                    cycle_len: 2 
                },
            0x6C => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::H, 5),
                    cycle_len: 2 
                },
            0x6D => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::L, 5),
                    cycle_len: 2 
                },
            0x6E => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::HL, 5),
                    cycle_len: 4 
                },
            0x6F => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::A, 5),
                    cycle_len: 2 
                },
            0x70 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::B, 6),
                    cycle_len: 2 
                },
            0x71 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::C, 6),
                    cycle_len: 2 
                },
            0x72 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::D, 6),
                    cycle_len: 2 
                },
            0x73 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::E, 6),
                    cycle_len: 2 
                },
            0x74 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::H, 6),
                    cycle_len: 2 
                },
            0x75 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::L, 6),
                    cycle_len: 2 
                },
            0x76 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::HL, 6),
                    cycle_len: 4 
                },
            0x77 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::A, 6),
                    cycle_len: 2 
                },
            0x78 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::B, 7),
                    cycle_len: 2 
                },
            0x79 => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::C, 7),
                    cycle_len: 2 
                },
            0x7A => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::D, 7),
                    cycle_len: 2 
                },
            0x7B => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::E, 7),
                    cycle_len: 2 
                },
            0x7C => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::H, 7),
                    cycle_len: 2 
                },
            0x7D => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::L, 7),
                    cycle_len: 2 
                },
            0x7E => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::HL, 7),
                    cycle_len: 4 
                },
            0x7F => Instruction { 
                    op: Opcode::BIT, 
                    instr_type: InstructionType::Bit(Word8::A, 7),
                    cycle_len: 2 
                },
            0x80 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::B, 0),
                    cycle_len: 2 
                },
            0x81 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::C, 0),
                    cycle_len: 2 
                },
            0x82 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::D, 0),
                    cycle_len: 2 
                },
            0x83 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::E, 0),
                    cycle_len: 2 
                },
            0x84 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::H, 0),
                    cycle_len: 2 
                },
            0x85 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::L, 0),
                    cycle_len: 2 
                },
            0x86 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::HL, 0),
                    cycle_len: 4 
                },
            0x87 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::A, 0),
                    cycle_len: 2 
                },
            0x88 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::B, 1),
                    cycle_len: 2 
                },
            0x89 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::C, 1),
                    cycle_len: 2 
                },
            0x8A => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::D, 1),
                    cycle_len: 2 
                },
            0x8B => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::E, 1),
                    cycle_len: 2 
                },
            0x8C => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::H, 1),
                    cycle_len: 2 
                },
            0x8D => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::L, 1),
                    cycle_len: 2 
                },
            0x8E => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::HL, 1),
                    cycle_len: 4 
                },
            0x8F => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::A, 1),
                    cycle_len: 2 
                },
            0x90 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::B, 2),
                    cycle_len: 2 
                },
            0x91 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::C, 2),
                    cycle_len: 2 
                },
            0x92 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::D, 2),
                    cycle_len: 2 
                },
            0x93 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::E, 2),
                    cycle_len: 2 
                },
            0x94 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::H, 2),
                    cycle_len: 2 
                },
            0x95 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::L, 2),
                    cycle_len: 2 
                },
            0x96 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::HL, 2),
                    cycle_len: 4 
                },
            0x97 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::A, 2),
                    cycle_len: 2 
                },
            0x98 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::B, 3),
                    cycle_len: 2 
                },
            0x99 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::C, 3),
                    cycle_len: 2 
                },
            0x9A => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::D, 3),
                    cycle_len: 2 
                },
            0x9B => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::E, 3),
                    cycle_len: 2 
                },
            0x9C => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::H, 3),
                    cycle_len: 2 
                },
            0x9D => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::L, 3),
                    cycle_len: 2 
                },
            0x9E => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::HL, 3),
                    cycle_len: 4 
                },
            0x9F => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::A, 3),
                    cycle_len: 2 
                },
            0xA0 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::B, 4),
                    cycle_len: 2 
                },
            0xA1 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::C, 4),
                    cycle_len: 2 
                },
            0xA2 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::D, 4),
                    cycle_len: 2 
                },
            0xA3 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::E, 4),
                    cycle_len: 2 
                },
            0xA4 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::H, 4),
                    cycle_len: 2 
                },
            0xA5 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::L, 4),
                    cycle_len: 2 
                },
            0xA6 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::HL, 4),
                    cycle_len: 4 
                },
            0xA7 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::A, 4),
                    cycle_len: 2 
                },
            0xA8 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::B, 5),
                    cycle_len: 2 
                },
            0xA9 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::C, 5),
                    cycle_len: 2 
                },
            0xAA => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::D, 5),
                    cycle_len: 2 
                },
            0xAB => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::E, 5),
                    cycle_len: 2 
                },
            0xAC => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::H, 5),
                    cycle_len: 2 
                },
            0xAD => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::L, 5),
                    cycle_len: 2 
                },
            0xAE => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::HL, 5),
                    cycle_len: 4 
                },
            0xAF => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::A, 5),
                    cycle_len: 2 
                },
            0xB0 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::B, 6),
                    cycle_len: 2 
                },
            0xB1 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::C, 6),
                    cycle_len: 2 
                },
            0xB2 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::D, 6),
                    cycle_len: 2 
                },
            0xB3 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::E, 6),
                    cycle_len: 2 
                },
            0xB4 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::H, 6),
                    cycle_len: 2 
                },
            0xB5 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::L, 6),
                    cycle_len: 2 
                },
            0xB6 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::HL, 6),
                    cycle_len: 4 
                },
            0xB7 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::A, 6),
                    cycle_len: 2 
                },
            0xB8 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::B, 7),
                    cycle_len: 2 
                },
            0xB9 => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::C, 7),
                    cycle_len: 2 
                },
            0xBA => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::D, 7),
                    cycle_len: 2 
                },
            0xBB => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::E, 7),
                    cycle_len: 2 
                },
            0xBC => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::H, 7),
                    cycle_len: 2 
                },
            0xBD => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::L, 7),
                    cycle_len: 2 
                },
            0xBE => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::HL, 7),
                    cycle_len: 4 
                },
            0xBF => Instruction { 
                    op: Opcode::RES, 
                    instr_type: InstructionType::Bit(Word8::A, 7),
                    cycle_len: 2 
                },
            0xC0 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::B, 0),
                    cycle_len: 2 
                },
            0xC1 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::C, 0),
                    cycle_len: 2 
                },
            0xC2 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::D, 0),
                    cycle_len: 2 
                },
            0xC3 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::E, 0),
                    cycle_len: 2 
                },
            0xC4 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::H, 0),
                    cycle_len: 2 
                },
            0xC5 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::L, 0),
                    cycle_len: 2 
                },
            0xC6 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::HL, 0),
                    cycle_len: 4 
                },
            0xC7 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::A, 0),
                    cycle_len: 2 
                },
            0xC8 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::B, 1),
                    cycle_len: 2 
                },
            0xC9 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::C, 1),
                    cycle_len: 2 
                },
            0xCA => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::D, 1),
                    cycle_len: 2 
                },
            0xCB => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::E, 1),
                    cycle_len: 2 
                },
            0xCC => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::H, 1),
                    cycle_len: 2 
                },
            0xCD => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::L, 1),
                    cycle_len: 2 
                },
            0xCE => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::HL, 1),
                    cycle_len: 4 
                },
            0xCF => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::A, 1),
                    cycle_len: 2 
                },
            0xD0 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::B, 2),
                    cycle_len: 2 
                },
            0xD1 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::C, 2),
                    cycle_len: 2 
                },
            0xD2 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::D, 2),
                    cycle_len: 2 
                },
            0xD3 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::E, 2),
                    cycle_len: 2 
                },
            0xD4 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::H, 2),
                    cycle_len: 2 
                },
            0xD5 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::L, 2),
                    cycle_len: 2 
                },
            0xD6 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::HL, 2),
                    cycle_len: 4 
                },
            0xD7 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::A, 2),
                    cycle_len: 2 
                },
            0xD8 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::B, 3),
                    cycle_len: 2 
                },
            0xD9 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::C, 3),
                    cycle_len: 2 
                },
            0xDA => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::D, 3),
                    cycle_len: 2 
                },
            0xDB => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::E, 3),
                    cycle_len: 2 
                },
            0xDC => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::H, 3),
                    cycle_len: 2 
                },
            0xDD => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::L, 3),
                    cycle_len: 2 
                },
            0xDE => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::HL, 3),
                    cycle_len: 4
                },
            0xDF => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::A, 3),
                    cycle_len: 2 
                },
            0xE0 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::B, 4),
                    cycle_len: 2 
                },
            0xE1 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::C, 4),
                    cycle_len: 2 
                },
            0xE2 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::D, 4),
                    cycle_len: 2 
                },
            0xE3 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::E, 4),
                    cycle_len: 2 
                },
            0xE4 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::H, 4),
                    cycle_len: 2 
                },
            0xE5 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::L, 4),
                    cycle_len: 2 
                },
            0xE6 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::HL, 4),
                    cycle_len: 4 
                },
            0xE7 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::A, 4),
                    cycle_len: 2 
                },
            0xE8 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::B, 5),
                    cycle_len: 2 
                },
            0xE9 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::C, 5),
                    cycle_len: 2 
                },
            0xEA => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::D, 5),
                    cycle_len: 2 
                },
            0xEB => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::E, 5),
                    cycle_len: 2 
                },
            0xEC => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::H, 5),
                    cycle_len: 2 
                },
            0xED => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::L, 5),
                    cycle_len: 2 
                },
            0xEE => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::HL, 5),
                    cycle_len: 4 
                },
            0xEF => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::A, 5),
                    cycle_len: 2 
                },
            0xF0 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::B, 6),
                    cycle_len: 2 
                },
            0xF1 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::C, 6),
                    cycle_len: 2 
                },
            0xF2 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::D, 6),
                    cycle_len: 2 
                },
            0xF3 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::E, 6),
                    cycle_len: 2 
                },
            0xF4 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::H, 6),
                    cycle_len: 2 
                },
            0xF5 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::L, 6),
                    cycle_len: 2 
                },
            0xF6 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::HL, 6),
                    cycle_len: 4
                },
            0xF7 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::A, 6),
                    cycle_len: 2 
                },
            0xF8 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::B, 7),
                    cycle_len: 2 
                },
            0xF9 => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::C, 7),
                    cycle_len: 2 
                },
            0xFA => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::D, 7),
                    cycle_len: 2 
                },
            0xFB => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::E, 7),
                    cycle_len: 2 
                },
            0xFC => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::H, 7),
                    cycle_len: 2 
                },
            0xFD => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::L, 7),
                    cycle_len: 2 
                },
            0xFE => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::HL, 7),
                    cycle_len: 4 
                },
            0xFF => Instruction { 
                    op: Opcode::SET, 
                    instr_type: InstructionType::Bit(Word8::A, 7),
                    cycle_len: 2 
                },
             _ => Instruction { 
                    op: Opcode::NOP, 
                    instr_type: InstructionType::Misc,
                    cycle_len: 1 
                },
        }
    }
}
// *---------------------*