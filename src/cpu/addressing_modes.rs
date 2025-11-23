use crate::cpu::{Cpu, memory::Memory};

pub trait AddressingModes {
    fn immediate(&mut self) -> u16;
    fn zero_page(&mut self) -> u16;
    fn zero_page_x(&mut self) -> u16;
    fn absolute(&mut self) -> u16;
    fn absolute_x(&mut self) -> u16;
    fn absolute_y(&mut self) -> u16;
    fn indirect_x(&mut self) -> u16;
}

impl AddressingModes for Cpu {
    fn immediate(&mut self) -> u16 {
        let addr = self.program_counter;
        self.program_counter += 1;

        addr
    }

    fn zero_page(&mut self) -> u16 {
        let addr = self.read(self.program_counter) as u16;
        self.program_counter += 1;

        addr
    }

    fn zero_page_x(&mut self) -> u16 {
        let addr = self
            .read(self.program_counter)
            .wrapping_add(self.x_index_register) as u16;
        self.program_counter += 1;

        addr
    }

    fn absolute(&mut self) -> u16 {
        let addr = self.read_as_address(self.program_counter);
        self.program_counter += 2;

        addr
    }

    fn absolute_x(&mut self) -> u16 {
        let addr = self
            .read_as_address(self.program_counter)
            .wrapping_add(self.x_index_register as u16);
        self.program_counter += 2;

        addr
    }

    fn absolute_y(&mut self) -> u16 {
        let addr = self
            .read_as_address(self.program_counter)
            .wrapping_add(self.y_index_register as u16);
        self.program_counter += 2;

        addr
    }

    fn indirect_x(&mut self) -> u16 {
        let indirect_addr = self
            .read(self.program_counter)
            .wrapping_add(self.x_index_register) as u16;
        self.program_counter += 1;

        self.read_as_address(indirect_addr)
    }
}
