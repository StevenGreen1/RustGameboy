#[derive(Copy, Clone)]
pub enum ArithmeticTarget
{
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone)]
pub enum ArithmeticTarget16
{
    BC,
    DE,
    HL,
}

#[derive(Copy, Clone)]
pub enum StackTarget
{
    BC,
    DE,
    HL,
}

#[derive(Copy, Clone)]
pub enum JumpTest
{
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Copy, Clone)]
pub enum LoadByteTarget
{
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[derive(Copy, Clone)]
pub enum LoadByteSource
{
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

#[derive(Copy, Clone)]
pub enum LoadWordTarget
{
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone)]
pub enum Indirect
{
    BCIndirect,
    DEIndirect,
    HLIndirectMinus,
    HLIndirectPlus,
    WordIndirect,
    LastByteIndirect,
}

#[derive(Copy, Clone)]
pub enum LoadType
{
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget), // Source has to be direct input as no registers are big enough to occupy
    AFromIndirect(Indirect),
    IndirectFromA(Indirect),
    AFromByteAddress(),
    ByteAddressFromA(),
}

pub enum Instruction
{
    NOP(),
    HALT(),
    CALL(JumpTest),
    RETURN(JumpTest),
    PUSH(StackTarget),
    POP(StackTarget),
    LD(LoadType),
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget16),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    XOR16(ArithmeticTarget16),
    XORD8(),
    CP(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    CCF(),
    SCF(),
    RRA(),
    RR(ArithmeticTarget), // RR r8
    RLA(),
    RL(ArithmeticTarget), // RL r8
    RRCA(),
    RRC(ArithmeticTarget), // RRC r8
    RLCA(),
    RLC(ArithmeticTarget), // RLC r8
    CPL(),
    BIT(u8, ArithmeticTarget),  // BIT u3,r8
    BIT16(u8),                  // BIT u3,[HL] - Possibly wrong as HL is address
    RES(u8, ArithmeticTarget),  // RES u3,r8
    RES16(u8),                  // RES u3,[HL] - Possibly wrong as HL is address
    SET(u8, ArithmeticTarget),  // SET u3,r8
    SET16(u8),                  // SET u3,[HL] - Possibly wrong as HL is address
    SRL(ArithmeticTarget),      // SRL r8
    SRA(ArithmeticTarget),      // SRA r8
    SLA(ArithmeticTarget),      // SLA r8
    SWAP(ArithmeticTarget),     // SWAP r8
    SWAP16(ArithmeticTarget16), // SWAP [HL]
    JP(JumpTest),               // Jump instructions
}

impl Instruction
{
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction>
    {
        if prefixed
        {
            Instruction::from_byte_prefixed(byte)
        }
        else
        {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction>
    {
        match byte
        {
            0x00 => Some(Instruction::RLC(ArithmeticTarget::B)), // RLC B
            0x30 => Some(Instruction::SWAP(ArithmeticTarget::B)), // RLC B
            0x31 => Some(Instruction::SWAP(ArithmeticTarget::C)), // RLC B
            0x32 => Some(Instruction::SWAP(ArithmeticTarget::D)), // RLC B
            0x33 => Some(Instruction::SWAP(ArithmeticTarget::E)), // RLC B
            0x34 => Some(Instruction::SWAP(ArithmeticTarget::H)), // RLC B
            0x35 => Some(Instruction::SWAP(ArithmeticTarget::L)), // RLC B
            0x36 => Some(Instruction::SWAP16(ArithmeticTarget16::HL)), // RLC B
            0x37 => Some(Instruction::SWAP(ArithmeticTarget::A)), // RLC B

            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>
    {
        match byte
        {
            0x04 => Some(Instruction::INC(ArithmeticTarget::B)), // INC B
            0x14 => Some(Instruction::INC(ArithmeticTarget::D)), // INC D

            0x01 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::BC))),
            0x11 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::DE))),
            0x21 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL))),
            0x31 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP))),

            0xaf => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xa8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xa9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xaa => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xab => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xac => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xad => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xae => Some(Instruction::XOR16(ArithmeticTarget16::HL)),
            0xee => Some(Instruction::XORD8()),

            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }
}
