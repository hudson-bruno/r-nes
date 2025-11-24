use crate::{
    cpu::{Cpu, ExitStatus, Status, memory::stack::Stack},
    utils::BitsExt,
};

pub mod lookup;

pub trait Instructions {
    fn lda(&mut self) -> Option<ExitStatus>;
    fn brk(&mut self) -> Option<ExitStatus>;
}

impl Instructions for Cpu {
    fn lda(&mut self) -> Option<ExitStatus> {
        self.a_register = self.op_memory;

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));

        None
    }

    fn brk(&mut self) -> Option<ExitStatus> {
        self.stack_push_address(self.program_counter);
        self.stack_push(self.status_register.union(Status::BREAK).bits());

        self.status_register.insert(Status::INTERRUPT);
        self.program_counter = 0x0000;

        Some(ExitStatus::Brk)
    }
}
