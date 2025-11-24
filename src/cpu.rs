use crate::cpu::{instructions::lookup::INSTRUCTIONS_LOOKUP, memory::Memory};
use bitflags::bitflags;

mod addressing_modes;
mod instructions;
mod memory;

pub struct Cpu {
    pub memory: [u8; 2 * 1024],

    // Registers
    pub a_register: u8,
    pub status_register: Status,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub x_index_register: u8,
    pub y_index_register: u8,

    pub op_memory: u8,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Status: u8 {
        const CARRY     = 1 << 0;
        const ZERO      = 1 << 1;
        const INTERRUPT = 1 << 2;
        const DECIMAL   = 1 << 3;
        const BREAK     = 1 << 4;
        const UNUSED    = 1 << 5;
        const OVERFLOW  = 1 << 6;
        const NEGATIVE  = 1 << 7;
    }
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
            status_register: Status::empty(),
            program_counter: 0,
            stack_pointer: 0,
            x_index_register: 0,
            y_index_register: 0,
            op_memory: 0,
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

        if let Some(op) = &INSTRUCTIONS_LOOKUP[op_code as usize] {
            let addr = (op.addressing_mode)(self);
            self.op_memory = self.read(addr);

            (op.operation)(self)
        } else {
            Some(ExitStatus::UnknownOpCode)
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

    #[test]
    fn test_lda_zero_page() {
        let mut cpu = Cpu::new();
        cpu.memory[0..4].copy_from_slice(&[0xA5, 0x03, 0x00, 0x01]);
        let result = cpu.run();

        assert_eq!(result, ExitStatus::Brk);
        assert_eq!(cpu.a_register, 0x01);
    }

    #[test]
    fn test_lda_zero_page_x() {
        let mut cpu = Cpu::new();
        cpu.x_index_register = 0x01;
        cpu.memory[0..3].copy_from_slice(&[0xB5, 0xFE, 0x00]);
        cpu.memory[0xFF] = 0x01;
        let result = cpu.run();

        assert_eq!(result, ExitStatus::Brk);
        assert_eq!(cpu.a_register, 0x01);
    }

    #[test]
    fn test_lda_zero_page_x_overflow() {
        let mut cpu = Cpu::new();
        cpu.x_index_register = 0x04;
        cpu.memory[0..4].copy_from_slice(&[0xB5, 0xFF, 0x00, 0x01]);
        let result = cpu.run();

        assert_eq!(result, ExitStatus::Brk);
        assert_eq!(cpu.a_register, 0x01);
    }

    #[test]
    fn test_lda_absolute() {
        let mut cpu = Cpu::new();
        cpu.memory[0..4].copy_from_slice(&[0xAD, 0xFF, 0x07, 0x00]);
        cpu.memory[0x07FF] = 0x01;
        let result = cpu.run();

        assert_eq!(result, ExitStatus::Brk);
        assert_eq!(cpu.a_register, 0x01);
    }

    #[test]
    fn test_lda_absolute_x() {
        let mut cpu = Cpu::new();
        cpu.x_index_register = 0x01;
        cpu.memory[0..4].copy_from_slice(&[0xBD, 0xFE, 0x07, 0x00]);
        cpu.memory[0x07FF] = 0x01;
        let result = cpu.run();

        assert_eq!(result, ExitStatus::Brk);
        assert_eq!(cpu.a_register, 0x01);
    }

    // At the moment the cpu does not have access to
    // the full memory range, failing to test the overflow
    // case
    // #[test]
    // fn test_lda_absolute_x_overflow() {
    //     let mut cpu = Cpu::new();
    //     cpu.x_index_register = 0x05;
    //     cpu.memory[0..5].copy_from_slice(&[0xBD, 0xFF, 0x07, 0x00, 0x01]);
    //     let result = cpu.run();
    //
    //     assert_eq!(result, ExitStatus::Brk);
    //     assert_eq!(cpu.a_register, 0x01);
    // }

    #[test]
    fn test_lda_absolute_y() {
        let mut cpu = Cpu::new();
        cpu.y_index_register = 0x01;
        cpu.memory[0..4].copy_from_slice(&[0xB9, 0xFE, 0x07, 0x00]);
        cpu.memory[0x07FF] = 0x01;
        let result = cpu.run();

        assert_eq!(result, ExitStatus::Brk);
        assert_eq!(cpu.a_register, 0x01);
    }

    // At the moment the cpu does not have access to
    // the full memory range, failing to test the overflow
    // case
    // #[test]
    // fn test_lda_absolute_y_overflow() {
    //     let mut cpu = Cpu::new();
    //     cpu.y_index_register = 0x05;
    //     cpu.memory[0..5].copy_from_slice(&[0xB9, 0xFF, 0x07, 0x00, 0x01]);
    //     let result = cpu.run();
    //
    //     assert_eq!(result, ExitStatus::Brk);
    //     assert_eq!(cpu.a_register, 0x01);
    // }

    #[test]
    fn test_lda_indirect_x() {
        let mut cpu = Cpu::new();
        cpu.x_index_register = 0x01;
        cpu.memory[0..5].copy_from_slice(&[0xA1, 0x02, 0x00, 0xFF, 0x07]);
        cpu.memory[0x07FF] = 0x01;
        let result = cpu.run();

        assert_eq!(result, ExitStatus::Brk);
        assert_eq!(cpu.a_register, 0x01);
    }

    #[test]
    fn test_lda_indirect_x_overflow() {
        let mut cpu = Cpu::new();
        cpu.x_index_register = 0x04;
        cpu.memory[0..5].copy_from_slice(&[0xA1, 0xFF, 0x00, 0xFF, 0x07]);
        cpu.memory[0x07FF] = 0x01;
        let result = cpu.run();

        assert_eq!(result, ExitStatus::Brk);
        assert_eq!(cpu.a_register, 0x01);
    }

    #[test]
    fn test_lda_indirect_y() {
        let mut cpu = Cpu::new();
        cpu.y_index_register = 0x01;
        cpu.memory[0..5].copy_from_slice(&[0xB1, 0x03, 0x00, 0xFE, 0x07]);
        cpu.memory[0x07FF] = 0x01;
        let result = cpu.run();

        assert_eq!(result, ExitStatus::Brk);
        assert_eq!(cpu.a_register, 0x01);
    }

    // At the moment the cpu does not have access to
    // the full memory range, failing to test the overflow
    // case
    // #[test]
    // fn test_lda_indirect_y_overflow() {
    //     let mut cpu = Cpu::new();
    //     cpu.y_index_register = 0x06;
    //     cpu.memory[0..6].copy_from_slice(&[0xB1, 0x03, 0x00, 0xFF, 0x07, 0x01]);
    //     let result = cpu.run();
    //
    //     assert_eq!(result, ExitStatus::Brk);
    //     assert_eq!(cpu.a_register, 0x01);
    // }
}
