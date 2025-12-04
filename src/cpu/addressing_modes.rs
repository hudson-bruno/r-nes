use crate::cpu::{Cpu, memory::Memory, operand::OperandLocation};

pub trait AddressingModes {
    fn implicit(&mut self) -> OperandLocation;
    fn accumulator(&mut self) -> OperandLocation;
    fn immediate(&mut self) -> OperandLocation;
    fn zero_page(&mut self, mem: &mut impl Memory) -> OperandLocation;
    fn zero_page_x(&mut self, mem: &mut impl Memory) -> OperandLocation;
    fn zero_page_y(&mut self, mem: &mut impl Memory) -> OperandLocation;
    fn relative(&mut self) -> OperandLocation;
    fn absolute(&mut self, mem: &mut impl Memory) -> OperandLocation;
    fn absolute_x(&mut self, mem: &mut impl Memory) -> OperandLocation;
    fn absolute_y(&mut self, mem: &mut impl Memory) -> OperandLocation;
    fn indirect(&mut self, mem: &mut impl Memory) -> OperandLocation;
    fn indirect_x(&mut self, mem: &mut impl Memory) -> OperandLocation;
    fn indirect_y(&mut self, mem: &mut impl Memory) -> OperandLocation;
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

    fn zero_page(&mut self, mem: &mut impl Memory) -> OperandLocation {
        let addr = mem.read(self.program_counter) as u16;
        self.program_counter += 1;

        OperandLocation::Memory(addr)
    }

    fn zero_page_x(&mut self, mem: &mut impl Memory) -> OperandLocation {
        let addr = mem
            .read(self.program_counter)
            .wrapping_add(self.x_index_register) as u16;
        self.program_counter += 1;

        OperandLocation::Memory(addr)
    }

    fn zero_page_y(&mut self, mem: &mut impl Memory) -> OperandLocation {
        let addr = mem
            .read(self.program_counter)
            .wrapping_add(self.y_index_register) as u16;
        self.program_counter += 1;

        OperandLocation::Memory(addr)
    }

    fn relative(&mut self) -> OperandLocation {
        let addr = self.program_counter;
        self.program_counter += 1;

        OperandLocation::Relative(addr)
    }

    fn absolute(&mut self, mem: &mut impl Memory) -> OperandLocation {
        let addr = mem.read_as_address(self.program_counter, self.program_counter + 1);
        self.program_counter += 2;

        OperandLocation::Memory(addr)
    }

    fn absolute_x(&mut self, mem: &mut impl Memory) -> OperandLocation {
        let addr = mem
            .read_as_address(self.program_counter, self.program_counter + 1)
            .wrapping_add(self.x_index_register as u16);
        self.program_counter += 2;

        OperandLocation::Memory(addr)
    }

    fn absolute_y(&mut self, mem: &mut impl Memory) -> OperandLocation {
        let addr = mem
            .read_as_address(self.program_counter, self.program_counter + 1)
            .wrapping_add(self.y_index_register as u16);
        self.program_counter += 2;

        OperandLocation::Memory(addr)
    }

    fn indirect(&mut self, mem: &mut impl Memory) -> OperandLocation {
        let indirect_addr = mem.read_as_address(self.program_counter, self.program_counter + 1);
        self.program_counter += 2;

        let indirect_high_addr = if indirect_addr & 0x00FF == 0x00FF {
            indirect_addr & 0xFF00
        } else {
            indirect_addr + 1
        };

        let addr = mem.read_as_address(indirect_addr, indirect_high_addr);
        OperandLocation::Memory(addr)
    }

    fn indirect_x(&mut self, mem: &mut impl Memory) -> OperandLocation {
        let indirect_addr = mem
            .read(self.program_counter)
            .wrapping_add(self.x_index_register) as u16;
        self.program_counter += 1;

        OperandLocation::Memory(mem.read_as_address(indirect_addr, indirect_addr + 1))
    }

    fn indirect_y(&mut self, mem: &mut impl Memory) -> OperandLocation {
        let indirect_addr = mem.read(self.program_counter) as u16;
        self.program_counter += 1;

        OperandLocation::Memory(
            mem.read_as_address(indirect_addr, indirect_addr + 1)
                .wrapping_add(self.y_index_register as u16),
        )
    }
}
