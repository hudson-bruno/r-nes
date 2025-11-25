use crate::cpu::Status;

pub trait BitsExt {
    fn get_bit(&self, index: u8) -> bool;
}

impl BitsExt for u8 {
    fn get_bit(&self, index: u8) -> bool {
        self & (1 << index) != 0
    }
}

pub trait BitFlagExt {
    fn update_with_except(&mut self, new_value: Self, mask: Self);
}

impl BitFlagExt for Status {
    fn update_with_except(&mut self, new_value: Self, mask: Self) {
        *self = (new_value & !mask) | (*self & mask);
    }
}
