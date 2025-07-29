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
pub enum Inc16Target
{
    BC,
    DE,
    HL,
    HLI,
    SP,
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
    SPFromHL(),
    HLFromSPN(),
    IndirectFromSP(),
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
    INC16(Inc16Target),
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
    JP(JumpTest),               // Absolute jump instructions
    JR(JumpTest),               // Relative jump instructions
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
        println!("from_byte_prefixed {:x}", byte);
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

            0x78 => Some(Instruction::BIT(7, ArithmeticTarget::B)),
            0x79 => Some(Instruction::BIT(7, ArithmeticTarget::C)),
            0x7a => Some(Instruction::BIT(7, ArithmeticTarget::D)),
            0x7b => Some(Instruction::BIT(7, ArithmeticTarget::E)),
            0x7c => Some(Instruction::BIT(7, ArithmeticTarget::H)),
            0x7d => Some(Instruction::BIT(7, ArithmeticTarget::L)),
            0x7e => Some(Instruction::BIT16(7)), // Only 16 bit target is HL, so no target here
            0x7f => Some(Instruction::BIT(7, ArithmeticTarget::A)),

            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>
    {
        println!("from_byte_not_prefixed");
        match byte
        {
            0x00 => Some(Instruction::NOP()),

            0x01 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::BC))),
            0x11 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::DE))),
            0x21 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL))),
            0x31 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP))),

            0x02 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::BCIndirect))),
            0x12 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::DEIndirect))),
            0x22 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::HLIndirectPlus))),
            0x32 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::HLIndirectMinus))),

            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
            0x0e => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
            0x1e => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
            0x2e => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D8))),
            0x3e => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),

            0x18 => Some(Instruction::JR(JumpTest::Always)),
            0x20 => Some(Instruction::JR(JumpTest::NotZero)),
            0x30 => Some(Instruction::JR(JumpTest::NotCarry)),
            0x28 => Some(Instruction::JR(JumpTest::Zero)),
            0x38 => Some(Instruction::JR(JumpTest::Carry)),

            0xaf => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xa8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xa9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xaa => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xab => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xac => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xad => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xae => Some(Instruction::XOR16(ArithmeticTarget16::HL)),
            0xee => Some(Instruction::XORD8()),

            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::L))),
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A))),

            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
            0x7a => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
            0x7b => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
            0x7c => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
            0x7d => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
            0x7e => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
            0x7f => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),

            0xe0 => Some(Instruction::LD(LoadType::ByteAddressFromA())),
            0xf0 => Some(Instruction::LD(LoadType::AFromByteAddress())),

            0x08 => Some(Instruction::LD(LoadType::IndirectFromSP())),
            0xf9 => Some(Instruction::LD(LoadType::SPFromHL())),
            0xf8 => Some(Instruction::LD(LoadType::HLFromSPN())),

            0xe2 => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::LastByteIndirect))),
            0x02 => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::BCIndirect))),
            0x12 => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::DEIndirect))),
            0x22 => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::HLIndirectPlus))),
            0x32 => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::HLIndirectMinus))),
            0xea => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::WordIndirect))),

            0x3c => Some(Instruction::INC(ArithmeticTarget::A)),
            0x04 => Some(Instruction::INC(ArithmeticTarget::B)),
            0x14 => Some(Instruction::INC(ArithmeticTarget::D)),
            0x24 => Some(Instruction::INC(ArithmeticTarget::H)),
            0x0c => Some(Instruction::INC(ArithmeticTarget::C)),
            0x1c => Some(Instruction::INC(ArithmeticTarget::E)),
            0x2c => Some(Instruction::INC(ArithmeticTarget::L)),
            0x34 => Some(Instruction::INC16(Inc16Target::HL)),
            0x03 => Some(Instruction::INC16(Inc16Target::BC)),
            0x13 => Some(Instruction::INC16(Inc16Target::DE)),
            0x23 => Some(Instruction::INC16(Inc16Target::HL)),
            0x33 => Some(Instruction::INC16(Inc16Target::SP)),

            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }
}
