use std::time::Instant;

type Dimension = Position;

pub static BLOCK_SIZE: i32 = 25;
pub static GAP: i32 = 1;

pub static BOARD_SIZE: Dimension = Dimension { x: 10, y: 20 };

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
    pub fall_rate_millis: u128, // elapsed ms before blocks drop to next row

    pub block_drop_clock: Instant,
}

pub fn initialise() -> World {
    World {
        position: Position { x: 0, y: 0 },
        fall_rate_millis: 500,
        block_drop_clock: Instant::now(),
    }
}
