use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PpuCtrl: u8 {
        const NAMETABLE_LOW  = 1 << 0;
        const NAMETABLE_HIGH = 1 << 1;
        const NAMETABLE_MASK = 0b0000_0011;

        const X_SCROLL  = 1 << 0;
        const Y_SCROLL = 1 << 1;

        const VRAM_INCREMENT = 1 << 2;
        const SPRITE_PATTERN = 1 << 3;
        const BG_PATTERN     = 1 << 4;
        const SPRITE_SIZE    = 1 << 5;
        const MASTER_SLAVE   = 1 << 6;
        const VBLANK_NMI     = 1 << 7;

    }
}

impl PpuCtrl {
    pub fn transfer_nametable_to_t(&self, t: &mut u16) {
        let nametable_select = self.nametable_value();

        *t &= !0b1111_0011_1111_1111;
        *t |= nametable_select << 10;
    }

    pub fn nametable_value(&self) -> u16 {
        (self.bits() & Self::NAMETABLE_MASK.bits()) as u16
    }

    pub fn nametable_addr(&self) -> u16 {
        let val = self.nametable_value();
        0x2000 + (val * 0x400)
    }

    pub fn vram_increment(&self) -> u16 {
        if self.contains(Self::VRAM_INCREMENT) {
            32
        } else {
            1
        }
    }

    pub fn sprite_pattern_addr(&self) -> u16 {
        if self.contains(Self::SPRITE_PATTERN) {
            0x1000
        } else {
            0x0000
        }
    }

    pub fn bg_pattern_addr(&self) -> u16 {
        if self.contains(Self::BG_PATTERN) {
            0x1000
        } else {
            0x0000
        }
    }

    pub fn sprite_height(&self) -> u8 {
        if self.contains(Self::SPRITE_SIZE) {
            16
        } else {
            8
        }
    }

    pub fn x_scroll_position(&self) -> u16 {
        if self.contains(Self::X_SCROLL) {
            256
        } else {
            0
        }
    }

    pub fn y_scroll_position(&self) -> u16 {
        if self.contains(Self::Y_SCROLL) {
            240
        } else {
            0
        }
    }
}
