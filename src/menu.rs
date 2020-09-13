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
            game::Input::LeftKeyDown => {}
            game::Input::RightKeyDown => {
                menu.mode_selected += 1;
                menu.mode_selected %= menu.modes.len() as usize;
                menu.items[1] = build_mode_item(&menu.modes, menu.mode_selected);
            }
            game::Input::UpKeyDown => {
                menu.item_selected = if menu.item_selected as i32 - 1 < 0 {
                    (menu.items.len() - 1) as usize
                } else {
                    (menu.item_selected - 1) as usize
                };
            }
            game::Input::DownKeyDown => {
                menu.item_selected += 1;
                menu.item_selected %= menu.items.len() as usize;
            }
            game::Input::ReturnDown => match menu.items[menu.item_selected] {
                Item::Play { label: _ } => {
                    world.state = game::State::Play;
                    game::initialise_game(&mut world);
                }
                Item::Quit { label: _ } => world.state = game::State::Quit,
                _ => {}
            },
            _ => {}
        }
    }
}

fn build_mode_item(modes: &Vec<game::Mode>, selected: usize) -> Item {
    Item::Mode {
        label: format!(
            "Mode: {}",
            match &modes[selected] {
                game::Mode::Classic { label } | game::Mode::Chill { label } => label,
            }
        ),
    }
}
