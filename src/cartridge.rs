pub struct Cartridge {
    pub program_memory: [u8; 32 * 1024],
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            program_memory: [0; 32 * 1024],
        }
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        Self::new()
    }
}
