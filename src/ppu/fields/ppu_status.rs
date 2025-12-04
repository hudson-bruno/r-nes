use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PpuStatus: u8 {
        const SPRITE_OVERFLOW = 1 << 5;
        const SPRITE_ZERO_HIT = 1 << 6;
        const VBLANK          = 1 << 7;
    }
}
