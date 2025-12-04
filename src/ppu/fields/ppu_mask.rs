use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PpuMask: u8 {
        const GREYSCALE                  = 1 << 0;
        const SHOW_BACKGROUND_LEFTMOST_8 = 1 << 1;
        const SHOW_SPRITE_LEFTMOST_8     = 1 << 2;
        const RENDER_BACKGROUND          = 1 << 3;
        const RENDER_SPRITE              = 1 << 4;
        const EMPHASIZE_RED              = 1 << 5;
        const EMPHASIZE_GREEN            = 1 << 6;
        const EMPHASIZE_BLUE             = 1 << 7;
    }
}
