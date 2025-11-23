use crate::cpu::Cpu;

pub trait Memory {
    fn read(&self, addr: u16) -> u8;
    fn read_as_address(&self, addr: u16) -> u16;
}

impl Memory for Cpu {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn read_as_address(&self, addr: u16) -> u16 {
        let low = self.read(addr);
        let high = self.read(addr + 1);

        u16::from_le_bytes([low, high])
    }
}
