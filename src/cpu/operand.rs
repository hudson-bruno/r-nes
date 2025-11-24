use crate::cpu::{Cpu, memory::Memory};

pub enum OperandLocation {
    Implicit,
    Accumulator,
    Memory(u16),
}

pub trait Operand {
    fn get_operand(&self) -> Option<u8>;
    fn update_operand(&mut self, value: u8);
}

impl Operand for Cpu {
    fn get_operand(&self) -> Option<u8> {
        match self.operand_location {
            OperandLocation::Implicit => None,
            OperandLocation::Accumulator => Some(self.a_register),
            OperandLocation::Memory(addr) => Some(self.read(addr)),
        }
    }

    fn update_operand(&mut self, value: u8) {
        match self.operand_location {
            OperandLocation::Accumulator => {
                self.a_register = value;
            }
            OperandLocation::Memory(addr) => {
                self.write(addr, value);
            }
            _ => (),
        }
    }
}
