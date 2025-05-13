mod registers;
use cpu::registers::Registers;

enum Instruction
{
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
    RR(ArithmeticTarget),           // RR r8
    RLA(),
    RL(ArithmeticTarget),           // RL r8
    RRCA(),
    RRC(ArithmeticTarget),          // RRC r8
    RLCA(),
    RLC(ArithmeticTarget),          // RLC r8
    CPL(),
    BIT(u8, ArithmeticTarget),      // BIT u3,r8
    BIT16(u8),                      // BIT u3,[HL] - Possibly wrong as HL is address
    RES(u8, ArithmeticTarget),      // RES u3,r8
    RES16(u8),                      // RES u3,[HL] - Possibly wrong as HL is address
    SET(u8, ArithmeticTarget),      // SET u3,r8
    SET16(u8),                      // SET u3,[HL] - Possibly wrong as HL is address
    SRL(ArithmeticTarget),          // SRL r8
    SRA(ArithmeticTarget),          // SRA r8
    SLA(ArithmeticTarget),          // SLA r8
    SWAP(ArithmeticTarget),         // SWAP r8
}

enum ArithmeticTarget
{
    A, B, C, D, E, H, L
}

enum ArithmeticTarget16
{
    BC, DE, HL,
}

struct CPU
{
    pub registers: Registers
}

impl CPU
{
    fn get_arithmetic_target_value(&self, target: ArithmeticTarget) -> Option<u8>
    {
        match target
        {
            ArithmeticTarget::B => Some(self.registers.b),
            ArithmeticTarget::C => Some(self.registers.c),
            ArithmeticTarget::D => Some(self.registers.d),
            ArithmeticTarget::E => Some(self.registers.e),
            ArithmeticTarget::H => Some(self.registers.h),
            ArithmeticTarget::L => Some(self.registers.l),
            _ => None,
        }
    }

    fn get_arithmetic_target_mut(&mut self, target: ArithmeticTarget) -> Option<&mut u8>
    {
        match target
        {
            ArithmeticTarget::B => Some(&mut self.registers.b),
            ArithmeticTarget::C => Some(&mut self.registers.c),
            ArithmeticTarget::D => Some(&mut self.registers.d),
            ArithmeticTarget::E => Some(&mut self.registers.e),
            ArithmeticTarget::H => Some(&mut self.registers.h),
            ArithmeticTarget::L => Some(&mut self.registers.l),
            _ => None,
        }
    }

    fn get_arithmetic_target_value16(&self, target: ArithmeticTarget16) -> Option<u16>
    {
        match target
        {
            ArithmeticTarget16::BC => Some(self.registers.get_bc()),
            ArithmeticTarget16::DE => Some(self.registers.get_de()),
            ArithmeticTarget16::HL => Some(self.registers.get_hl())
        }
    }

    fn execute(&mut self, instruction: Instruction)
    {
        match instruction
        {
            Instruction::ADD(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_value(target)
                {
                    let result = self.add(value);
                    self.registers.a = result;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::ADDHL(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_value16(target)
                {
                    let result = self.addhl(value);
                    self.registers.set_hl(result);
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::SUB(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_value(target)
                {
                    let result = self.sub(value);
                    self.registers.a = result;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::SBC(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_value(target)
                {
                    let result = self.sbc(value);
                    self.registers.a = result;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::AND(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_value(target)
                {
                    let result = self.and(value);
                    self.registers.a = result;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::OR(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_value(target)
                {
                    let result = self.or(value);
                    self.registers.a = result;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::XOR(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_value(target)
                {
                    let result = self.xor(value);
                    self.registers.a = result;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::CP(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_value(target)
                {
                    self.sub(value);
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::INC(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    let result = initial + 1;
                    *value = result;

                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = (initial & 0xF) + 1 > 0xF;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::DEC(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    let result = initial - 1;
                    *value = result;

                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = true;
                    self.registers.f.half_carry = initial & 0xF == 0;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::CCF() =>
            {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = !self.registers.f.carry;
            }

            Instruction::SCF() =>
            {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
            }

            Instruction::RRA() =>
            {
                let msb = if self.registers.f.carry { 1 << 7 } else { 0 };

                let initial = self.registers.a;
                let result = initial >> 1 | msb;
                self.registers.a = result;

                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = initial & 0x1 == 1;
            }

            Instruction::RR(target) =>
            {
                let msb = if self.registers.f.carry { 1 << 7 } else { 0 };

                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    let result = initial >> 1 | msb;
                    *value = result;
    
                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                    self.registers.f.carry = initial & 0x1 == 1;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::RLA() =>
            {
                let lsb = if self.registers.f.carry { 1 } else { 0 };
                let initial = self.registers.a;
                let result = initial << 1 | lsb;
                self.registers.a = result;

                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = initial & 0x80 == 1;
            }

            Instruction::RL(target) =>
            {
                let lsb = if self.registers.f.carry { 1 } else { 0 };

                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    let result = initial << 1 | lsb;
                    *value = result;
    
                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                    self.registers.f.carry = initial & 0x80 == 1;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::RRCA() =>
            {
                let initial = self.registers.a;
                let lsb = initial & 0x1;
                let result = initial >> 1 | lsb << 7;
                self.registers.a = result;

                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = lsb != 1;
            }

            Instruction::RRC(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    let lsb = initial & 0x1;
                    let result = initial >> 1 | lsb << 7;
                    *value = result;

                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                    self.registers.f.carry = lsb != 1;
                }
                else
                {
                    // TODO: support more targets
                }
            }


            Instruction::RLCA() =>
            {
                let initial = self.registers.a;
                let msb = (initial & 0x80) >> 7;
                let result = initial << 1 | msb;
                self.registers.a = result;

                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = msb == 1;
            }

            Instruction::RLC(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    let msb = (initial & 0x80) >> 7;
                    let result = initial << 1 | msb;
                    *value = result;
    
                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                    self.registers.f.carry = msb == 1;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::CPL() =>
            {
                let initial = self.registers.a;
                self.registers.a = !initial;

                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;
            }

            Instruction::BIT(bit_to_check, target) =>
            {
                if let Some(value) = self.get_arithmetic_target_value(target)
                {
                    let bit_set = ((1 << bit_to_check) & value) > 0;

                    self.registers.f.zero = bit_set == false;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::BIT16(bit_to_check) =>
            {
                let value = self.registers.get_hl();
                let bit_set = ((1 << bit_to_check) & value) > 0;

                self.registers.f.zero = bit_set == false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            }

            Instruction::RES(bit_to_set, target) =>
            {
                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    let result = !(1 << bit_to_set) & initial;
                    *value = result;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::RES16(bit_to_set) =>
            {
                let value = self.registers.get_hl();
                let result = !(1 << bit_to_set) & value;
                self.registers.set_hl(result);
            }

            Instruction::SET(bit_to_set, target) =>
            {
                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    let result = (1 << bit_to_set) & initial;
                    *value = result;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::SET16(bit_to_set) =>
            {
                let value = self.registers.get_hl();
                let result = (1 << bit_to_set) & value;
                self.registers.set_hl(result);
            }

            Instruction::SRL(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    // Right shift whole register
                    let result = initial >> 1;
                    *value = result;

                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                    self.registers.f.carry = (initial & 0x1) > 0;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::SRA(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    // Right shift but preserve sign i.e. the 7th bit
                    let result = (initial >> 1) & (initial & 0x80);
                    *value = result;

                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                    self.registers.f.carry = (initial & 0x1) > 0;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::SLA(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    // Shift but preserve sign i.e. the 7th bit
                    let result = initial << 1;
                    *value = result;

                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                    self.registers.f.carry = (initial & 0x80) > 0;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::SWAP(target) =>
            {
                if let Some(value) = self.get_arithmetic_target_mut(target)
                {
                    let initial = *value;
                    // Shift but preserve sign i.e. the 7th bit
                    let result = (initial << 4) & (initial >> 4);
                    *value = result;

                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                    self.registers.f.carry = false;
                }
                else
                {
                    // TODO: support more targets
                }
            }
        }
    }

    fn add(&mut self, value: u8) -> u8
    {
        // Rust panics on overflow unless specifically instructed to do it regardless.  Capture did overflow for flag setting.
        let (result, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Did result overflow from lower 4-bits (nibble)
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        result
    }

    fn addhl(&mut self, value: u16) -> u16
    {
        let (result, did_overflow) = self.registers.get_hl().overflowing_add(value);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFFF) + (value & 0xFFF) > 0xFFF;
        result
    }

    fn sub(&mut self, value: u8) -> u8
    {
        let result = self.registers.a - value;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = self.registers.a < value;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        result
    }

    fn sbc(&mut self, value: u8) -> u8
    {
        let to_subtract = value + self.registers.f.carry as u8;
        let result = self.registers.a - to_subtract;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = self.registers.a < to_subtract;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (to_subtract & 0xF);
        result
    }

    fn and(&mut self, value: u8) -> u8
    {
        let result = self.registers.a & value;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = true;
        result
    }

    fn or(&mut self, value: u8) -> u8
    {
        let result = self.registers.a | value;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        result
    }

    fn xor(&mut self, value: u8) -> u8
    {
        let result = self.registers.a ^ value;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        result
    }
}
