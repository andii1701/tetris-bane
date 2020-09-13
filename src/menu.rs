use crate::game;

pub enum Item {
    Play { label: String },
    Mode { label: String },
    Quit { label: String },
}

pub struct Menu {
    pub items: Vec<Item>,
    pub selected: usize,
}

pub fn initialise() -> Menu {
    Menu {
        items: vec![
            Item::Play {
                label: "Play".to_string(),
            },
            Item::Mode {
                label: "Mode:".to_string(),
            },
            Item::Quit {
                label: "Quit".to_string(),
            },
        ],
        selected: 0,
    }
}

pub fn update(event: &Option<game::Input>, mut world: &mut game::World) {
    let menu = &mut world.menu;

    if let Some(event) = event {
        match event {
            // NOTE: DownKeyUp needs to be first in the match call otherwise
            // the DownKeyUp event will be missed if the user is holding down
            // another key.
            game::Input::LeftKeyDown => {}
            game::Input::RightKeyDown => {}
            game::Input::UpKeyDown => {
                menu.selected = if menu.selected as i32 - 1 < 0 {
                    (menu.items.len() - 1) as usize
                } else {
                    (menu.selected - 1) as usize
                };
            }
            game::Input::DownKeyDown => {
                menu.selected += 1;
                menu.selected %= menu.items.len() as usize;
            }
            game::Input::ReturnDown => match menu.items[menu.selected] {
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
