use crate::ppu::fields::{PpuCtrl, PpuMask, PpuStatus};

pub mod fields;
pub mod memory;

pub struct Ppu {
    pub ppu_vram: [u8; 2 * 1024],

    pub ppu_ctrl: PpuCtrl,
    pub ppu_mask: PpuMask,
    pub ppu_status: PpuStatus,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_scroll: u8,
    pub ppu_addr: u16,
    pub ppu_data: u8,
    pub ppu_data_buffer: u8,
    pub oam_dma: u8,

    pub v: u16,
    pub t: u16,
    pub x: u8,
    pub w: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            ppu_vram: [0; 2 * 1024],
            ppu_ctrl: PpuCtrl::empty(),
            ppu_mask: PpuMask::empty(),
            ppu_status: PpuStatus::empty(),
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,
            ppu_data_buffer: 0,
            oam_dma: 0,
            v: 0,
            t: 0,
            x: 0,
            w: false,
        }
    }

    pub fn step(&self) -> bool {
        false
    }

    pub fn read_ppu_status(&mut self) -> u8 {
        self.w = false;

        let result = self.ppu_status;
        self.ppu_status.remove(PpuStatus::VBLANK);

        result.bits()
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data
    }

    pub fn read_ppu_data(&mut self) -> u8 {
        let result = if self.ppu_addr >= 0x3F00 {
            self.ppu_data
        } else {
            self.ppu_data_buffer
        };
        // self.ppu_data_buffer = self.ppu_read(self.ppu_addr)
        self.ppu_addr += self.ppu_ctrl.vram_increment();

        // result
        result
    }

    pub fn write_ppu_ctrl(&mut self, value: PpuCtrl) {
        // TODO: After power/reset, writes to this register are ignored until the first pre-render scanline.
        // WARN: Ignore bit 0 race condition https://www.nesdev.org/wiki/PPU_registers#Bit_0_race_condition
        let vblank_just_enabled =
            !self.ppu_ctrl.contains(PpuCtrl::VBLANK_NMI) && value.contains(PpuCtrl::VBLANK_NMI);

        self.ppu_ctrl = value - PpuCtrl::MASTER_SLAVE;

        if vblank_just_enabled && self.ppu_status.contains(PpuStatus::VBLANK) {
            // TODO: Trigger nmi
        }

        self.ppu_ctrl.transfer_nametable_to_t(&mut self.t);
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
        // oamaddr write
        self.oam_addr = value;
    }

    pub fn write_oam_data(&mut self, value: u8) {
        // oamdata read/write
        self.oam_data = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn write_ppu_scroll(&mut self, value: u8) {
        // ppuscroll write 2x
        //
        match self.w {
            true => {
                self.w = false;
            }
            false => {
                self.w = true;
            }
        }
    }

    pub fn write_ppu_addr(&mut self, value: u8) {
        // ppuaddr write 2x
        match self.w {
            true => {
                self.w = false;
                // high byte
            }
            false => {
                self.w = true;
                // low byte
            }
        }
    }

    pub fn write_ppu_data(&mut self, value: u8) {
        // ppudata read/write
        self.ppu_data = value;
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
