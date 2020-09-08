use std::ops::Add;
use std::time::Instant;

pub const BLOCK_SIZE: i32 = 25;
pub const GAP: i32 = 1;
pub const BOARD_SIZE: Dimension = Dimension { x: 10, y: 20 };

type Dimension = Position;
type Delta = Position;
type Board = [[Option<Color>; BOARD_SIZE.x as usize]; BOARD_SIZE.y as usize];

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

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Block {
    positions: Vec<Position>,
    color: Color,
}

pub struct World {
    pub block: Block,
    pub board: Board,

    pub fall_rate_millis: u128, // elapsed ms before blocks drop to next row

    pub block_drop_clock: Instant,
}

pub fn initialise() -> World {
    let mut board = [[None; BOARD_SIZE.x as usize]; BOARD_SIZE.y as usize];
    let starting_block = Block {
        positions: vec![Position { y: 0, x: 0 }, Position { y: 0, x: 1 }],
        color: Color {
            r: 50,
            g: 255,
            b: 50,
        },
    };
    paint_positions(&mut board, &starting_block.positions, starting_block.color);
    World {
        board: board,
        block: starting_block,
        fall_rate_millis: 500,
        block_drop_clock: Instant::now(),
    }
}

pub fn paint_positions(board: &mut Board, positions: &Vec<Position>, color: Color) {
    for p in positions.iter() {
        board[p.y as usize][p.x as usize] = Some(color);
    }
}

pub fn unpaint_positions(board: &mut Board, positions: &Vec<Position>) {
    for p in positions.iter() {
        board[p.y as usize][p.x as usize] = None;
    }
}

pub fn attempt_to_move_block(delta: Delta, block: &mut Block, board: &mut Board) {
    fn new_positions_from_delta(delta: Delta, block: &Block) -> Vec<Position> {
        let mut new_positions: Vec<Position> = Vec::new();
        for position in block.positions.iter() {
            let new_position = *position + delta;
            if !can_move_here(new_position) {
                new_positions.clear();
                return new_positions;
            }
            new_positions.push(new_position);
        }
        new_positions
    }

    let new_positions = new_positions_from_delta(delta, block);

    if !new_positions.is_empty() {
        unpaint_positions(board, &block.positions);
        block.positions = new_positions;
        paint_positions(board, &block.positions, block.color);
    }
}

pub fn update(event: &Option<InputEvent>, world: &mut World) {
    if let Some(event) = event {
        match event {
            InputEvent::Left => {}

            InputEvent::Right => {}
            _ => {}
        }
    }

    if world.block_drop_clock.elapsed().as_millis() > world.fall_rate_millis {
        attempt_to_move_block(Delta { y: 1, x: 0 }, &mut world.block, &mut world.board);
        world.block_drop_clock = Instant::now();
    }
}

fn can_move_here(p: Position) -> bool {
    if !(0..BOARD_SIZE.x).contains(&p.x) {
        return false;
    }
    if !(0..BOARD_SIZE.y).contains(&p.y) {
        return false;
    }
    true
}
