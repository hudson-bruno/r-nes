use crate::{bus::Bus, nes::Nes};

pub mod stack;

pub trait Memory {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);

    fn read_as_address(&mut self, low_addr: u16, high_addr: u16) -> u16 {
        let low = self.read(low_addr);
        let high = self.read(high_addr);

        u16::from_le_bytes([low, high])
    }
}

impl Memory for Bus {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cpu_memory[(addr & 0x07FF) as usize],
            0x2000..=0x3FFF => self.ppu.read(addr & 0x07),
            0x4000..=0x4017 => todo!("APU/IO registers not yet implemented"),
            0x4018..=0x401F => todo!("APU/IO testing functionality not yet implemented"),
            0x4020..=0xFFFF => match &mut self.cartridge {
                Some(cartridge) => cartridge.read(addr),
                None => (addr >> 8) as u8, // Open bus
            },
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.cpu_memory[(addr & 0x07FF) as usize] = value,
            0x2000..=0x3FFF => self.ppu.write(addr & 0x07, value),
            0x4000..=0x4017 => todo!("APU/IO registers not yet implemented"),
            0x4018..=0x401F => todo!("APU/IO testing functionality not yet implemented"),
            0x4020..=0xFFFF => {
                if let Some(cartridge) = &mut self.cartridge {
                    cartridge.write(addr, value)
                }
            }
        }
    }
}

impl Memory for Nes {
    fn read(&mut self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.bus.write(addr, value)
    }
}
