use sdl2::mixer::Music;

use crate::game;

const FADE_DURATION: i32 = 2000; //ms

pub fn handle_music(music: &Music, state: &game::State) {
    match (state, Music::is_playing()) {
        (game::State::Menu, true) => Music::fade_out(FADE_DURATION).unwrap(),
        (game::State::Play, false) => music.fade_in(-1, FADE_DURATION).unwrap(),
        _ => {}
    }
}
