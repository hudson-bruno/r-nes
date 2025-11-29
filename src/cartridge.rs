use std::path::Path;

use thiserror::Error;

use crate::{
    cpu::memory::Memory,
    ines::{INes, INesError},
    mapper::{Mapper, mapper_000::Mapper000},
};

pub struct Cartridge {
    pub program_memory: Vec<u8>,
    pub character_memory: Vec<u8>,

    pub mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            program_memory: vec![0; 32 * 1024],
            character_memory: vec![0; 8 * 1024],
            mapper: Box::new(Mapper000 {
                program_memory_banks: 2,
            }),
        }
    }

    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, INesError> {
        let ines = INes::load_from_path(path)?;

        Ok(ines.try_into().expect("mapper not supported"))
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<INes> for Cartridge {
    type Error = INesToCartridgeError;

    fn try_from(ines: INes) -> Result<Self, Self::Error> {
        if ines.header.mapper != 0 {
            return Err(INesToCartridgeError::MapperNotSupported(ines.header.mapper));
        }

        Ok(Self {
            program_memory: ines.prg_rom,
            character_memory: ines.chr_rom,
            mapper: Box::new(Mapper000 {
                program_memory_banks: ines.header.prg_rom_size,
            }),
        })
    }
}

#[derive(Error, Debug)]
pub enum INesToCartridgeError {
    #[error("mapper {0} not supported")]
    MapperNotSupported(u8),
}

impl Memory for Cartridge {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x6000..=0x7FFF => todo!("cartridge ram functionality not yet implemented"),
            0x8000..=0xFFFF => {
                let mapped_address = self.mapper.map_cpu_read(addr) as usize;

                *self
                    .program_memory
                    .get(mapped_address)
                    .unwrap_or(&((addr >> 8) as u8)) // Open bus
            }
            _ => (addr >> 8) as u8, // Open bus
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x6000..=0x7FFF => todo!("cartridge ram functionality not yet implemented"),
            0x8000..=0xFFFF => {
                todo!("attempted to write in cartridge rom! addr = {addr}, value = {value}")
            }
            _ => (),
        }
    }
}
