use crate::cpu::Cpu;

pub mod stack;

pub trait Memory {
    fn read(&self, addr: u16) -> u8;
    fn read_as_address(&self, low_addr: u16, high_addr: u16) -> u16;
    fn write(&mut self, addr: u16, value: u8);
}

impl Memory for Cpu {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn read_as_address(&self, low_addr: u16, high_addr: u16) -> u16 {
        let low = self.read(low_addr);
        let high = self.read(high_addr);

        u16::from_le_bytes([low, high])
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}
