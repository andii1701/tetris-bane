use crate::game;

pub enum Item {
    Play { label: String },
    Mode { label: String },
    Quit { label: String },
}

pub struct Menu {
    pub items: Vec<Item>,
    pub modes: Vec<game::Mode>,
    pub item_selected: usize,
    pub mode_selected: usize,
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

    Menu {
        items: vec![
            Item::Play {
                label: "Play".to_string(),
            },
            build_mode_item(&modes, mode_selected),
            Item::Quit {
                label: "Quit".to_string(),
            },
        ],
        item_selected: 0,
        modes: modes,
        mode_selected: mode_selected,
    }
}

pub fn update(event: &Option<game::Input>, mut world: &mut game::World) {
    let menu = &mut world.menu;

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
            game::Input::ReturnDown | game::Input::SpaceDown => {
                match menu.items[menu.item_selected] {
                    Item::Play { label: _ } => {
                        world.state = game::State::Play;
                        game::initialise_game(&mut world);
                    }
                    Item::Quit { label: _ } => world.state = game::State::Quit,
                    Item::Mode { label: _ } => change_mode(&mut world.menu, 1),
                }
            }
            _ => {}
        }
    }
}

fn change_mode(menu: &mut Menu, delta: i32) {
    match menu.items[menu.item_selected] {
        Item::Mode { label: _ } => {
            menu.mode_selected = change_index_wrapped(menu.mode_selected, delta, menu.modes.len());
            menu.items[menu.item_selected] = build_mode_item(&menu.modes, menu.mode_selected);
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
