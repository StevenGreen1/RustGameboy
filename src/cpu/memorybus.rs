use crate::gpu::GPU;
use crate::gpu::VRAM_BEGIN;
use crate::gpu::VRAM_END;
//use crate::gpu::VRAM_SIZE;

pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;
pub struct MemoryBus
{
    memory: [u8; 0xFFFF],
    gpu: GPU,
}

impl MemoryBus
{
    pub fn new(boot_rom: Vec<u8>) -> Self
    {
        let mut memory = [0; 0xFFFF];

        let len = boot_rom.len().min(BOOT_ROM_END);
        memory[..len].copy_from_slice(&boot_rom[..len]);

        Self { memory, gpu: GPU::new() }
    }

    pub fn read_byte(&self, address: u16) -> u8
    {
        let address = address as usize;
        match address
        {
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(address - VRAM_BEGIN),
            _ => self.memory[address],
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8)
    {
        let address = address as usize;
        match address
        {
            VRAM_BEGIN..=VRAM_END => self.gpu.write_vram(address - VRAM_BEGIN, value),
            _ => self.memory[address] = value,
        }
    }
}
