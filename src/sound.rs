use crate::game;

const BANE_MUSIC_PATH: &str = "assets/music/Kevin_MacLeod_-_Ghost_Dance.ogg";

pub fn music_path(mode: &game::Mode) -> String {
    match mode {
        game::Mode::Bane { .. } => BANE_MUSIC_PATH.to_string(),
        _ => "".to_string(),
    }
}
