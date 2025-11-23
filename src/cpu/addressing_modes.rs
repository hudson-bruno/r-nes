use crate::cpu::Cpu;

pub trait AddressingModes {
    fn immediate(&mut self) -> u16;
}

impl AddressingModes for Cpu {
    fn immediate(&mut self) -> u16 {
        let addr = self.program_counter;
        self.program_counter += 1;

        addr
    }
}
