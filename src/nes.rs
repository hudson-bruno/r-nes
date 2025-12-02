use crate::{
    bus::Bus,
    cartridge::Cartridge,
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

    pub fn new_with_cartridge(cartridge: Cartridge) -> Self {
        let mut bus = Bus::new();
        bus.cartridge = Some(cartridge);

        Nes {
            cpu: Cpu::new(&mut bus),
            bus,
        }
    }

    pub fn run(&mut self) -> ExitStatus {
        self.cpu.run(&mut self.bus)
    }

    pub fn step(&mut self) -> Option<ExitStatus> {
        self.cpu.step(&mut self.bus)
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
