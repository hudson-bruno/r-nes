pub struct Bus {
    pub cpu_memory: [u8; 64 * 1024],
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu_memory: [0; 64 * 1024],
        }
    }
}
