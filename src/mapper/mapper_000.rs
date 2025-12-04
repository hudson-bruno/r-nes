use crate::mapper::Mapper;

pub struct Mapper000 {
    pub program_memory_banks: u8,
}

impl Mapper for Mapper000 {
    fn map_cpu_read(&self, addr: u16) -> u16 {
        let prg_rom_addr_mask = if self.program_memory_banks > 1 {
            0x7FFF
        } else {
            0x3FFF
        };

        match addr {
            0x6000..=0x7FFF => todo!("cartridge prg ram functionality not yet implemented"),
            0x8000..=0xFFFF => addr & prg_rom_addr_mask,
            _ => todo!("outside mapper cpu address range"),
        }
    }
    fn map_cpu_write(&self, addr: u16) -> u16 {
        match addr {
            0x6000..=0x7FFF => todo!("cartridge prg ram functionality not yet implemented"),
            0x8000..=0xFFFF => {
                todo!("attempted to write in cartridge prg rom! addr = {addr}")
            }
            _ => todo!("outside mapper cpu address range"),
        }
    }

    fn map_ppu_read(&self, addr: u16) -> u16 {
        match addr {
            0x0000..=0x1FFF => addr,
            _ => todo!("outside mapper ppu address range"),
        }
    }
    fn map_ppu_write(&self, addr: u16) -> u16 {
        match addr {
            0x0000..=0x1FFF => todo!("attempted to write in cartridge chr rom! addr = {addr}"),
            _ => todo!("outside mapper ppu address range"),
        }
    }
}
