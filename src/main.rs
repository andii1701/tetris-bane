use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;

fn render_weird_gradient(width: u64, height: u64) {
    unsafe {
        //let mut row: *mut u32 = buffer as *mut u32;

        //let color32 =
        //    (255 as u32) << 24 | (255 as u32) << 16 | (255 as u32) << 8 | (255 as u32) << 0;
        //let foo2: u8 = 0;
        //let foo3: u32 = color32;
        //*row = foo3;
        //let foo = 3;
        //*pixel = color32;

        /*let mut row: *mut u8 = buffer as *mut u8;

        for y in 0..=height - 1 {
            let mut pixelChannel: *const u8 = row;
            for x in 0..=width - 1 {
                let full: u8 = 255;
                let zero: u8 = 0;
                pixelChannel = &full; // R
                pixelChannel = pixelChannel.offset(1);
                pixelChannel = &zero; // G
                pixelChannel = pixelChannel.offset(1);
                pixelChannel = &zero; // B
                pixelChannel = pixelChannel.offset(1);
                // Alpha
                pixelChannel = &zero;
                pixelChannel = pixelChannel.offset(1);
            }
        }
        row = row.offset(pitch as isize);*/
    }
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

    let bytes_per_pixel = 4;
    let pitch = width * bytes_per_pixel;
    let buffer_size = pitch * height;
    let mut buffer = vec![0; buffer_size as usize];

    let surface = Surface::from_data(
        buffer.as_mut_slice(),
        width,
        height,
        pitch,
        PixelFormatEnum::RGBA32,
    )
    .unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;

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
