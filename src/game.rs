pub static GRID_SIZE: u32 = 20;

pub enum InputEvent {
    Left,
    Right,
    Up,
    Down,
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct World {
    pub position: Position,
    pub fall_rate: f32, // elapsed seconds block take to fall
    pub elapsed_time: f32,
}

pub fn initialise() -> World {
    World {
        position: Position { x: 10, y: 10 },
        fall_rate: 0.5,
        elapsed_time: 99.0,
    }
}
