use crate::{bus::Bus, nes::Nes};

pub mod stack;

pub trait Memory {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);

    fn read_as_address(&self, low_addr: u16, high_addr: u16) -> u16 {
        let low = self.read(low_addr);
        let high = self.read(high_addr);

        u16::from_le_bytes([low, high])
    }
}

impl Memory for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cpu_memory[(addr & 0x07FF) as usize],
            _ => self.cpu_memory[(addr) as usize],
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.cpu_memory[(addr & 0x07FF) as usize] = value,
            _ => self.cpu_memory[(addr) as usize] = value,
        }
    }
}

impl Memory for Nes {
    fn read(&self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.bus.write(addr, value)
    }
}
