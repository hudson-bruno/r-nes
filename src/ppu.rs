use crate::{
    cartridge::Cartridge,
    ppu::{
        fields::{PpuCtrl, PpuMask, PpuStatus},
        memory::PpuMemory,
    },
};

pub mod fields;
pub mod memory;

pub struct Ppu {
    pub ppu_vram: [[u8; 1024]; 2],

    pub ppu_ctrl: PpuCtrl,
    pub ppu_mask: PpuMask,
    pub ppu_status: PpuStatus,
    pub oam_addr: u8,
    pub oam_data: [u8; 256],
    pub ppu_data_buffer: u8,
    pub oam_dma: u8,

    pub v: u16,
    pub t: u16,
    pub x: u8,
    pub w: bool,

    // Cycle management
    pub cycle: i16,
    pub scanline: i16,

    // Background rendering
    pub bg_next_tile_id: u8,
    pub bg_next_tile_attr: u8,
    pub bg_next_tile_lsb: u8,
    pub bg_next_tile_msb: u8,
    pub bg_shifter_pattern_lo: u16,
    pub bg_shifter_pattern_hi: u16,
    pub bg_shifter_attrib_lo: u16,
    pub bg_shifter_attrib_hi: u16,

    pub palette: [u8; 64],
    pub palette_ram: [u8; 32],
    pub frame_data: [[u8; 256]; 240],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            ppu_vram: [[0; 1024]; 2],
            ppu_ctrl: PpuCtrl::empty(),
            ppu_mask: PpuMask::empty(),
            ppu_status: PpuStatus::empty(),
            oam_addr: 0,
            oam_data: [0; 256],
            ppu_data_buffer: 0,
            oam_dma: 0,
            v: 0,
            t: 0,
            x: 0,
            w: false,
            cycle: 0,
            scanline: 0,
            bg_next_tile_id: 0,
            bg_next_tile_attr: 0,
            bg_next_tile_lsb: 0,
            bg_next_tile_msb: 0,
            bg_shifter_pattern_lo: 0,
            bg_shifter_pattern_hi: 0,
            bg_shifter_attrib_lo: 0,
            bg_shifter_attrib_hi: 0,
            palette: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
                0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29,
                0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
                0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
            ],
            palette_ram: [0; 32],
            frame_data: [[0; 256]; 240],
        }
    }

    pub fn step(&mut self, mut cartridge: Option<&mut Cartridge>) -> bool {
        let mut nmi = false;

        if self.scanline >= -1 && self.scanline < 240 {
            if self.scanline == 0 && self.cycle == 0 {
                self.cycle = 1;
            }

            if self.scanline == -1 && self.cycle == 1 {
                self.ppu_status.remove(PpuStatus::VBLANK);
                self.ppu_status.remove(PpuStatus::SPRITE_OVERFLOW); // TODO: Check constants
                self.ppu_status.remove(PpuStatus::SPRITE_ZERO_HIT); // TODO: Check constants
            }

            if (self.cycle >= 1 && self.cycle <= 256) && (self.scanline >= 0 && self.scanline < 240) {
                let background_pixel: u8 = {
                    if self.ppu_mask.contains(PpuMask::RENDER_BACKGROUND) {
                        let bit_mux = 0x8000 >> self.x;
                        let p0_pixel = (self.bg_shifter_pattern_lo & bit_mux) > 0;
                        let p1_pixel = (self.bg_shifter_pattern_hi & bit_mux) > 0;
                        ((p1_pixel as u8) << 1) | (p0_pixel as u8)
                    } else {
                        0
                    }
                };

                let background_palette: u8 = {
                    if self.ppu_mask.contains(PpuMask::RENDER_BACKGROUND) {
                        let bit_mux = 0x8000 >> self.x;
                        let p0_palette = (self.bg_shifter_attrib_lo & bit_mux) > 0;
                        let p1_palette = (self.bg_shifter_attrib_hi & bit_mux) > 0;
                        ((p1_palette as u8) << 1) | (p0_palette as u8)
                    } else {
                        0
                    }
                };

                // The final palette index is formed by combining the pixel color bits and the palette attribute bits.
                let final_palette_entry = (background_palette << 2) | background_pixel;
                let color = self.read_palette(final_palette_entry as u16);

                self.frame_data[self.scanline as usize][(self.cycle - 1) as usize] = color;
            }

            if (self.cycle >= 2 && self.cycle < 258) || (self.cycle >= 321 && self.cycle < 338) {
                self.update_shifters();

                match (self.cycle - 1) % 8 {
                    0 => {
                        self.load_background_shifters();
                        self.bg_next_tile_id =
                            self.read(0x2000 | (self.v & 0x0FFF), cartridge.as_deref_mut());
                    }
                    2 => {
                        self.bg_next_tile_attr = self.read(
                            0x23C0
                                | (self.v & 0x0C00)
                                | ((self.v >> 4) & 0x38)
                                | ((self.v >> 2) & 0x07),
                            cartridge.as_deref_mut(),
                        );
                        if (self.v & 0x40) != 0 {
                            self.bg_next_tile_attr >>= 4;
                        }
                        if (self.v & 0x02) != 0 {
                            self.bg_next_tile_attr >>= 2;
                        }
                        self.bg_next_tile_attr &= 0x03;
                    }
                    4 => {
                        let addr = self.ppu_ctrl.bg_pattern_addr()
                            + (self.bg_next_tile_id as u16 * 16)
                            + ((self.v >> 12) & 0x07);
                        self.bg_next_tile_lsb = self.read(addr, cartridge.as_deref_mut());
                    }
                    6 => {
                        let addr = self.ppu_ctrl.bg_pattern_addr()
                            + (self.bg_next_tile_id as u16 * 16)
                            + ((self.v >> 12) & 0x07)
                            + 8;
                        self.bg_next_tile_msb = self.read(addr, cartridge.as_deref_mut());
                    }
                    7 => {
                        self.increment_scroll_x();
                    }
                    _ => {}
                }
            }

            if self.cycle == 256 {
                self.increment_scroll_y();
            }

            if self.cycle == 257 {
                self.load_background_shifters();
                self.transfer_address_x();
            }

            if self.cycle == 338 || self.cycle == 340 {
                self.bg_next_tile_id =
                    self.read(0x2000 | (self.v & 0x0FFF), cartridge.as_deref_mut());
            }

            if self.scanline == -1 && self.cycle >= 280 && self.cycle < 305 {
                self.transfer_address_y();
            }
        }

        if self.scanline == 241 && self.cycle == 1 {
            self.ppu_status.insert(PpuStatus::VBLANK);
            if self.ppu_ctrl.contains(PpuCtrl::VBLANK_NMI) {
                nmi = true;
            }
        }

        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 261 {
                self.scanline = -1;
            }
        }

        nmi
    }

    pub fn read_ppu_status(&mut self) -> u8 {
        self.w = false;

        let result = self.ppu_status;
        self.ppu_status.remove(PpuStatus::VBLANK);

        result.bits()
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }

    pub fn read_ppu_data(&mut self, cartridge: Option<&mut Cartridge>) -> u8 {
        let new_value = self.read(self.v, cartridge);

        let result = if self.v >= 0x3F00 {
            new_value
        } else {
            self.ppu_data_buffer
        };

        self.ppu_data_buffer = new_value;
        self.v += self.ppu_ctrl.vram_increment();

        result
    }

    pub fn write_ppu_ctrl(&mut self, value: PpuCtrl) {
        // TODO: After power/reset, writes to this register are ignored until the first pre-render scanline.
        // WARN: Ignore bit 0 race condition https://www.nesdev.org/wiki/PPU_registers#Bit_0_race_condition
        let vblank_just_enabled =
            !self.ppu_ctrl.contains(PpuCtrl::VBLANK_NMI) && value.contains(PpuCtrl::VBLANK_NMI);

        self.ppu_ctrl = value - PpuCtrl::MASTER_SLAVE;
        self.ppu_ctrl.transfer_nametable_to_t(&mut self.t);

        if vblank_just_enabled && self.ppu_status.contains(PpuStatus::VBLANK) {
            // TODO: Trigger nmi
        }
    }

    pub fn write_ppu_mask(&mut self, value: PpuMask) {
        // TODO: After power/reset, writes to this register are ignored until the first pre-render scanline.
        // WARN: To avoid numerous hardware bugs and limitations, it is generally recommended that rendering
        // be turned on or off only during vblank. This can be done by writing the desired PPUMASK value
        // to a variable rather than the register itself and then only copying that variable to PPUMASK
        // during vblank in the NMI handler.
        self.ppu_mask = value;
    }

    pub fn write_oam_addr(&mut self, value: u8) {
        // TODO: In progress...
        self.oam_addr = value;
    }

    pub fn write_oam_data(&mut self, value: u8) {
        // TODO: In progress...
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn write_ppu_scroll(&mut self, value: u8) {
        match self.w {
            false => {
                self.x = value & 0b0000_0111;

                let coarse_x = (value >> 3) as u16;
                self.t &= 0b1111_1111_1110_0000;
                self.t |= coarse_x;

                self.w = true;
            }
            true => {
                let fine_y = (value & 0b0000_0111) as u16;
                self.t &= 0b1000_1111_1111_1111;
                self.t |= fine_y << 12;

                let coarse_y = (value >> 3) as u16;
                self.t &= 0b1111_1100_0001_1111;
                self.t |= coarse_y << 5;

                self.w = false;
            }
        }
    }

    pub fn write_ppu_addr(&mut self, value: u8) {
        match self.w {
            false => {
                self.t &= 0b0000_0000_1111_1111;
                self.t |= ((value & 0b0011_1111) as u16) << 8;

                self.w = true;
            }
            true => {
                self.t &= 0b1111_1111_0000_0000;
                self.t |= value as u16;

                self.v = self.t;
                self.w = false;
            }
        }
    }

    pub fn write_ppu_data(&mut self, value: u8, cartridge: Option<&mut Cartridge>) {
        self.write(self.v, value, cartridge);

        self.v += self.ppu_ctrl.vram_increment();
    }

    pub fn read_palette(&self, addr: u16) -> u8 {
        let mut relative_addr = addr & 0x1F;

        relative_addr = match relative_addr {
            0x10 | 0x14 | 0x18 | 0x1C => relative_addr - 0x10,
            _ => relative_addr,
        };

        let mut result = self.palette_ram[relative_addr as usize];
        if self.ppu_mask.contains(PpuMask::GREYSCALE) {
            result &= 0x30;
        }

        result
    }

    pub fn write_palette(&mut self, addr: u16, value: u8) {
        let mut relative_addr = addr & 0x1F;

        relative_addr = match relative_addr {
            0x10 | 0x14 | 0x18 | 0x1C => relative_addr - 0x10,
            _ => relative_addr,
        };

        self.palette_ram[relative_addr as usize] = value;
    }

    pub fn cpu_read(&mut self, addr: u16, cartridge: Option<&mut Cartridge>) -> u8 {
        let relative_addr = addr & 0x07;

        match relative_addr {
            0x0002 => self.read_ppu_status(),
            0x0004 => self.read_oam_data(),
            0x0007 => self.read_ppu_data(cartridge),
            _ => (addr >> 8) as u8, // Open bus
        }
    }

    pub fn cpu_write(&mut self, addr: u16, value: u8, cartridge: Option<&mut Cartridge>) {
        let relative_addr = addr & 0x07;

        match relative_addr {
            0x0000 => self.write_ppu_ctrl(PpuCtrl::from_bits_retain(value)),
            0x0001 => self.write_ppu_mask(PpuMask::from_bits_retain(value)),
            0x0003 => self.write_oam_addr(value),
            0x0004 => self.write_oam_data(value),
            0x0005 => self.write_ppu_scroll(value),
            0x0006 => self.write_ppu_addr(value),
            0x0007 => self.write_ppu_data(value, cartridge),
            _ => (),
        }
    }
    fn increment_scroll_x(&mut self) {
        if self.ppu_mask.contains(PpuMask::RENDER_BACKGROUND)
            || self.ppu_mask.contains(PpuMask::RENDER_SPRITE)
        {
            if (self.v & 0x001F) == 31 {
                self.v &= !0x001F;
                self.v ^= 0x0400;
            } else {
                self.v += 1;
            }
        }
    }

    fn increment_scroll_y(&mut self) {
        if self.ppu_mask.contains(PpuMask::RENDER_BACKGROUND)
            || self.ppu_mask.contains(PpuMask::RENDER_SPRITE)
        {
            if (self.v & 0x7000) != 0x7000 {
                self.v += 0x1000;
            } else {
                self.v &= !0x7000;
                let mut y = (self.v & 0x03E0) >> 5;
                if y == 29 {
                    y = 0;
                    self.v ^= 0x0800;
                } else if y == 31 {
                    y = 0;
                } else {
                    y += 1;
                }
                self.v = (self.v & !0x03E0) | (y << 5);
            }
        }
    }

    fn transfer_address_x(&mut self) {
        if self.ppu_mask.contains(PpuMask::RENDER_BACKGROUND)
            || self.ppu_mask.contains(PpuMask::RENDER_SPRITE)
        {
            self.v = (self.v & !0x041F) | (self.t & 0x041F);
        }
    }

    fn transfer_address_y(&mut self) {
        if self.ppu_mask.contains(PpuMask::RENDER_BACKGROUND)
            || self.ppu_mask.contains(PpuMask::RENDER_SPRITE)
        {
            self.v = (self.v & !0x7BE0) | (self.t & 0x7BE0);
        }
    }
    fn load_background_shifters(&mut self) {
        self.bg_shifter_pattern_lo =
            (self.bg_shifter_pattern_lo & 0xFF00) | self.bg_next_tile_lsb as u16;
        self.bg_shifter_pattern_hi =
            (self.bg_shifter_pattern_hi & 0xFF00) | self.bg_next_tile_msb as u16;

        self.bg_shifter_attrib_lo = (self.bg_shifter_attrib_lo & 0xFF00)
            | if (self.bg_next_tile_attr & 0b01) != 0 {
                0xFF
            } else {
                0x00
            };
        self.bg_shifter_attrib_hi = (self.bg_shifter_attrib_hi & 0xFF00)
            | if (self.bg_next_tile_attr & 0b10) != 0 {
                0xFF
            } else {
                0x00
            };
    }

    fn update_shifters(&mut self) {
        if self.ppu_mask.contains(PpuMask::RENDER_BACKGROUND) {
            self.bg_shifter_pattern_lo <<= 1;
            self.bg_shifter_pattern_hi <<= 1;
            self.bg_shifter_attrib_lo <<= 1;
            self.bg_shifter_attrib_hi <<= 1;
        }
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
