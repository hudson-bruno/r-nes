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
}
