use std::{fs::File, io::Seek};

use tempfile::tempfile;

use r_nes::{
    cartridge::Cartridge,
    cpu::memory::Memory,
    ines::{INes, header::INesHeader},
};

pub fn create_test_nes_file(ines: INes) -> File {
    let mut file = tempfile().unwrap();

    ines.save(&mut file).expect("failed to save rom to file");
    file.rewind().expect("failed to rewing test file");

    file
}

#[test]
fn test_load_cartridge() {
    let prg_rom = (0..16 * 1024).map(|i| (i % 256) as u8).collect();
    let chr_rom = (0..8 * 1024).map(|i| (i % 256) as u8).collect();

    let mut test_file = create_test_nes_file(INes {
        header: INesHeader {
            prg_rom_size: 1,
            chr_rom_size: 1,
            ..Default::default()
        },
        trainer: None,
        prg_rom,
        chr_rom,
    });

    let cartridge = Cartridge::load(&mut test_file).unwrap();

    assert_eq!(cartridge.program_memory.len(), 16 * 1024);
    assert_eq!(cartridge.character_memory.len(), 8 * 1024);

    for i in 0..cartridge.program_memory.len() {
        assert_eq!(cartridge.program_memory[i], (i % 256) as u8);
    }

    for i in 0..cartridge.character_memory.len() {
        assert_eq!(cartridge.character_memory[i], (i % 256) as u8);
    }
}

#[test]
fn test_cpu_read_1_prg_bank() {
    let prg_rom = (0..16 * 1024).map(|i| (i % 256) as u8).collect();
    let chr_rom = (0..8 * 1024).map(|i| (i % 256) as u8).collect();

    let mut test_file = create_test_nes_file(INes {
        header: INesHeader {
            prg_rom_size: 1,
            chr_rom_size: 1,
            ..Default::default()
        },
        trainer: None,
        prg_rom,
        chr_rom,
    });

    let cartridge = Cartridge::load(&mut test_file).unwrap();

    assert_eq!(cartridge.read(0x8000), 0);
    assert_eq!(cartridge.read(0xBFFF), (0x3FFF % 256) as u8);

    assert_eq!(cartridge.read(0xC000), 0);
    assert_eq!(cartridge.read(0xFFFF), (0x3FFF % 256) as u8);
}

#[test]
fn test_cpu_read_2_prg_banks() {
    let prg_rom = (0..2 * 16 * 1024).map(|i| (i % 256) as u8).collect();
    let chr_rom = (0..8 * 1024).map(|i| (i % 256) as u8).collect();

    let mut test_file = create_test_nes_file(INes {
        header: INesHeader {
            prg_rom_size: 2,
            chr_rom_size: 1,
            ..Default::default()
        },
        trainer: None,
        prg_rom,
        chr_rom,
    });

    let cartridge = Cartridge::load(&mut test_file).unwrap();

    assert_eq!(cartridge.read(0x8000), 0);
    assert_eq!(cartridge.read(0xBFFF), (0x3FFF % 256) as u8);

    assert_eq!(cartridge.read(0xC000), (0x4000 % 256) as u8);
    assert_eq!(cartridge.read(0xFFFF), (0x7FFF % 256) as u8);
}
