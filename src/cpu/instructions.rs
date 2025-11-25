use crate::{
    cpu::{
        Cpu, ExitStatus, Status,
        memory::stack::Stack,
        operand::{Operand, OperandLocation, OperandValue},
    },
    utils::{BitFlagExt, BitsExt},
};

pub mod lookup;

pub trait Instructions {
    fn brk(&mut self) -> Option<ExitStatus>;
    fn ora(&mut self) -> Option<ExitStatus>;
    fn asl(&mut self) -> Option<ExitStatus>;
    fn php(&mut self) -> Option<ExitStatus>;
    fn bpl(&mut self) -> Option<ExitStatus>;
    fn clc(&mut self) -> Option<ExitStatus>;
    fn jsr(&mut self) -> Option<ExitStatus>;
    fn and(&mut self) -> Option<ExitStatus>;
    fn bit(&mut self) -> Option<ExitStatus>;
    fn rol(&mut self) -> Option<ExitStatus>;
    fn plp(&mut self) -> Option<ExitStatus>;
    fn bmi(&mut self) -> Option<ExitStatus>;
    fn sec(&mut self) -> Option<ExitStatus>;
    fn rti(&mut self) -> Option<ExitStatus>;
    fn eor(&mut self) -> Option<ExitStatus>;
    fn lsr(&mut self) -> Option<ExitStatus>;
    fn pha(&mut self) -> Option<ExitStatus>;
    fn jmp(&mut self) -> Option<ExitStatus>;
    fn bvc(&mut self) -> Option<ExitStatus>;
    fn cli(&mut self) -> Option<ExitStatus>;
    fn rts(&mut self) -> Option<ExitStatus>;
    fn adc(&mut self) -> Option<ExitStatus>;
    fn ror(&mut self) -> Option<ExitStatus>;
    fn pla(&mut self) -> Option<ExitStatus>;
    fn bvs(&mut self) -> Option<ExitStatus>;
    fn sei(&mut self) -> Option<ExitStatus>;
    fn sta(&mut self) -> Option<ExitStatus>;
    fn sty(&mut self) -> Option<ExitStatus>;
    fn stx(&mut self) -> Option<ExitStatus>;
    fn dey(&mut self) -> Option<ExitStatus>;
    fn txa(&mut self) -> Option<ExitStatus>;
    fn bcc(&mut self) -> Option<ExitStatus>;
    fn tya(&mut self) -> Option<ExitStatus>;
    fn txs(&mut self) -> Option<ExitStatus>;
    fn ldy(&mut self) -> Option<ExitStatus>;
    fn lda(&mut self) -> Option<ExitStatus>;
    fn ldx(&mut self) -> Option<ExitStatus>;
    fn tay(&mut self) -> Option<ExitStatus>;
    fn tax(&mut self) -> Option<ExitStatus>;
    fn bcs(&mut self) -> Option<ExitStatus>;
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
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        self.a_register |= operand;

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));

        None
    }

    fn asl(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        self.status_register.set(Status::CARRY, operand.get_bit(7));

        let result = operand << 1;
        self.update_operand(result);

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

    fn bpl(&mut self) -> Option<ExitStatus> {
        let OperandValue::I8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        if !self.status_register.contains(Status::NEGATIVE) {
            self.program_counter = self.program_counter.wrapping_add_signed(operand as i16);
        }

        None
    }

    fn clc(&mut self) -> Option<ExitStatus> {
        self.status_register.remove(Status::CARRY);

        None
    }

    fn jsr(&mut self) -> Option<ExitStatus> {
        let OperandLocation::Memory(addr) = self.operand_location else {
            return Some(ExitStatus::MissingOperand);
        };

        self.stack_push_address(self.program_counter);
        self.program_counter = addr;

        None
    }

    fn and(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        self.a_register &= operand;

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));

        None
    }

    fn bit(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        let result = self.a_register & operand;

        self.status_register.set(Status::ZERO, result == 0);
        self.status_register
            .set(Status::OVERFLOW, operand.get_bit(6));
        self.status_register
            .set(Status::NEGATIVE, operand.get_bit(7));

        None
    }

    fn rol(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        let result = (operand << 1) | (self.status_register.contains(Status::CARRY) as u8);
        self.update_operand(result);

        self.status_register.set(Status::CARRY, operand.get_bit(7));
        self.status_register.set(Status::ZERO, result == 0);
        self.status_register
            .set(Status::NEGATIVE, result.get_bit(7));

        None
    }

    fn plp(&mut self) -> Option<ExitStatus> {
        let status_in_stack = self.stack_pop();
        self.status_register.update_with_except(
            Status::from_bits_retain(status_in_stack),
            Status::UNUSED | Status::BREAK,
        );

        None
    }

    fn bmi(&mut self) -> Option<ExitStatus> {
        let OperandValue::I8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        if self.status_register.contains(Status::NEGATIVE) {
            self.program_counter = self.program_counter.wrapping_add_signed(operand as i16);
        }

        None
    }

    fn sec(&mut self) -> Option<ExitStatus> {
        self.status_register.insert(Status::CARRY);

        None
    }

    fn rti(&mut self) -> Option<ExitStatus> {
        let status_in_stack = self.stack_pop();
        let program_counter_in_stack = self.stack_pop_address();

        self.status_register.update_with_except(
            Status::from_bits_retain(status_in_stack),
            Status::UNUSED | Status::BREAK,
        );
        self.program_counter = program_counter_in_stack;

        None
    }

    fn eor(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        self.a_register ^= operand;

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));

        None
    }

    fn lsr(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        let result = operand >> 1;
        self.update_operand(result);

        self.status_register.set(Status::CARRY, operand.get_bit(0));
        self.status_register.set(Status::ZERO, result == 0);
        self.status_register.remove(Status::NEGATIVE);

        None
    }

    fn pha(&mut self) -> Option<ExitStatus> {
        self.stack_push(self.a_register);

        None
    }

    fn jmp(&mut self) -> Option<ExitStatus> {
        let OperandLocation::Memory(addr) = self.operand_location else {
            return Some(ExitStatus::MissingOperand);
        };

        self.program_counter = addr;

        None
    }

    fn bvc(&mut self) -> Option<ExitStatus> {
        let OperandValue::I8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        if !self.status_register.contains(Status::OVERFLOW) {
            self.program_counter = self.program_counter.wrapping_add_signed(operand as i16);
        }

        None
    }

    fn cli(&mut self) -> Option<ExitStatus> {
        self.status_register.remove(Status::INTERRUPT);

        None
    }

    fn rts(&mut self) -> Option<ExitStatus> {
        let program_counter_in_stack = self.stack_pop_address();

        self.program_counter = program_counter_in_stack + 1;

        None
    }

    fn adc(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        let result: u16 = self.a_register as u16
            + operand as u16
            + (self.status_register.contains(Status::CARRY) as u16);
        let result_u8 = result as u8;

        self.status_register.set(Status::CARRY, result > 0xFF);
        self.status_register.set(Status::ZERO, result_u8 == 0);
        self.status_register.set(
            Status::OVERFLOW,
            ((result_u8 ^ self.a_register) & (result_u8 ^ operand) & 0x80) != 0,
        );
        self.status_register
            .set(Status::NEGATIVE, result_u8.get_bit(7));

        self.a_register = result_u8;

        None
    }

    fn ror(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        let result = (operand >> 1) | ((self.status_register.contains(Status::CARRY) as u8) << 7);
        self.update_operand(result);

        self.status_register.set(Status::CARRY, operand.get_bit(0));
        self.status_register.set(Status::ZERO, result == 0);
        self.status_register
            .set(Status::NEGATIVE, result.get_bit(7));

        None
    }

    fn pla(&mut self) -> Option<ExitStatus> {
        self.a_register = self.stack_pop();

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));

        None
    }

    fn bvs(&mut self) -> Option<ExitStatus> {
        let OperandValue::I8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        if self.status_register.contains(Status::OVERFLOW) {
            self.program_counter = self.program_counter.wrapping_add_signed(operand as i16);
        }

        None
    }

    fn sei(&mut self) -> Option<ExitStatus> {
        self.status_register.insert(Status::INTERRUPT);

        None
    }

    fn sta(&mut self) -> Option<ExitStatus> {
        self.update_operand(self.a_register);

        None
    }

    fn sty(&mut self) -> Option<ExitStatus> {
        self.update_operand(self.y_index_register);

        None
    }

    fn stx(&mut self) -> Option<ExitStatus> {
        self.update_operand(self.x_index_register);

        None
    }

    fn dey(&mut self) -> Option<ExitStatus> {
        self.y_index_register = self.y_index_register.wrapping_sub(1);

        self.status_register
            .set(Status::ZERO, self.y_index_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.y_index_register.get_bit(7));

        None
    }

    fn txa(&mut self) -> Option<ExitStatus> {
        self.a_register = self.x_index_register;

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));

        None
    }

    fn bcc(&mut self) -> Option<ExitStatus> {
        let OperandValue::I8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        if !self.status_register.contains(Status::CARRY) {
            self.program_counter = self.program_counter.wrapping_add_signed(operand as i16);
        }

        None
    }

    fn tya(&mut self) -> Option<ExitStatus> {
        self.a_register = self.y_index_register;

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));

        None
    }

    fn txs(&mut self) -> Option<ExitStatus> {
        self.stack_pointer = self.x_index_register;

        None
    }

    fn ldy(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        self.y_index_register = operand;

        self.status_register
            .set(Status::ZERO, self.y_index_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.y_index_register.get_bit(7));

        None
    }

    fn lda(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        self.a_register = operand;

        self.status_register.set(Status::ZERO, self.a_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.a_register.get_bit(7));

        None
    }

    fn ldx(&mut self) -> Option<ExitStatus> {
        let OperandValue::U8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        self.x_index_register = operand;

        self.status_register
            .set(Status::ZERO, self.x_index_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.x_index_register.get_bit(7));

        None
    }

    fn tay(&mut self) -> Option<ExitStatus> {
        self.y_index_register = self.a_register;

        self.status_register
            .set(Status::ZERO, self.y_index_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.y_index_register.get_bit(7));

        None
    }

    fn tax(&mut self) -> Option<ExitStatus> {
        self.x_index_register = self.a_register;

        self.status_register
            .set(Status::ZERO, self.x_index_register == 0);
        self.status_register
            .set(Status::NEGATIVE, self.x_index_register.get_bit(7));

        None
    }

    fn bcs(&mut self) -> Option<ExitStatus> {
        let OperandValue::I8(operand) = self.get_operand() else {
            return Some(ExitStatus::MissingOperand);
        };

        if self.status_register.contains(Status::CARRY) {
            self.program_counter = self.program_counter.wrapping_add_signed(operand as i16);
        }

        None
    }
}
