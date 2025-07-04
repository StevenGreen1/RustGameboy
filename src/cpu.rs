mod registers;
use crate::cpu::registers::Registers;

mod memorybus;
use crate::cpu::memorybus::MemoryBus;

mod instruction;
use crate::cpu::instruction::{
    ArithmeticTarget, ArithmeticTarget16, Indirect, Instruction, JumpTest, LoadByteSource,
    LoadByteTarget, LoadType, LoadWordTarget, StackTarget,
};

pub struct CPU
{
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
    is_halted: bool,
    inst_count: u16,
}

impl CPU
{
    pub fn new(boot_rom: Vec<u8>) -> Self
    {
        CPU {
            registers: Registers::new(0),
            pc: 0,
            sp: 0,
            bus: MemoryBus::new(boot_rom),
            is_halted: false,
            inst_count: 0,
        }
    }

    pub fn step(&mut self)
    {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        println!(
            "instruction_byte = 0x{:x}, instruction count {}",
            instruction_byte, self.inst_count
        );
        let prefixed = instruction_byte == 0xCB;
        if prefixed
        {
            instruction_byte = self.bus.read_byte(self.pc + 1);
            // Skip the next pc as it is read here
            self.pc += 1;
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        }
        else
        {
            let description =
                format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };
        self.inst_count = self.inst_count.wrapping_add(1);

        println!("The current pc is {}", self.pc);
        println!("The next pc is {}", next_pc);

        // To implement
        //if self.bus.has_interrupt()
        //{
        //    self.is_halted = false;
        //}
        if !self.is_halted
        {
            self.pc = next_pc;
        }
    }

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
            ArithmeticTarget16::HL => Some(self.registers.get_hl()),
        }
    }

    fn set_arithmetic_target_value16(&mut self, target: ArithmeticTarget16, value: u16)
    {
        match target
        {
            ArithmeticTarget16::BC => self.registers.set_bc(value),
            ArithmeticTarget16::DE => self.registers.set_de(value),
            ArithmeticTarget16::HL => self.registers.set_hl(value),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> u16
    {
        let mut pc_increment = 1;

        match instruction
        {
            Instruction::NOP() =>
            {}
            Instruction::HALT() =>
            {
                self.is_halted = true;
            }
            Instruction::CALL(test) =>
            {
                let jump_condition = match test
                {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ =>
                    {
                        panic!("TODO: support more conditions")
                    }
                };
                self.call(jump_condition);
            }
            Instruction::RETURN(test) =>
            {
                let jump_condition = match test
                {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ =>
                    {
                        panic!("TODO: support more conditions")
                    }
                };
                self.return_(jump_condition);
            }
            Instruction::PUSH(source) =>
            {
                let value = match source
                {
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    _ =>
                    {
                        panic!("TODO: support more targets")
                    }
                };
                self.push(value);
            }
            Instruction::POP(target) =>
            {
                let result = self.pop();
                match target
                {
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    _ =>
                    {
                        panic!("TODO: support more targets")
                    }
                };
            }
            Instruction::LD(load_type) => match load_type
            {
                LoadType::Byte(target, source) =>
                {
                    let source_value = match source
                    {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::B => self.registers.b,
                        LoadByteSource::C => self.registers.c,
                        LoadByteSource::D => self.registers.d,
                        LoadByteSource::E => self.registers.e,
                        LoadByteSource::H => self.registers.h,
                        LoadByteSource::L => self.registers.l,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        _ =>
                        {
                            panic!("TODO: implement other sources")
                        }
                    };

                    match target
                    {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::B => self.registers.b = source_value,
                        LoadByteTarget::C => self.registers.c = source_value,
                        LoadByteTarget::D => self.registers.d = source_value,
                        LoadByteTarget::E => self.registers.e = source_value,
                        LoadByteTarget::H => self.registers.h = source_value,
                        LoadByteTarget::L => self.registers.l = source_value,
                        LoadByteTarget::HLI =>
                        {
                            self.bus.write_byte(self.registers.get_hl(), source_value)
                        }
                    }

                    match source
                    {
                        LoadByteSource::D8 => pc_increment = 2,
                        _ => pc_increment = 1,
                    }
                }
                LoadType::Word(target) =>
                {
                    let word = self.read_next_word();
                    match target
                    {
                        LoadWordTarget::AF => self.registers.set_af(word),
                        LoadWordTarget::BC => self.registers.set_bc(word),
                        LoadWordTarget::DE => self.registers.set_de(word),
                        LoadWordTarget::HL => self.registers.set_hl(word),
                        LoadWordTarget::SP => self.sp = word,
                        _ =>
                        {
                            panic!("TODO: implement other targets")
                        }
                    }
                    pc_increment = 3;
                }
                LoadType::AFromIndirect(indirect) =>
                {
                    self.registers.a = match indirect
                    {
                        Indirect::BCIndirect => self.bus.read_byte(self.registers.get_bc()),
                        Indirect::DEIndirect => self.bus.read_byte(self.registers.get_de()),
                        Indirect::HLIndirectMinus =>
                        {
                            let hl = self.registers.get_hl();
                            self.registers.set_hl(hl.wrapping_sub(1));
                            self.bus.read_byte(hl)
                        }
                        Indirect::HLIndirectPlus =>
                        {
                            let hl = self.registers.get_hl();
                            self.registers.set_hl(hl.wrapping_add(1));
                            self.bus.read_byte(hl)
                        }
                        Indirect::WordIndirect =>
                        {
                            let word = self.read_next_word();
                            self.bus.read_byte(word)
                        }
                        Indirect::LastByteIndirect =>
                        {
                            self.bus.read_byte(0xFF00 + self.registers.c as u16)
                        }
                        _ =>
                        {
                            panic!("TODO: implement other targets")
                        }
                    }
                }
                LoadType::IndirectFromA(indirect) => match indirect
                {
                    Indirect::BCIndirect =>
                    {
                        let address = self.registers.get_bc();
                        self.bus.write_byte(address, self.registers.a)
                    }
                    Indirect::DEIndirect =>
                    {
                        let address = self.registers.get_de();
                        self.bus.write_byte(address, self.registers.a)
                    }
                    Indirect::HLIndirectMinus =>
                    {
                        let hl = self.registers.get_hl();
                        self.registers.set_hl(hl.wrapping_sub(1));
                        self.bus.write_byte(hl, self.registers.a)
                    }
                    Indirect::HLIndirectPlus =>
                    {
                        let hl = self.registers.get_hl();
                        self.registers.set_hl(hl.wrapping_add(1));
                        self.bus.write_byte(hl, self.registers.a)
                    }
                    Indirect::WordIndirect =>
                    {
                        let word = self.read_next_word();
                        self.bus.write_byte(word, self.registers.a)
                    }
                    Indirect::LastByteIndirect =>
                    {
                        let c_reg = self.registers.c as u16;
                        self.bus.write_byte(0xFF00 + c_reg, self.registers.a)
                    }
                },
                LoadType::AFromByteAddress() =>
                {
                    let offset = self.read_next_byte() as u16;
                    self.registers.a = self.bus.read_byte(0xFF00 + offset);
                    pc_increment = 2
                }
                LoadType::ByteAddressFromA() =>
                {
                    let offset = self.read_next_byte() as u16;
                    self.bus.write_byte(0xFF00 + offset, self.registers.a);
                    pc_increment = 2
                }
                _ =>
                {
                    panic!("TODO: implement other load types")
                }
            },

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

            Instruction::XOR16(target) =>
            {
                if let Some(address) = self.get_arithmetic_target_value16(target)
                {
                    let value = self.bus.read_byte(address);
                    let result = self.xor(value);
                    self.registers.a = result;
                }
                else
                {
                    // TODO: support more targets
                }
            }

            Instruction::XORD8() =>
            {
                let value = self.read_next_byte();
                let result = self.xor(value);
                self.registers.a = result;
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

            Instruction::SWAP16(target) =>
            {
                if let Some(address) = self.get_arithmetic_target_value16(target)
                {
                    let value = self.bus.read_byte(address);
                    let initial = value;
                    // Shift but preserve sign i.e. the 7th bit
                    let result = (initial << 4) & (initial >> 4);
                    self.bus.write_byte(address, result);

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

            Instruction::JP(test) =>
            {
                let jump_condition = match test
                {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                return self.jump(jump_condition);
            }

            Instruction::JR(test) =>
            {
                let jump_condition = match test
                {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                return self.jump_relative(jump_condition);
            }
        }

        return self.pc.wrapping_add(pc_increment);
    }

    fn read_next_byte(&mut self) -> u8
    {
        return self.bus.read_byte(self.pc + 1);
    }

    fn read_next_word(&mut self) -> u16
    {
        return ((self.bus.read_byte(self.pc + 2) as u16) << 8)
            | (self.bus.read_byte(self.pc + 1) as u16);
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

    fn jump(&self, should_jump: bool) -> u16
    {
        if should_jump
        {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        }
        else
        {
            // If we don't jump we need to still move the program
            // counter forward by 3 since the jump instruction is
            // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            self.pc.wrapping_add(3)
        }
    }

    fn jump_relative(&mut self, should_jump: bool) -> u16
    {
        let next_step = self.pc.wrapping_add(2);
        if should_jump
        {
            let offset = self.read_next_byte() as i8;
            let pc = if offset >= 0
            {
                next_step.wrapping_add(offset as u16)
            }
            else
            {
                next_step.wrapping_sub(offset.abs() as u16)
            };
            pc
        }
        else
        {
            next_step
        }
    }

    fn push(&mut self, value: u16)
    {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16
    {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn call(&mut self, should_jump: bool) -> u16
    {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump
        {
            self.push(next_pc);
            self.read_next_word()
        }
        else
        {
            next_pc
        }
    }

    fn return_(&mut self, should_jump: bool) -> u16
    {
        if should_jump
        {
            self.pop()
        }
        else
        {
            self.pc.wrapping_add(1)
        }
    }
}
