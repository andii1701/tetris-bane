use crate::game;

pub enum Item {
    Play { label: String },
    Mode { label: String },
    Quit { label: String },
}

pub struct Menu {
    pub items: Vec<Item>,
    pub selected: u8,
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

pub fn update(event: &Option<game::Input>, menu: &mut Menu) {
    if let Some(event) = event {
        match event {
            // NOTE: DownKeyUp needs to be first in the match call otherwise
            // the DownKeyUp event will be missed if the user is holding down
            // another key.
            game::Input::LeftKeyDown => {}
            game::Input::RightKeyDown => {}
            game::Input::UpKeyDown => {
                menu.selected -= 1;
                menu.selected %= menu.items.len() as u8;
            }
            game::Input::DownKeyDown => {
                menu.selected += 1;
                menu.selected %= menu.items.len() as u8;
            }
            _ => {}
        }
    }
}
