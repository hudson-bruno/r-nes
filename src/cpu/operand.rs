use crate::cpu::{Cpu, memory::Memory};

pub enum OperandLocation {
    Implicit,
    Accumulator,
    Memory(u16),
    Relative(u16),
}

pub enum OperandValue {
    None,
    U8(u8),
    I8(i8),
}

pub trait Operand {
    fn get_operand(&self) -> OperandValue;
    fn update_operand(&mut self, value: u8);
}

impl Operand for Cpu {
    fn get_operand(&self) -> OperandValue {
        match self.operand_location {
            OperandLocation::Implicit => OperandValue::None,
            OperandLocation::Accumulator => OperandValue::U8(self.a_register),
            OperandLocation::Memory(addr) => OperandValue::U8(self.read(addr)),
            OperandLocation::Relative(addr) => {
                OperandValue::I8(i8::from_be_bytes([self.read(addr)]))
            }
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
