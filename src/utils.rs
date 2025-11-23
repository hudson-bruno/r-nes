pub trait BitsExt {
    fn get_bit(&self, index: u8) -> bool;
}

impl BitsExt for u8 {
    fn get_bit(&self, index: u8) -> bool {
        self & (1 << index) != 0
    }
}
