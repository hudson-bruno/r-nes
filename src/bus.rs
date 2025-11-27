use crate::cartridge::Cartridge;

pub struct Bus {
    pub cpu_memory: [u8; 2 * 1024],
    pub cartridge: Option<Cartridge>,
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
        }
    }
}
