use crate::{
    cpu::memory::Memory,
    ppu::{Ppu, PpuCtrl, PpuMask},
};

impl Memory for Ppu {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0002 => self.read_ppu_status(),
            0x0004 => self.read_oam_data(),
            0x0007 => self.read_ppu_data(),
            _ => (addr >> 8) as u8, // Open bus
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000 => self.write_ppu_ctrl(PpuCtrl::from_bits_retain(value)),
            0x0001 => self.write_ppu_mask(PpuMask::from_bits_retain(value)),
            0x0003 => self.write_oam_addr(value),
            0x0004 => self.write_oam_data(value),
            0x0005 => self.write_ppu_scroll(value),
            0x0006 => self.write_ppu_addr(value),
            0x0007 => self.write_ppu_data(value),
            _ => (),
        }
    }
}
