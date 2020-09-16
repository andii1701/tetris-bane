use crate::game;
use crate::sound;

const GAME_TITLE: &str = "Tetris Bane";

pub enum Item {
    Play { label: String },
    Mode { label: String },
    Quit { label: String },
    Resume { label: String },
    EndGame { label: String },
    Music { label: String },
}

pub struct Menu {
    pub items: Vec<Item>,
    pub modes: Vec<game::Mode>,
    pub item_selected: usize,
    pub mode_selected: usize,
    pub title: String,
    pub music_toggle: bool,
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

    Menu {
        items: menu_items(&modes, mode_selected, music_toggle),
        item_selected: 0,
        modes: modes,
        mode_selected: mode_selected,
        title: GAME_TITLE.to_string(),
        music_toggle: music_toggle,
    }
}

pub fn menu_items(modes: &Vec<game::Mode>, mode_selected: usize, music_toggle: bool) -> Vec<Item> {
    vec![
        Item::Play {
            label: "Play".to_string(),
        },
        build_mode_item(modes, mode_selected),
        Item::Music {
            label: format!("Music: {}", bool_to_string(music_toggle)),
        },
        Item::Quit {
            label: "Quit".to_string(),
        },
    ]
}

pub fn paused_menu_items() -> Vec<Item> {
    vec![
        Item::Resume {
            label: "Resume".to_string(),
        },
        Item::EndGame {
            label: "End Game".to_string(),
        },
        Item::Quit {
            label: "Quit".to_string(),
        },
    ]
}

pub fn update(event: &Option<game::Input>, mut world: &mut game::World) {
    let mut menu = &mut world.menu;

    if let Some(event) = event {
        match event {
            game::Input::LeftKeyDown => change_mode(&mut world.menu, -1),
            game::Input::RightKeyDown => change_mode(&mut world.menu, 1),
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
                        menu.items = menu_items(&menu.modes, menu.mode_selected, menu.music_toggle);
                        world.menu.title = GAME_TITLE.to_string();
                    }
                    Item::Resume { .. } => {
                        world.state = game::State::Play;
                        world.menu.title = GAME_TITLE.to_string();
                    }
                    Item::Quit { .. } => world.state = game::State::Quit,
                    Item::Mode { .. } => change_mode(&mut menu, 1),
                    Item::Music { .. } => toggle_music(&mut menu),
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
        label: format!("Music: {}", bool_to_string(menu.music_toggle)),
    };
}

fn change_mode(mut menu: &mut Menu, delta: i32) {
    match menu.items[menu.item_selected] {
        Item::Mode { .. } => {
            menu.mode_selected = change_index_wrapped(menu.mode_selected, delta, menu.modes.len());
            menu.items[menu.item_selected] = build_mode_item(&menu.modes, menu.mode_selected);
        }
        Item::Music { .. } => toggle_music(&mut menu),
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
        label: format!(
            "Mode:  < {} >",
            match &modes[selected] {
                game::Mode::Classic { label }
                | game::Mode::Chill { label }
                | game::Mode::Bane { label } => label,
            },
        ),
    }
}

fn bool_to_string(b: bool) -> String {
    match b {
        true => "On".to_string(),
        false => "Off".to_string(),
    }
}
