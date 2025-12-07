use std::{env, fs::File};

use r_nes::{cartridge::Cartridge, cpu::ExitStatus, nes::Nes};
use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    render::TextureAccess,
};

const NES_WIDTH: u32 = 256;
const NES_HEIGHT: u32 = 240;

const SYSTEM_PALETTE: [(u8, u8, u8); 64] = [
    (0x80, 0x80, 0x80),
    (0x00, 0x3D, 0xA6),
    (0x00, 0x12, 0xB0),
    (0x44, 0x00, 0x96),
    (0xA1, 0x00, 0x5E),
    (0xC7, 0x00, 0x28),
    (0xBA, 0x06, 0x00),
    (0x8C, 0x17, 0x00),
    (0x5C, 0x2F, 0x00),
    (0x10, 0x45, 0x00),
    (0x05, 0x4A, 0x00),
    (0x00, 0x47, 0x2E),
    (0x00, 0x41, 0x66),
    (0x00, 0x00, 0x00),
    (0x05, 0x05, 0x05),
    (0x05, 0x05, 0x05),
    (0xC7, 0xC7, 0xC7),
    (0x00, 0x77, 0xFF),
    (0x21, 0x55, 0xFF),
    (0x82, 0x37, 0xFA),
    (0xEB, 0x2F, 0xB5),
    (0xFF, 0x29, 0x50),
    (0xFF, 0x22, 0x00),
    (0xD6, 0x32, 0x00),
    (0xC4, 0x62, 0x00),
    (0x35, 0x80, 0x00),
    (0x05, 0x8F, 0x00),
    (0x00, 0x8A, 0x55),
    (0x00, 0x99, 0xCC),
    (0x21, 0x21, 0x21),
    (0x09, 0x09, 0x09),
    (0x09, 0x09, 0x09),
    (0xFF, 0xFF, 0xFF),
    (0x0F, 0xD7, 0xFF),
    (0x69, 0xA2, 0xFF),
    (0xD4, 0x80, 0xFF),
    (0xFF, 0x45, 0xF3),
    (0xFF, 0x61, 0x8B),
    (0xFF, 0x88, 0x33),
    (0xFF, 0x9C, 0x12),
    (0xFA, 0xBC, 0x20),
    (0x9F, 0xE3, 0x0E),
    (0x2B, 0xF0, 0x35),
    (0x0C, 0xF0, 0xA4),
    (0x05, 0xFB, 0xFF),
    (0x5E, 0x5E, 0x5E),
    (0x0D, 0x0D, 0x0D),
    (0x0D, 0x0D, 0x0D),
    (0xFF, 0xFF, 0xFF),
    (0xA6, 0xFC, 0xFF),
    (0xB3, 0xEC, 0xFF),
    (0xDA, 0xAB, 0xEB),
    (0xFF, 0xA8, 0xF9),
    (0xFF, 0xAB, 0xB3),
    (0xFF, 0xD2, 0xB0),
    (0xFF, 0xEF, 0xA6),
    (0xFF, 0xF7, 0x9C),
    (0xD7, 0xE8, 0x95),
    (0xA6, 0xED, 0xAF),
    (0xA2, 0xF2, 0xDA),
    (0x99, 0xFF, 0xFC),
    (0xDD, 0xDD, 0xDD),
    (0x11, 0x11, 0x11),
    (0x11, 0x11, 0x11),
];

fn main() -> Result<(), ExitStatus> {
    let args: Vec<String> = env::args().collect();
    let rom_path = if args.len() > 1 { &args[1] } else { "rom.nes" };

    let cartridge = {
        let mut file = File::open(rom_path).expect("Could not open ROM file");
        Cartridge::load(&mut file).unwrap()
    };

    let mut nes = Nes::new_with_cartridge(cartridge);

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("r-nes", NES_WIDTH * 3, NES_HEIGHT * 3)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture(
            PixelFormat::RGB24,
            TextureAccess::Streaming,
            NES_WIDTH,
            NES_HEIGHT,
        )
        .unwrap();
    let mut frame_buffer = vec![0u8; (NES_WIDTH * NES_HEIGHT * 3) as usize];

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        while nes.bus.ppu.scanline < 241 {
            nes.step();
        }

        for y in 0..NES_HEIGHT {
            for x in 0..NES_WIDTH {
                let palette_index = nes.bus.ppu.frame_data[y as usize][x as usize];
                let (r, g, b) = SYSTEM_PALETTE[palette_index as usize];
                let buffer_index = ((y * NES_WIDTH + x) * 3) as usize;
                frame_buffer[buffer_index] = r;
                frame_buffer[buffer_index + 1] = g;
                frame_buffer[buffer_index + 2] = b;
            }
        }

        texture
            .update(None, &frame_buffer, (NES_WIDTH * 3) as usize)
            .unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        while nes.bus.ppu.scanline >= 241 {
            nes.step();
        }
    }

    Ok(())
}
