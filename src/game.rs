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
#[derive(Copy, Clone)]
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

                            InputEvent::Right => {}
                            _ => {}
                        }
                    }

                    if world.block_drop_clock.elapsed().as_millis() > world.fall_rate_millis {
                        let new_position = Position { y: y + 1, x: x };
                        if can_move_here(new_position) {
                            let tmp = world.board[y as usize][x as usize];
                            world.board[y as usize][x as usize] = None;
                            world.board[new_position.y as usize][new_position.x as usize] = tmp;
                            world.block_drop_clock = Instant::now();
                        }
                    }
                }
                None => {}
            }
        }
    }
}
//
fn can_move_here(p: Position) -> bool {
    if !(0..BOARD_SIZE.x).contains(&p.x) {
        return false;
    }
    if !(0..BOARD_SIZE.y).contains(&p.y) {
        return false;
    }
    true
}
