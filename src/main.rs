use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

static bytes_per_pixel: u32 = 4;

fn render_weird_gradient(width: u32, height: u32, pitch: u32) -> Vec<u8> {
    let buffer_size = pitch * height;
    let mut buffer: Vec<u8> = vec![0; buffer_size as usize];

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
            buffer[i] = x as u8;
            i += 1;
            //A
            buffer[i] = 255;
            i += 1;
        }
    }

    buffer
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-tetris-sdl", 1024, 768)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // TODO: Figure out what accelerated does
    let mut canvas = window.into_canvas().accelerated().build().unwrap();

    let (width, height) = canvas.output_size().unwrap();
    let pitch = width * bytes_per_pixel;

    let mut buffer = render_weird_gradient(width, height, pitch);

    let surface = Surface::from_data(
        buffer.as_mut_slice(),
        width,
        height,
        pitch,
        PixelFormatEnum::RGBA32,
    )
    .unwrap();

    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();

    //canvas.set_draw_color(Color::RGB(0, 255, 255));
    //canvas.clear();
    canvas.copy(&texture, None, None);
    canvas.present();
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
        // The rest of the game loop goes here...

        //canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
