use std::{fs::File, time::Duration};

use r_nes::{cartridge::Cartridge, cpu::ExitStatus, nes::Nes};
use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    render::TextureAccess,
};

const NES_WIDTH: u32 = 256;
const NES_HEIGHT: u32 = 240;

fn main() -> Result<(), ExitStatus> {
    let cartridge = {
        let mut file = File::open("/home/hudson/nes/super_mario_brothers.nes").unwrap();
        Cartridge::load(&mut file).unwrap()
    };

    let mut nes = Nes::new_with_cartridge(cartridge);

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("r-nes", NES_WIDTH, NES_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    canvas.set_draw_color(Color::RGB(0, 0, 255));
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
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    nes.step().map_or(Ok(()), Err)?;
                }
                _ => {}
            }
        }

        for (index, byte) in frame_buffer.iter_mut().enumerate() {
            *byte = (index % 255) as u8;
        }

        texture
            .update(None, &frame_buffer, (NES_WIDTH * 3) as usize)
            .unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        println!("uep");
    }

    Ok(())
}
