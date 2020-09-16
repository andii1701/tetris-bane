// TODO
// menu, with res options, vol control, full screen.
// fade?
// sound
// Rebrand to Tetris Bane
// wasm
// Profile menu and see why its chewing up so much cpu
// licenses

use std::path::Path;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mixer;
use sdl2::mixer::Music;
use sdl2::pixels::Color;

mod block;
mod game;
mod game_sdl_layer;
mod menu;
mod sound;
mod sound_sdl;

const SYSTEM_FONT_PATH: &str = "assets/fonts/Bitstream-Vera-Sans-Mono/VeraMono.ttf";
const SOUND_PATH: &str = "assets/sounds/chrip_44.wav";
//const MUSIC_PATH: &str = "assets/music/music.ogg";

const OVERLAY_FONT_SIZE: u16 = 12;

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
    sdl2::mixer::open_audio(44_100, mixer::DEFAULT_FORMAT, mixer::DEFAULT_CHANNELS, 1024).unwrap();
    sdl2::mixer::init(mixer::InitFlag::OGG).unwrap();
    sdl2::mixer::allocate_channels(2);

    // Sound
    let mut sound_chunk = sdl2::mixer::Chunk::from_file(SOUND_PATH).unwrap();
    let mut sound_chunk_volume = sound_chunk.get_volume();

    let system_font_path: &Path = Path::new(SYSTEM_FONT_PATH);
    let overlay_font = ttf_context
        .load_font(system_font_path, OVERLAY_FONT_SIZE)
        .unwrap();

    let game_fonts = game_sdl_layer::initialise_fonts(&ttf_context);

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

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut fps_string: String = " ".to_string();
    let mut fps = 60;
    let mut wallclock = Instant::now();
    let mut show_fps = true;

    let mut input_event: Option<game::Input> = None;
    let mut world = game::initialise_world();

    let music = Some(Music::from_file("assets/music/music.ogg").unwrap());
    sound_sdl::handle_music(&music, &world.music_file);

    while world.state != game::State::Quit {
        let start = timer_subsystem.performance_counter();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => world.state = game::State::Quit,
                Event::Window {
                    win_event: WindowEvent::Resized(_width, _height),
                    ..
                } => {}
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::F1) => {
                        show_fps = !show_fps;
                    }

                    Some(Keycode::Escape) => input_event = Some(game::Input::EscKeyDown),

                    /*Some(Keycode::S) => match mixer::Channel::all().play(&sound_chunk, 0) {
                        Err(e) => println!("Error playing sound: {:?}", e),
                        Ok(_) => {}
                    },*/
                    /*Some(Keycode::M) => match Music::is_paused() {
                        true => Music::resume(),
                        false => Music::pause(),
                    },*/
                    //Some(Keycode::Minus) => Music::set_volume(Music::get_volume() - 8),
                    //Some(Keycode::Equals) => Music::set_volume(Music::get_volume() + 8),
                    Some(Keycode::Num9) => {
                        sound_chunk_volume -= 8;
                        if sound_chunk_volume < 0 {
                            sound_chunk_volume = 0;
                        }
                        sound_chunk.set_volume(sound_chunk_volume);
                    }
                    Some(Keycode::Num0) => {
                        sound_chunk_volume += 8;
                        if sound_chunk_volume > 128 {
                            sound_chunk_volume = 128;
                        }
                        sound_chunk.set_volume(sound_chunk_volume);
                    }
                    Some(Keycode::Up) | Some(Keycode::W) => {
                        input_event = Some(game::Input::UpKeyDown);
                    }
                    Some(Keycode::Down) => {
                        input_event = Some(game::Input::DownKeyDown);
                    }
                    Some(Keycode::S) => {
                        input_event = Some(game::Input::SKeyDown);
                    }
                    Some(Keycode::P) => {
                        input_event = Some(game::Input::PKeyDown);
                    }
                    Some(Keycode::Left) | Some(Keycode::A) => {
                        input_event = Some(game::Input::LeftKeyDown);
                    }
                    Some(Keycode::Right) | Some(Keycode::D) => {
                        input_event = Some(game::Input::RightKeyDown);
                    }
                    Some(Keycode::Return) => {
                        input_event = Some(game::Input::ReturnKeyDown);
                    }
                    Some(Keycode::Space) => {
                        input_event = Some(game::Input::SpaceKeyDown);
                    }
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::Down) => {
                        input_event = Some(game::Input::DownKeyUp);
                    }
                    Some(Keycode::Space) => {
                        input_event = Some(game::Input::SpaceKeyUp);
                    }
                    Some(Keycode::S) => {
                        input_event = Some(game::Input::SKeyUp);
                    }

                    _ => {}
                },
                _ => {}
            }
        }

        game_sdl_layer::update_and_render(&mut canvas, &game_fonts, &input_event, &mut world);

        input_event = None;

        if show_fps {
            let font_surface = overlay_font
                .render(&fps_string)
                .blended(fps_color(fps))
                .unwrap();
            let texture = font_surface.as_texture(&texture_creator).unwrap();
            canvas.copy(&texture, None, font_surface.rect()).unwrap();
        }

        canvas.present();

        if show_fps && wallclock.elapsed().as_millis() > 333 {
            let end = timer_subsystem.performance_counter();
            let elapsed = (end - start) as f32 / timer_subsystem.performance_frequency() as f32;
            fps = (1.0 / elapsed as f32) as u32;
            fps_string = format!("fps: {}", fps);
            wallclock = Instant::now();
        }
    }
}
