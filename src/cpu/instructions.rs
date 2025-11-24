use crate::{
    cpu::{Cpu, ExitStatus, Status},
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
        Some(ExitStatus::Brk)
    }
}
