use sdl2::mixer::Music;

pub fn handle_music(music: &Option<Music>, file: &String) {
    if let Some(music) = &music {
        music.play(1).unwrap();
    }

    //Music::pause();
}
