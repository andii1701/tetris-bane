// TODO limit frame rate

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

static BYTES_PER_PIXEL: u32 = 4;

fn render_weird_gradient(buffer: &mut Vec<u8>, width: u32, height: u32, x_offset: u32) {
    let mut i = 0;
    for y in 0..height {
        for x in 0..width {
            //R
            buffer[i] = 0;
            i += 1;

            //G
            buffer[i] = y as u8;
            i += 1;

            //B
            buffer[i] = (x + x_offset) as u8;
            i += 1;
            //A
            buffer[i] = 255;
            i += 1;
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem
        .window("rust-tetris-sdl", 1024, 768)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // TODO: Figure out what accelerated does
    let mut canvas = window.into_canvas().accelerated().build().unwrap();

    let (width, height) = canvas.output_size().unwrap();
    let pitch = width * BYTES_PER_PIXEL;

    let buffer_size = (pitch * height) as usize;
    let mut buffer: Vec<u8> = vec![0; buffer_size as usize];
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut x_offset: u32 = 0;
    'running: loop {
        let start = timer_subsystem.performance_counter();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    //x_offset += 1;
                    println!("M");
                }

                _ => {}
            }
        }
        // The rest of the game loop goes here...

        render_weird_gradient(&mut buffer, width, height, x_offset);
        x_offset += 1;
        let surface = Surface::from_data(
            buffer.as_mut_slice(),
            width,
            height,
            pitch,
            PixelFormatEnum::RGBA32,
        )
        .unwrap();

        let texture = surface.as_texture(&texture_creator).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        let end = timer_subsystem.performance_counter();
        let elapsed = (end - start) as f32 / timer_subsystem.performance_frequency() as f32;
        if x_offset % 60 == 0 {
            println!("fps: {}", (1.0 / elapsed as f32));
        }
    }
}
