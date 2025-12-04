use crate::{
    cartridge::{Cartridge, Mirroring},
    ppu::Ppu,
};

pub trait PpuMemory {
    fn read(&mut self, addr: u16, cartridge: Option<&mut Cartridge>) -> u8;
    fn write(&mut self, addr: u16, value: u8, cartridge: Option<&mut Cartridge>);
}

impl PpuMemory for Ppu {
    fn read(&mut self, addr: u16, cartridge: Option<&mut Cartridge>) -> u8 {
        match addr {
            0x0000..=0x3EFF => match cartridge {
                Some(cartridge) => {
                    let mapped_addr = cartridge.mapper.map_ppu_read(addr) as usize;
                    match mapped_addr {
                        0x0000..=0x1FFF => cartridge.character_memory[mapped_addr],
                        0x2000..=0x3EFF => {
                            let relative_addr = addr & 0x0FFF;
                            let relative_bank_addr = (relative_addr & 0x03FF) as usize;
                            match cartridge.mirroring {
                                Mirroring::Horizontal => match relative_addr {
                                    0x0000..=0x03FF => self.ppu_vram[0][relative_bank_addr],
                                    0x0400..=0x07FF => self.ppu_vram[0][relative_bank_addr],
                                    0x0800..=0x0BFF => self.ppu_vram[1][relative_bank_addr],
                                    0x0C00..=0x0FFF => self.ppu_vram[1][relative_bank_addr],
                                    _ => (addr >> 8) as u8, // Open bus
                                },
                                Mirroring::Vertical => match relative_addr {
                                    0x0000..=0x03FF => self.ppu_vram[0][relative_bank_addr],
                                    0x0400..=0x07FF => self.ppu_vram[1][relative_bank_addr],
                                    0x0800..=0x0BFF => self.ppu_vram[0][relative_bank_addr],
                                    0x0C00..=0x0FFF => self.ppu_vram[1][relative_bank_addr],
                                    _ => (addr >> 8) as u8, // Open bus
                                },
                            }
                        }
                        _ => (addr >> 8) as u8, // Open bus
                    }
                }
                None => (addr >> 8) as u8, // Open bus
            },
            0x3F00..=0x3FFF => self.read_palette(addr),
            _ => (addr >> 8) as u8, // Open bus
        }
    }

    fn write(&mut self, addr: u16, value: u8, cartridge: Option<&mut Cartridge>) {
        match addr {
            0x0000..=0x3EFF => {
                if let Some(cartridge) = cartridge {
                    let mapped_addr = cartridge.mapper.map_ppu_write(addr) as usize;
                    match mapped_addr {
                        0x0000..=0x1FFF => cartridge.character_memory[mapped_addr] = value,
                        0x2000..=0x3EFF => {
                            let relative_addr = addr & 0x0FFF;
                            let relative_bank_addr = (relative_addr & 0x03FF) as usize;
                            match cartridge.mirroring {
                                Mirroring::Horizontal => match relative_addr {
                                    0x0000..=0x03FF => self.ppu_vram[0][relative_bank_addr] = value,
                                    0x0400..=0x07FF => self.ppu_vram[0][relative_bank_addr] = value,
                                    0x0800..=0x0BFF => self.ppu_vram[1][relative_bank_addr] = value,
                                    0x0C00..=0x0FFF => self.ppu_vram[1][relative_bank_addr] = value,
                                    _ => (),
                                },
                                Mirroring::Vertical => match relative_addr {
                                    0x0000..=0x03FF => self.ppu_vram[0][relative_bank_addr] = value,
                                    0x0400..=0x07FF => self.ppu_vram[1][relative_bank_addr] = value,
                                    0x0800..=0x0BFF => self.ppu_vram[0][relative_bank_addr] = value,
                                    0x0C00..=0x0FFF => self.ppu_vram[1][relative_bank_addr] = value,
                                    _ => (),
                                },
                            }
                        }
                        _ => (),
                    }
                }
            }
            0x3F00..=0x3FFF => self.write_palette(addr, value),
            _ => (),
        }
    }
}
