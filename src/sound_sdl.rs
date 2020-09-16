use sdl2::mixer::Music;

pub fn handle_music(music: &Music) {
    if !Music::is_playing() {
        music.play(1).unwrap();
    }

    //Music::pause();
}
