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
        loop {
            if let Some(err) = self.step() {
                return err;
            }
        }
    }

    pub fn step(&mut self) -> Option<ExitStatus> {
        if let Some(exit_status) = self.cpu.step(&mut self.bus) {
            return Some(exit_status);
        }

        for _ in 0..3 {
            if self.bus.ppu.step() {
                self.cpu.nmi(&mut self.bus);
            }
        }

        None
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
