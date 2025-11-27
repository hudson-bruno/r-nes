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
            0x2000..=0x3FFF => todo!("PPU registers not yet implemented"),
            0x4000..=0x4017 => todo!("APU/IO registers not yet implemented"),
            0x4018..=0x401F => todo!("APU/IO testing functionality not yet implemented"),
            cartridge_addr @ 0x4020..=0xFFFF => match cartridge_addr {
                0x6000..=0x7FFF => todo!("Cartridge ram functionality not yet implemented"),
                0x8000..=0xFFFF => {
                    match self.cartridge {
                        Some(ref cartridge) => {
                            cartridge.program_memory[(cartridge_addr & 0x7FFF) as usize]
                        }
                        None => (cartridge_addr >> 8) as u8, // Open bus
                    }
                }
                _ => 0,
            },
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.cpu_memory[(addr & 0x07FF) as usize] = value,
            0x2000..=0x3FFF => todo!("PPU registers not yet implemented"),
            0x4000..=0x4017 => todo!("APU/IO registers not yet implemented"),
            0x4018..=0x401F => todo!("APU/IO testing functionality not yet implemented"),
            cartridge_addr @ 0x4020..=0xFFFF => match cartridge_addr {
                0x6000..=0x7FFF => todo!("Cartridge ram functionality not yet implemented"),
                0x8000..=0xFFFF => todo!("Cartridge rom functionality not yet implemented"),
                _ => (),
            },
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
