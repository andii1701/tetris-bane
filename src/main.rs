use std::path::Path;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;

static BYTES_PER_PIXEL: u32 = 4;

static FONT_PATH: &str = "resources/fonts/Bitstream-Vera-Sans-Mono/VeraMono.ttf";

static OVERLAY_FONT_SIZE: u16 = 12;

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

fn fps_color(fps: u32) -> Color {
    match fps {
        0..=23 => Color::RGB(255, 0, 0),
        24..=57 => Color::RGB(255, 255, 0),
        _ => Color::RGB(0, 255, 0),
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let font_path: &Path = Path::new(FONT_PATH);
    let font = ttf_context.load_font(font_path, OVERLAY_FONT_SIZE).unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem
        .window("rust-tetris-sdl", 1024, 768)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let (width, height) = canvas.output_size().unwrap();
    let pitch = width * BYTES_PER_PIXEL;

    let buffer_size = (pitch * height) as usize;
    let mut buffer: Vec<u8> = vec![0; buffer_size as usize];
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut x_offset: u32 = 0;
    let mut fps_string: String = " ".to_string();
    let mut fps = 60;
    let mut timer = Instant::now();
    let mut show_fps = true;

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
                    keycode: Some(Keycode::F1),
                    ..
                } => {
                    show_fps = !show_fps;
                }

                _ => {}
            }
        }
        // The rest of the game loop goes here...

        render_weird_gradient(&mut buffer, width, height, x_offset);
        x_offset += 1;
        let mut surface = Surface::from_data(
            buffer.as_mut_slice(),
            width,
            height,
            pitch,
            PixelFormatEnum::RGBA32,
        )
        .unwrap();
        if show_fps {
            let font_surface = font.render(&fps_string).blended(fps_color(fps)).unwrap();
            font_surface.blit(None, &mut surface, None).unwrap();
        }

        let texture = surface.as_texture(&texture_creator).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        if timer.elapsed().as_millis() > 333 {
            let end = timer_subsystem.performance_counter();
            let elapsed = (end - start) as f32 / timer_subsystem.performance_frequency() as f32;
            fps = (1.0 / elapsed as f32) as u32;
            fps_string = format!("fps: {}", fps);
            timer = Instant::now();
        }
    }
}
