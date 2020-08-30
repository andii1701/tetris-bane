// TODO limit frame rate

use std::time::Duration;

use libc::malloc;

use core::ffi::c_void; // TODO figure out which lib to use here

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;

use sdl2_sys::{SDL_CreateRGBSurfaceWithFormatFrom, SDL_PixelFormatEnum};

static BYTES_PER_PIXEL: u32 = 4;

fn render_weird_gradient(buffer: *mut u8, width: u32, height: u32, pitch: u32, x_offset: u32) {
    unsafe {
        let mut row: *mut u8 = buffer;

        for y in 0..height {
            let mut pixel_channel: *mut u8 = row;
            for x in 0..width {
                //R
                *pixel_channel = 0 as u8;

                pixel_channel = pixel_channel.offset(1);
                //G
                *pixel_channel = y as u8;

                pixel_channel = pixel_channel.offset(1);
                //B
                *pixel_channel = (x + x_offset) as u8;

                pixel_channel = pixel_channel.offset(1);
                //A
                *pixel_channel = 255;

                pixel_channel = pixel_channel.offset(1);
            }
            row = row.offset(pitch as isize);
        }
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
    let pitch = width * BYTES_PER_PIXEL;

    let mut surface: Surface;
    let buffer: *mut u8;
    unsafe {
        let buffer_size = pitch * height;
        buffer = malloc(buffer_size as usize) as *mut u8;

        render_weird_gradient(buffer, width, height, pitch, 0);
        let surface_ptr = SDL_CreateRGBSurfaceWithFormatFrom(
            buffer as *mut c_void,
            width as i32,
            height as i32,
            BYTES_PER_PIXEL as i32,
            pitch as i32,
            SDL_PixelFormatEnum::SDL_PIXELFORMAT_RGBA32 as u32,
        );
        surface = Surface::from_ll(surface_ptr);
    }
    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();

    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut x_offset: u32 = 0;
    'running: loop {
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
                    x_offset += 1;
                    println!("M");
                }

                _ => {}
            }
        }
        // The rest of the game loop goes here...
        unsafe {
            render_weird_gradient(buffer, width, height, pitch, x_offset);
            let surface_ptr = SDL_CreateRGBSurfaceWithFormatFrom(
                buffer as *mut c_void,
                width as i32,
                height as i32,
                BYTES_PER_PIXEL as i32,
                pitch as i32,
                SDL_PixelFormatEnum::SDL_PIXELFORMAT_RGBA32 as u32,
            );
            surface = Surface::from_ll(surface_ptr);
        }
        let texture_creator = canvas.texture_creator();

        let texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
