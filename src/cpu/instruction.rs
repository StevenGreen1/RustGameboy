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
pub enum IncDec16Target
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
    AF,
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
pub enum ADDHLTarget
{
    BC,
    DE,
    HL,
    SP,
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

pub enum RSTLocation
{
    X00,
    X08,
    X10,
    X18,
    X20,
    X28,
    X30,
    X38,
}

impl RSTLocation
{
    pub fn to_hex(&self) -> u16
    {
        match self
        {
            RSTLocation::X00 => 0x00,
            RSTLocation::X08 => 0x08,
            RSTLocation::X10 => 0x10,
            RSTLocation::X18 => 0x18,
            RSTLocation::X20 => 0x20,
            RSTLocation::X28 => 0x28,
            RSTLocation::X30 => 0x30,
            RSTLocation::X38 => 0x38,
        }
    }
}

pub enum Instruction
{
    NOP(),
    HALT(),
    CALL(JumpTest),
    RET(JumpTest),
    RETI(),
    PUSH(StackTarget),
    POP(StackTarget),
    LD(LoadType),
    ADD(ArithmeticTarget),
    ADD16(ArithmeticTarget16),
    ADDD8(),
    ADDHL(ADDHLTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    XOR16(ArithmeticTarget16),
    XORD8(),
    CP(ArithmeticTarget),
    CP16(ArithmeticTarget16),
    CPD8(),
    INC(ArithmeticTarget),
    INC16(IncDec16Target),
    DEC(ArithmeticTarget),
    DEC16(IncDec16Target),
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
    RST(RSTLocation),
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

            0xc5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xd5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xe5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xf5 => Some(Instruction::PUSH(StackTarget::AF)),

            0xc1 => Some(Instruction::POP(StackTarget::BC)),
            0xd1 => Some(Instruction::POP(StackTarget::DE)),
            0xe1 => Some(Instruction::POP(StackTarget::HL)),
            0xf1 => Some(Instruction::POP(StackTarget::AF)),

            0xc4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xd4 => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xcc => Some(Instruction::CALL(JumpTest::Zero)),
            0xdc => Some(Instruction::CALL(JumpTest::Carry)),
            0xcd => Some(Instruction::CALL(JumpTest::Always)),

            0xbf => Some(Instruction::CP(ArithmeticTarget::A)),
            0xb8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xb9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xba => Some(Instruction::CP(ArithmeticTarget::D)),
            0xbb => Some(Instruction::CP(ArithmeticTarget::E)),
            0xbc => Some(Instruction::CP(ArithmeticTarget::H)),
            0xbd => Some(Instruction::CP(ArithmeticTarget::L)),
            0xbe => Some(Instruction::CP16(ArithmeticTarget16::HL)),
            0xfe => Some(Instruction::CPD8()),

            0xf2 => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::LastByteIndirect))),
            0x0a => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::BCIndirect))),
            0x1a => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::DEIndirect))),
            0x2a => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::HLIndirectPlus))),
            0x3a => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::HLIndirectMinus))),
            0xfa => Some(Instruction::LD(LoadType::AFromIndirect(Indirect::WordIndirect))),

            0xc4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xd4 => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xcc => Some(Instruction::CALL(JumpTest::Zero)),
            0xdc => Some(Instruction::CALL(JumpTest::Carry)),
            0xcd => Some(Instruction::CALL(JumpTest::Always)),

            0xc0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xd0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xc8 => Some(Instruction::RET(JumpTest::Zero)),
            0xd8 => Some(Instruction::RET(JumpTest::Carry)),
            0xc9 => Some(Instruction::RET(JumpTest::Always)),
            0xd9 => Some(Instruction::RETI()),

            0xc7 => Some(Instruction::RST(RSTLocation::X00)),
            0xd7 => Some(Instruction::RST(RSTLocation::X10)),
            0xe7 => Some(Instruction::RST(RSTLocation::X20)),
            0xf7 => Some(Instruction::RST(RSTLocation::X30)),
            0xcf => Some(Instruction::RST(RSTLocation::X08)),
            0xdf => Some(Instruction::RST(RSTLocation::X18)),
            0xef => Some(Instruction::RST(RSTLocation::X28)),
            0xff => Some(Instruction::RST(RSTLocation::X38)),

            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD16(ArithmeticTarget16::HL)),
            0xc6 => Some(Instruction::ADDD8()),

            0x09 => Some(Instruction::ADDHL(ADDHLTarget::BC)),
            0x19 => Some(Instruction::ADDHL(ADDHLTarget::DE)),
            0x29 => Some(Instruction::ADDHL(ADDHLTarget::HL)),
            0x39 => Some(Instruction::ADDHL(ADDHLTarget::SP)),

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
            0x34 => Some(Instruction::INC16(IncDec16Target::HL)),
            0x03 => Some(Instruction::INC16(IncDec16Target::BC)),
            0x13 => Some(Instruction::INC16(IncDec16Target::DE)),
            0x23 => Some(Instruction::INC16(IncDec16Target::HL)),
            0x33 => Some(Instruction::INC16(IncDec16Target::SP)),

            0x3d => Some(Instruction::DEC(ArithmeticTarget::A)),
            0x05 => Some(Instruction::DEC(ArithmeticTarget::B)),
            0x0d => Some(Instruction::DEC(ArithmeticTarget::C)),
            0x15 => Some(Instruction::DEC(ArithmeticTarget::D)),
            0x1d => Some(Instruction::DEC(ArithmeticTarget::E)),
            0x25 => Some(Instruction::DEC(ArithmeticTarget::H)),
            0x2d => Some(Instruction::DEC(ArithmeticTarget::L)),
            0x35 => Some(Instruction::DEC16(IncDec16Target::HL)),
            0x0b => Some(Instruction::DEC16(IncDec16Target::BC)),
            0x1b => Some(Instruction::DEC16(IncDec16Target::DE)),
            0x2b => Some(Instruction::DEC16(IncDec16Target::HL)),
            0x3b => Some(Instruction::DEC16(IncDec16Target::SP)),

            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }
}
