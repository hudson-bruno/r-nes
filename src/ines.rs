use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

use thiserror::Error;

use crate::ines::header::{INesHeader, INesHeaderError};

pub mod header;

pub struct INes {
    pub header: INesHeader,
    pub trainer: Option<[u8; 512]>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl INes {
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, INesError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut raw_header: [u8; 16] = [0; 16];
        reader.read_exact(&mut raw_header)?;

        let header: INesHeader = raw_header.try_into()?;

        let trainer = if header.has_trainer {
            let mut trainer: [u8; 512] = [0; 512];
            reader.read_exact(&mut trainer)?;

            Some(trainer)
        } else {
            None
        };

        let mut prg_rom = vec![0; (16 * 1024) * (header.prg_rom_size as usize)];
        reader.read_exact(&mut prg_rom)?;

        let mut chr_rom = vec![0; (8 * 1024) * (header.chr_rom_size as usize)];
        reader.read_exact(&mut chr_rom)?;

        Ok(Self {
            header,
            trainer,
            prg_rom,
            chr_rom,
        })
    }
}

#[derive(Error, Debug)]
pub enum INesError {
    #[error("failed to fetch file header")]
    IoHeaderError(#[from] io::Error),

    #[error("invalid file header")]
    HeaderParseError(#[from] INesHeaderError),
}
