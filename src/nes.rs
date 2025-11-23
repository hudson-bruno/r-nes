use crate::cpu::{Cpu, ExitStatus};

pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn new() -> Self {
        Nes { cpu: Cpu::new() }
    }

    pub fn run(&mut self) -> ExitStatus {
        self.cpu.run()
    }
}

impl Default for Nes {
    fn default() -> Self {
        Self::new()
    }
}
