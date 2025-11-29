use thiserror::Error;

use crate::utils::BitsExt;

const NES_IDENTIFICATION: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

pub struct INesHeader {
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
    pub nametable_arrangement: bool,
    pub has_persistent_memory: bool,
    pub has_trainer: bool,
    pub alternative_nametable_layout: bool,
    pub console_type: u8,
    pub is_nes_2: bool,
    pub mapper: u8,
    pub prg_ram_size: u8,
}

impl TryFrom<[u8; 16]> for INesHeader {
    type Error = INesHeaderError;

    fn try_from(bytes: [u8; 16]) -> Result<Self, Self::Error> {
        if NES_IDENTIFICATION != bytes[0..4] {
            return Err(INesHeaderError::WrongIdentification);
        }

        let is_nes_2 = (bytes[7] & 0b0000_1100) >> 2 == 2;

        if is_nes_2 {
            return Err(INesHeaderError::Nes2NotSupported);
        }

        Ok(Self {
            prg_rom_size: bytes[4],
            chr_rom_size: bytes[5],
            nametable_arrangement: bytes[6].get_bit(0),
            has_persistent_memory: bytes[6].get_bit(1),
            has_trainer: bytes[6].get_bit(2),
            alternative_nametable_layout: bytes[6].get_bit(3),
            console_type: bytes[7] & 0b0000_0011,
            is_nes_2,
            mapper: (bytes[7] & 0b1111_0000) | (bytes[6] >> 4),
            prg_ram_size: bytes[8],
        })
    }
}

#[derive(Error, Debug)]
pub enum INesHeaderError {
    #[error("wrong identification")]
    WrongIdentification,

    #[error("NES 2.0 rom not supported")]
    Nes2NotSupported,
}
