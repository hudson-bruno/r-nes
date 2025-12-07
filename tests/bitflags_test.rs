#[cfg(test)]
mod tests {
    use super::*;
    use bitflags::bitflags;

    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct PpuCtrl: u8 {
            const NAMETABLE_LOW  = 1 << 0;
            const NAMETABLE_HIGH = 1 << 1;
            const NAMETABLE_MASK = 0b0000_0011;
            const VRAM_INCREMENT = 1 << 2;
            const SPRITE_PATTERN = 1 << 3;
            const BG_PATTERN     = 1 << 4;
            const SPRITE_SIZE    = 1 << 5;
            const MASTER_SLAVE   = 1 << 6;
            const VBLANK_NMI     = 1 << 7;
        }
    }

    #[test]
    fn test_ppu_ctrl_subtraction() {
        // Case 1: Value has MASTER_SLAVE set
        let val = PpuCtrl::VBLANK_NMI | PpuCtrl::MASTER_SLAVE; // 0x80 | 0x40 = 0xC0
        let result = val - PpuCtrl::MASTER_SLAVE;
        assert_eq!(result, PpuCtrl::VBLANK_NMI); // Should be 0x80
        assert!(!result.contains(PpuCtrl::MASTER_SLAVE));
        assert!(result.contains(PpuCtrl::VBLANK_NMI));

        // Case 2: Value does NOT have MASTER_SLAVE set
        let val = PpuCtrl::VBLANK_NMI; // 0x80
        let result = val - PpuCtrl::MASTER_SLAVE;
        
        // If subtraction is arithmetic: 0x80 - 0x40 = 0x40 (MASTER_SLAVE set, VBLANK_NMI lost!)
        // If subtraction is set difference: 0x80 & !0x40 = 0x80 (Correct)
        
        assert_eq!(result, PpuCtrl::VBLANK_NMI, "Set difference failed, acts like arithmetic?");
        assert!(result.contains(PpuCtrl::VBLANK_NMI));
    }
}
