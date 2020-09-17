use sdl2::mixer::Music;

use crate::game;

const FADE_DURATION: i32 = 2000; //ms

pub fn handle_music(music: &Music, state: &game::State, toggle: bool, volume: i32) {
    if Music::get_volume() != volume {
        Music::set_volume(volume)
    }

    match (state, Music::is_playing(), toggle) {
        // Game is in menu and music is playing
        (game::State::Menu, true, _) => Music::fade_out(FADE_DURATION).unwrap(),
        // Game is paused and music is playing and toggle is off
        (game::State::Paused, true, false) => Music::fade_out(FADE_DURATION).unwrap(),
        // Game is paused and music is not playing and toggle is on
        (game::State::Paused, false, true) => music.fade_in(-1, FADE_DURATION).unwrap(),
        // Game is playing, music is not playing and toggle is on
        (game::State::Play, false, true) => music.fade_in(-1, FADE_DURATION).unwrap(),
        _ => {}
    }
}
