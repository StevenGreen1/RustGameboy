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
