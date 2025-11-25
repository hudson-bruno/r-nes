use crate::cpu::{
    instructions::lookup::INSTRUCTIONS_LOOKUP, memory::Memory, operand::OperandLocation,
};
use bitflags::bitflags;

pub mod addressing_modes;
pub mod instructions;
pub mod memory;
pub mod operand;

pub struct Cpu {
    pub memory: [u8; 2 * 1024],

    // Registers
    pub a_register: u8,
    pub status_register: Status,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub x_index_register: u8,
    pub y_index_register: u8,

    pub operand_location: OperandLocation,
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
    MissingOperand,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: [0; 2 * 1024],
            a_register: 0,
            status_register: Status::UNUSED,
            program_counter: 0,
            stack_pointer: 0xFF,
            x_index_register: 0,
            y_index_register: 0,
            operand_location: OperandLocation::Implicit,
        }
    }

    pub fn run(&mut self) -> ExitStatus {
        loop {
            if let Some(err) = self.clock() {
                return err;
            }
        }
    }

    pub fn clock(&mut self) -> Option<ExitStatus> {
        let op_code = self.read(self.program_counter);
        self.program_counter += 1;

        if let Some(op) = &INSTRUCTIONS_LOOKUP[op_code as usize] {
            self.operand_location = (op.addressing_mode)(self);

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
