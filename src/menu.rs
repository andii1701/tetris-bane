use crate::game;
use crate::sound;

const GAME_TITLE: &str = "Tetris Bane";

const LABEL_LENGTH: usize = 22;

pub enum Item {
    Play { label: String },
    Mode { label: String },
    Quit { label: String },
    Resume { label: String },
    EndGame { label: String },
    Music { label: String },
    MusicVolume { label: String },
}

pub struct Menu {
    pub items: Vec<Item>,
    pub modes: Vec<game::Mode>,
    pub item_selected: usize,
    pub mode_selected: usize,
    pub title: String,
    pub music_toggle: bool,
    pub music_volume: i32,
}

pub fn initialise() -> Menu {
    let modes = vec![
        game::Mode::Bane {
            label: "Bane".to_string(),
        },
        game::Mode::Classic {
            label: "Classic".to_string(),
        },
        game::Mode::Chill {
            label: "Chill".to_string(),
        },
    ];
    let mode_selected = 0;
    let music_toggle = true;
    let music_volume = 70;

    Menu {
        items: menu_items(&modes, mode_selected, music_toggle, music_volume),
        item_selected: 0,
        modes: modes,
        mode_selected: mode_selected,
        title: GAME_TITLE.to_string(),
        music_toggle: music_toggle,
        music_volume: music_volume,
    }
}

pub fn menu_items(
    modes: &Vec<game::Mode>,
    mode_selected: usize,
    music_toggle: bool,
    music_volume: i32,
) -> Vec<Item> {
    vec![
        Item::Play {
            label: "Play                  ".to_string(),
        },
        build_mode_item(modes, mode_selected),
        Item::Music {
            label: music_label(music_toggle),
        },
        Item::MusicVolume {
            label: volume_label(music_volume),
        },
        Item::Quit {
            label: "Quit                  ".to_string(),
        },
    ]
}

pub fn paused_menu_items(music_toggle: bool, music_volume: i32) -> Vec<Item> {
    vec![
        Item::Resume {
            label: "Resume                ".to_string(),
        },
        Item::EndGame {
            label: "End Game              ".to_string(),
        },
        Item::Music {
            label: music_label(music_toggle),
        },
        Item::MusicVolume {
            label: volume_label(music_volume),
        },
        Item::Quit {
            label: "Quit                  ".to_string(),
        },
    ]
}

pub fn update(event: &Option<game::Input>, mut world: &mut game::World) {
    let mut menu = &mut world.menu;

    if let Some(event) = event {
        match event {
            game::Input::LeftKeyDown => shift_left_or_right(&mut world.menu, -1),
            game::Input::RightKeyDown => shift_left_or_right(&mut world.menu, 1),
            game::Input::UpKeyDown => {
                menu.item_selected = change_index_wrapped(menu.item_selected, -1, menu.items.len());
            }
            game::Input::DownKeyDown => {
                menu.item_selected = change_index_wrapped(menu.item_selected, 1, menu.items.len());
            }
            game::Input::ReturnKeyDown | game::Input::SpaceKeyDown => {
                match menu.items[menu.item_selected] {
                    Item::Play { .. } => {
                        world.state = game::State::Play;
                        world.music_file = sound::music_path(&menu.modes[menu.mode_selected]);
                        game::initialise_game(&mut world);
                    }
                    Item::EndGame { .. } => {
                        world.state = game::State::Menu;
                        menu.item_selected = 0;
                        menu.items = menu_items(
                            &menu.modes,
                            menu.mode_selected,
                            menu.music_toggle,
                            menu.music_volume,
                        );
                        world.menu.title = GAME_TITLE.to_string();
                    }
                    Item::Resume { .. } => {
                        world.state = game::State::Play;
                        world.menu.title = GAME_TITLE.to_string();
                    }
                    Item::Quit { .. } => world.state = game::State::Quit,
                    Item::Mode { .. } => shift_left_or_right(&mut world.menu, -1),
                    Item::Music { .. } => toggle_music(&mut menu),
                    Item::MusicVolume { .. } => {}
                }
            }
            game::Input::EscKeyDown => match world.state {
                game::State::Paused => {
                    world.state = game::State::Play;
                }
                game::State::Menu => world.state = game::State::Quit,
                _ => assert!(false, "Should never be in this menu state."),
            },
            _ => {}
        }
    }
}

fn toggle_music(menu: &mut Menu) {
    menu.music_toggle = !menu.music_toggle;
    menu.items[menu.item_selected] = Item::Music {
        label: music_label(menu.music_toggle),
    };
}

fn shift_left_or_right(mut menu: &mut Menu, delta: i32) {
    match menu.items[menu.item_selected] {
        Item::Mode { .. } => {
            menu.mode_selected = change_index_wrapped(menu.mode_selected, delta, menu.modes.len());
            menu.items[menu.item_selected] = build_mode_item(&menu.modes, menu.mode_selected);
        }
        Item::Music { .. } => toggle_music(&mut menu),
        Item::MusicVolume { .. } => {
            let volume = menu.music_volume + delta * 10;
            menu.music_volume = match volume {
                volume if volume < 0 => 0,
                volume if volume > 128 => 128,
                _ => volume,
            };
            menu.items[menu.item_selected] = Item::MusicVolume {
                label: volume_label(menu.music_volume),
            };
        }

        _ => {}
    }
}

fn change_index_wrapped(index: usize, delta: i32, length: usize) -> usize {
    assert!([-1, 1].contains(&delta), "Delta must be -1 or 1.");
    if index as i32 + delta < 0 {
        (length as i32 + delta) as usize
    } else {
        ((index as i32 + delta) % length as i32) as usize
    }
}

fn build_mode_item(modes: &Vec<game::Mode>, selected: usize) -> Item {
    Item::Mode {
        label: mode_label(&modes[selected]),
    }
}

fn mode_label(mode: &game::Mode) -> String {
    let prefix = "Mode:";
    let mode = format!(
        "{}",
        match mode {
            game::Mode::Classic { label }
            | game::Mode::Chill { label }
            | game::Mode::Bane { label } => label,
        }
    );

    let padding = " ".repeat(LABEL_LENGTH - prefix.len() - mode.len());
    format!("{}{}{}", prefix, padding, mode)
}

fn volume_label(volume: i32) -> String {
    let n_dots = (volume as f32 / 10.) as usize;
    let n_spaces = 12 - n_dots;
    format!("Volume: |{}{}|", ".".repeat(n_dots), " ".repeat(n_spaces))
}

fn music_label(toggle: bool) -> String {
    let prefix = "Music:";
    let toggle = match toggle {
        true => "On".to_string(),
        false => "Off".to_string(),
    };

    let padding = " ".repeat(LABEL_LENGTH - prefix.len() - toggle.len());
    format!("{}{}{}", prefix, padding, toggle)
}
