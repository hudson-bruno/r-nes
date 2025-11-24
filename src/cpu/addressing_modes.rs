use crate::cpu::{Cpu, memory::Memory, operand::OperandLocation};

pub trait AddressingModes {
    fn implicit(&mut self) -> OperandLocation;
    fn accumulator(&mut self) -> OperandLocation;
    fn immediate(&mut self) -> OperandLocation;
    fn zero_page(&mut self) -> OperandLocation;
    fn zero_page_x(&mut self) -> OperandLocation;
    fn absolute(&mut self) -> OperandLocation;
    fn absolute_x(&mut self) -> OperandLocation;
    fn absolute_y(&mut self) -> OperandLocation;
    fn indirect_x(&mut self) -> OperandLocation;
    fn indirect_y(&mut self) -> OperandLocation;
}

impl AddressingModes for Cpu {
    fn implicit(&mut self) -> OperandLocation {
        OperandLocation::Implicit
    }

    fn accumulator(&mut self) -> OperandLocation {
        OperandLocation::Accumulator
    }

    fn immediate(&mut self) -> OperandLocation {
        let addr = self.program_counter;
        self.program_counter += 1;

        OperandLocation::Memory(addr)
    }

    fn zero_page(&mut self) -> OperandLocation {
        let addr = self.read(self.program_counter) as u16;
        self.program_counter += 1;

        OperandLocation::Memory(addr)
    }

    fn zero_page_x(&mut self) -> OperandLocation {
        let addr = self
            .read(self.program_counter)
            .wrapping_add(self.x_index_register) as u16;
        self.program_counter += 1;

        OperandLocation::Memory(addr)
    }

    fn absolute(&mut self) -> OperandLocation {
        let addr = self.read_as_address(self.program_counter);
        self.program_counter += 2;

        OperandLocation::Memory(addr)
    }

    fn absolute_x(&mut self) -> OperandLocation {
        let addr = self
            .read_as_address(self.program_counter)
            .wrapping_add(self.x_index_register as u16);
        self.program_counter += 2;

        OperandLocation::Memory(addr)
    }

    fn absolute_y(&mut self) -> OperandLocation {
        let addr = self
            .read_as_address(self.program_counter)
            .wrapping_add(self.y_index_register as u16);
        self.program_counter += 2;

        OperandLocation::Memory(addr)
    }

    fn indirect_x(&mut self) -> OperandLocation {
        let indirect_addr = self
            .read(self.program_counter)
            .wrapping_add(self.x_index_register) as u16;
        self.program_counter += 1;

        OperandLocation::Memory(self.read_as_address(indirect_addr))
    }

    fn indirect_y(&mut self) -> OperandLocation {
        let indirect_addr = self.read(self.program_counter) as u16;
        self.program_counter += 1;

        OperandLocation::Memory(
            self.read_as_address(indirect_addr)
                .wrapping_add(self.y_index_register as u16),
        )
    }
}
