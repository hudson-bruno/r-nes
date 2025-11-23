use crate::cpu::Cpu;

pub trait Instructions {
    fn lda(&mut self, value: u8);
}

impl Instructions for Cpu {
    fn lda(&mut self, value: u8) {
        self.a_register = value;

        if self.a_register == 0 {
            self.status_register |= 0b0000_0010
        } else {
            self.status_register &= 0b1111_1101
        }

        if self.a_register & 0b1000_0000 != 0 {
            self.status_register |= 0b1000_0000
        } else {
            self.status_register &= 0b0111_1111
        }
    }
}
