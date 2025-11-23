use crate::cpu::Cpu;

pub trait Memory {
    fn read(&self, addr: u16) -> u8;
}

impl Memory for Cpu {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
}
