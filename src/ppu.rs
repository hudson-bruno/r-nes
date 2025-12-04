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

    pub palette: [u8; 64],
    pub palette_ram: [u8; 32],
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
            palette: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
                0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29,
                0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
                0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
            ],
            palette_ram: [0; 32],
        }
    }

    pub fn step(&mut self) -> bool {
        let mut nmi = false;
        for scanline in 0..=262 {
            for cycle in 0..=341 {
                if scanline == 241 && cycle == 1 {
                    self.ppu_status.insert(PpuStatus::VBLANK);
                    if self.ppu_ctrl.contains(PpuCtrl::VBLANK_NMI) {
                        nmi = true;
                    }
                }
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
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
