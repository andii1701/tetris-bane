use crate::game;

const BANE_MUSIC_PATH: &str = "assets/music/Kevin_MacLeod_-_Ghost_Dance.ogg";
const CHILL_MUSIC_PATH: &str = "assets/music/a-very-brady-special-by-kevin-macleod.ogg";
const CLASSIC_MUSIC_PATH: &str = "assets/music/midnight-tale-by-kevin-macleod.ogg";

pub fn music_path(mode: &game::Mode) -> String {
    match mode {
        game::Mode::Bane { .. } => BANE_MUSIC_PATH.to_string(),
        game::Mode::Chill { .. } => CHILL_MUSIC_PATH.to_string(),
        game::Mode::Classic { .. } => CLASSIC_MUSIC_PATH.to_string(),
    }
}
