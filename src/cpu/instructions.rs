use crate::{
    cpu::{
        Cpu, ExitStatus, Status,
        memory::{Memory, stack::Stack},
    },
    utils::BitsExt,
};

pub mod lookup;

pub trait Instructions {
    fn brk(&mut self) -> Option<ExitStatus>;
    fn ora(&mut self) -> Option<ExitStatus>;
    fn asl(&mut self) -> Option<ExitStatus>;
    fn php(&mut self) -> Option<ExitStatus>;
    fn lda(&mut self) -> Option<ExitStatus>;
}

impl Instructions for Cpu {
    fn brk(&mut self) -> Option<ExitStatus> {
        self.stack_push_address(self.program_counter);
        self.stack_push(self.status_register.union(Status::BREAK).bits());

        self.status_register.insert(Status::INTERRUPT);
        self.program_counter = 0x0000;

        Some(ExitStatus::Brk)
    }

    fn ora(&mut self) -> Option<ExitStatus> {
        self.a_register |= self.op_memory;

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));

        None
    }

    fn asl(&mut self) -> Option<ExitStatus> {
        self.status_register
            .set(Status::CARRY, self.op_memory.get_bit(7));

        let result = self.op_memory << 1;
        self.write(self.op_memory_address, result);

        self.status_register.set(Status::ZERO, result == 0);
        self.status_register
            .set(Status::NEGATIVE, result.get_bit(7));

        None
    }

    fn php(&mut self) -> Option<ExitStatus> {
        let status_to_stack = self.status_register.union(Status::BREAK | Status::UNUSED);
        self.stack_push(status_to_stack.bits());

        None
    }

    fn lda(&mut self) -> Option<ExitStatus> {
        self.a_register = self.op_memory;

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));

        None
    }
}
