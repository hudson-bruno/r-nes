use crate::cpu::{Cpu, memory::Memory};

pub trait Stack {
    fn stack_push(&mut self, mem: &mut impl Memory, value: u8);
    fn stack_pop(&mut self, mem: &mut impl Memory) -> u8;

    fn stack_push_address(&mut self, mem: &mut impl Memory, addr: u16) {
        let high = (addr >> 8) as u8;
        let low = addr as u8;

        self.stack_push(mem, high);
        self.stack_push(mem, low);
    }

    fn stack_pop_address(&mut self, mem: &mut impl Memory) -> u16 {
        let low = self.stack_pop(mem);
        let high = self.stack_pop(mem);

        u16::from_le_bytes([low, high])
    }
}

impl Stack for Cpu {
    fn stack_push(&mut self, mem: &mut impl Memory, value: u8) {
        mem.write(0x0100 + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_pop(&mut self, mem: &mut impl Memory) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        mem.read(0x0100 + self.stack_pointer as u16)
    }
}
