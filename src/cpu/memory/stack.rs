use crate::cpu::{Cpu, memory::Memory};

pub trait Stack: Memory {
    fn stack_push(&mut self, value: u8);
    fn stack_push_address(&mut self, addr: u16);
    fn stack_pop(&mut self) -> u8;
    fn stack_pop_address(&mut self) -> u16;
}

impl Stack for Cpu {
    fn stack_push(&mut self, value: u8) {
        self.write(0x0100 + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_address(&mut self, addr: u16) {
        let high = (addr >> 8) as u8;
        let low = addr as u8;

        self.stack_push(high);
        self.stack_push(low);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.read(0x0100 + self.stack_pointer as u16)
    }

    fn stack_pop_address(&mut self) -> u16 {
        let low = self.stack_pop();
        let high = self.stack_pop();

        u16::from_le_bytes([low, high])
    }
}
