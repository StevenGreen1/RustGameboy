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

pub enum ArithmeticTarget16
{
    BC,
    DE,
    HL,
}

pub enum StackTarget
{
    BC,
    DE,
    HL,
}

pub enum JumpTest
{
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

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

pub enum LoadWordTarget
{
    AF,
    BC,
    DE,
    HL,
}

pub enum Indirect
{
    BCIndirect,
    DEIndirect,
    HLIndirectMinus,
    HLIndirectPlus,
    WordIndirect,
    LastByteIndirect,
}

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
    BIT(u8, ArithmeticTarget), // BIT u3,r8
    BIT16(u8),                 // BIT u3,[HL] - Possibly wrong as HL is address
    RES(u8, ArithmeticTarget), // RES u3,r8
    RES16(u8),                 // RES u3,[HL] - Possibly wrong as HL is address
    SET(u8, ArithmeticTarget), // SET u3,r8
    SET16(u8),                 // SET u3,[HL] - Possibly wrong as HL is address
    SRL(ArithmeticTarget),     // SRL r8
    SRA(ArithmeticTarget),     // SRA r8
    SLA(ArithmeticTarget),     // SLA r8
    SWAP(ArithmeticTarget),    // SWAP r8
    JP(JumpTest),              // Jump instructions
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
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }
}
