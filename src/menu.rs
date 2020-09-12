pub struct Menu<'a> {
    pub items: Vec<&'a str>,
    pub selected: u8,
}

pub fn initialise() -> Menu<'static> {
    Menu {
        items: vec!["Play", "Mode: Classic", "Quit"],
        selected: 0,
    }
}
