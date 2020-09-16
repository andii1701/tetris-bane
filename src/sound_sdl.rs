use sdl2::mixer::Music;

use crate::game;

pub fn handle_music(music: &Music, state: &game::State) {
    match (state, Music::is_playing(), Music::is_paused()) {
        (game::State::Paused, true, false) => Music::pause(),
        (game::State::Play, true, true) => Music::resume(),
        (game::State::Play, false, false) => music.play(-1).unwrap(),
        _ => {}
    }
}
