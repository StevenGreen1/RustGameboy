mod registers;
use cpu::registers::Registers;

enum Instruction
{
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget16),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
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

    fn execute(&mut self, instruction: Instruction)
    {
        match instruction
        {
            Instruction::ADD(target) =>
            {
                match target
                {
                    ArithmeticTarget::B =>
                    {
                        let result = self.add(self.registers.b);
                        self.registers.a = result;
                    }
                    ArithmeticTarget::C =>
                    {
                        let result = self.add(self.registers.c);
                        self.registers.a = result;
                    }
                    ArithmeticTarget::D =>
                    {
                        let result = self.add(self.registers.d);
                        self.registers.a = result;
                    }
                    ArithmeticTarget::E =>
                    {
                        let result = self.add(self.registers.e);
                        self.registers.a = result;
                    }
                    ArithmeticTarget::H =>
                    {
                        let result = self.add(self.registers.h);
                        self.registers.a = result;
                    }
                    ArithmeticTarget::L =>
                    {
                        let result = self.add(self.registers.l);
                        self.registers.a = result;
                    }
                    _ => { /* TODO: support more targets */ }
                }
            }

            Instruction::ADDHL(target) =>
            {
                match target
                {
                    ArithmeticTarget16::BC =>
                    {
                        let result = self.addhl(self.registers.get_bc());
                        self.registers.set_hl(result);
                    }
                    ArithmeticTarget16::DE =>
                    {
                        let result = self.addhl(self.registers.get_de());
                        self.registers.set_hl(result);
                    }
                    ArithmeticTarget16::HL =>
                    {
                        let result = self.addhl(self.registers.get_hl());
                        self.registers.set_hl(result);
                    }
                    _ => { /* TODO: support more targets */ }
                }
            }

            Instruction::SUB(target) =>
            {
                match target
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
            }

            Instruction::SBC(target) =>
            {
                match target
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
        self.registers.f.subtract = 0;
        self.registers.f.carry = 0;
        self.registers.f.half_carry = 1;
        result
    }
}
