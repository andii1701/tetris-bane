use std::time::Instant;

type Dimension = Position;

pub static BLOCK_SIZE: i32 = 25;
pub static GAP: i32 = 1;

pub const BOARD_SIZE: Dimension = Dimension { x: 10, y: 20 };

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

#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub struct World {
    pub board: [[Option<Color>; BOARD_SIZE.x as usize]; BOARD_SIZE.y as usize],
    pub fall_rate_millis: u128, // elapsed ms before blocks drop to next row

    pub block_drop_clock: Instant,
}

pub fn initialise() -> World {
    let mut board = [[None; BOARD_SIZE.x as usize]; BOARD_SIZE.y as usize];
    board[0][0] = Some(Color { r: 255, g: 0, b: 0 });
    World {
        board: board,
        fall_rate_millis: 500,
        block_drop_clock: Instant::now(),
    }
}

pub fn update(event: &Option<InputEvent>, world: &mut World) {
    for y in 0..BOARD_SIZE.y {
        for x in 0..BOARD_SIZE.x {
            match world.board[y as usize][x as usize] {
                Some(color) => {
                    if let Some(event) = event {
                        match event {
                            InputEvent::Left => {
                                //world.position = new_position(world.position.x - 1, world.position.y)
                            }

                            InputEvent::Right => {
                                let position = new_position(x + 1, y);
                                let color = world.board[y as usize][x as usize];
                                world.board[y as usize][x as usize] = None;
                                world.board[position.y as usize][position.x as usize] = color;
                            }
                            _ => {}
                        }
                    }

                    if world.block_drop_clock.elapsed().as_millis() > world.fall_rate_millis {
                        //world.position = new_position(world.position.x, world.position.y + 1);
                        world.block_drop_clock = Instant::now();
                    }
                }
                None => {}
            }
        }
    }
}
// Returns a new position, if the block cannot move there returns passed position
fn new_position(x: i32, y: i32) -> Position {
    let x = match x {
        x if x < 0 => 0,
        x if x > BOARD_SIZE.x - 1 => BOARD_SIZE.x - 1,
        _ => x,
    };
    let y = match y {
        y if y < 0 => 0,
        y if y > BOARD_SIZE.y - 1 => BOARD_SIZE.y - 1,
        _ => y,
    };
    Position { x, y }
}
