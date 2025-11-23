use crate::{
    cpu::{Cpu, Status},
    utils::BitsExt,
};

pub trait Instructions {
    fn lda(&mut self, value: u8);
}

impl Instructions for Cpu {
    fn lda(&mut self, value: u8) {
        self.a_register = value;

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));
    }
}
