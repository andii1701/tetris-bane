// TODO manual sound toglle
// TODO volume control for sound
// TODO volume control for music
// TODO toggle music
// TODO menu, with res options, vol control, full screen

use std::path::Path;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
//use sdl2::rect::Rect;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::surface::Surface;

static BYTES_PER_PIXEL: u32 = 4;

static FONT_PATH: &str = "assets/fonts/Bitstream-Vera-Sans-Mono/VeraMono.ttf";
static SOUND_PATH: &str = "assets/sounds/chirp.wav";
static MUSIC_PATH: &str = "assets/music/music.ogg";

static OVERLAY_FONT_SIZE: u16 = 12;

struct Buffer {
    memory: Vec<u8>,
    width: u32,
    height: u32,
    pitch: u32,
}

fn render_weird_gradient(buffer: &mut Buffer, x_offset: u32) {
    let mut i = 0;
    for y in 0..buffer.height {
        for x in 0..buffer.width {
            //R
            buffer.memory[i] = 0;
            i += 1;

            //G
            buffer.memory[i] = y as u8;
            i += 1;

            //B
            buffer.memory[i] = (x + x_offset) as u8;
            i += 1;
            //A
            buffer.memory[i] = 255;
            i += 1;
        }
    }
}

fn resize_buffer(buffer: &mut Buffer, width: u32, height: u32) {
    buffer.width = width;
    buffer.height = height;
    buffer.pitch = width * BYTES_PER_PIXEL;
    buffer
        .memory
        .resize((buffer.pitch * buffer.height) as usize, 0);
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

    sdl_context.audio().unwrap();
    sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1_024).unwrap();
    sdl2::mixer::init(InitFlag::OGG).unwrap();
    sdl2::mixer::allocate_channels(2);

    // Sound
    let sound_chunk = sdl2::mixer::Chunk::from_file(SOUND_PATH).unwrap();
    sdl2::mixer::Channel::all().play(&sound_chunk, 1).unwrap();

    let music = sdl2::mixer::Music::from_file(MUSIC_PATH).unwrap();
    music.play(1).unwrap();

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
    let texture_creator = canvas.texture_creator();

    //let mut buffer: Vec<u8> =
    let (width, height) = canvas.output_size().unwrap();
    let pitch = width * BYTES_PER_PIXEL;
    let mut buffer = Buffer {
        memory: vec![0; (pitch * height) as usize],
        width: width,
        height: height,
        pitch: pitch,
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut x_offset: u32 = 0;
    let mut fps_string: String = " ".to_string();
    let mut fps = 60;
    let mut wallclock = Instant::now();
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
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    resize_buffer(&mut buffer, width as u32, height as u32);
                }
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

        render_weird_gradient(&mut buffer, x_offset);
        x_offset += 1;
        let mut surface = Surface::from_data(
            buffer.memory.as_mut_slice(),
            buffer.width,
            buffer.height,
            buffer.pitch,
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

        if wallclock.elapsed().as_millis() > 333 {
            let end = timer_subsystem.performance_counter();
            let elapsed = (end - start) as f32 / timer_subsystem.performance_frequency() as f32;
            fps = (1.0 / elapsed as f32) as u32;
            fps_string = format!("fps: {}", fps);
            wallclock = Instant::now();
        }
    }
}
