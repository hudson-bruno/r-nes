use crate::cpu::{addressing_modes::AddressingModes, instructions::Instructions, memory::Memory};

mod addressing_modes;
mod instructions;
mod memory;

pub struct Cpu {
    pub memory: [u8; 2 * 1024],

    // Registers
    pub a_register: u8,
    pub status_register: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub x_index_register: u8,
    pub y_index_register: u8,
}

#[derive(Debug, PartialEq)]
pub enum ExitStatus {
    Brk,
    UnknownOpCode,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: [0; 2 * 1024],
            a_register: 0,
            status_register: 0,
            program_counter: 0,
            stack_pointer: 0,
            x_index_register: 0,
            y_index_register: 0,
        }
    }

    pub fn run(&mut self) -> ExitStatus {
        loop {
            if let Some(err) = self.clock() {
                return err;
            }
        }
    }

    fn clock(&mut self) -> Option<ExitStatus> {
        let op_code = self.read(self.program_counter);
        self.program_counter += 1;

        match op_code {
            0xA9 => {
                let addr = self.immediate();
                let value = self.read(addr);

                self.lda(value);

                None
            }
            0x00 => Some(ExitStatus::Brk),
            _ => Some(ExitStatus::UnknownOpCode),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lda_immediate() {
        let mut cpu = Cpu::new();
        cpu.memory[0..3].copy_from_slice(&[0xA9, 0x01, 0x00]);
        let result = cpu.run();

        assert_eq!(result, ExitStatus::Brk);
        assert_eq!(cpu.a_register, 0x01);
    }
}
