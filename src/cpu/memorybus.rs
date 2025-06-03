use crate::gpu::GPU;
use crate::gpu::VRAM_BEGIN;
use crate::gpu::VRAM_END;
//use crate::gpu::VRAM_SIZE;

pub struct MemoryBus
{
    memory: [u8; 0xFFFF],
    gpu: GPU,
}
  
impl MemoryBus
{
    pub fn read_byte(&self, address: u16) -> u8
    {
        let address = address as usize;
        match address
        {
            VRAM_BEGIN ..= VRAM_END =>
            {
                self.gpu.read_vram(address - VRAM_BEGIN)
            }
            _ =>
            {
                self.memory[address]
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8)
    {
        let address = address as usize;
        match address
        {
            VRAM_BEGIN ..= VRAM_END =>
            {
                self.gpu.write_vram(address - VRAM_BEGIN, value)
            }
            _ =>
            {
                self.memory[address] = value
            }
        }

        
    }
}
