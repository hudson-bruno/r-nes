use crate::{
    bus::Bus,
    cpu::{Cpu, ExitStatus},
};

pub struct Nes {
    pub cpu: Cpu,
    pub bus: Bus,
}

impl Nes {
    pub fn new() -> Self {
        let mut bus = Bus::new();

        Nes {
            cpu: Cpu::new(&mut bus),
            bus,
        }
    }

    pub fn run(&mut self) -> ExitStatus {
        self.cpu.run(&mut self.bus)
    }

    pub fn clock(&mut self) -> Option<ExitStatus> {
        self.cpu.clock(&mut self.bus)
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&mut self.bus)
    }

    pub fn irq(&mut self) {
        self.cpu.irq(&mut self.bus);
    }

    pub fn nmi(&mut self) {
        self.cpu.nmi(&mut self.bus);
    }
}

impl Default for Nes {
    fn default() -> Self {
        Self::new()
    }
}
