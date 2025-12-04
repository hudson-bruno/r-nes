use crate::{cartridge::Cartridge, ppu::Ppu};

pub struct Bus {
    pub cpu_memory: [u8; 2 * 1024],
    pub cartridge: Option<Cartridge>,
    pub ppu: Ppu,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu_memory: [0; 2 * 1024],
            cartridge: None,
            ppu: Ppu::new(),
        }
    }
}
