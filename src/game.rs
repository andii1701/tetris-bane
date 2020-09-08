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

#[derive(Clone)]
pub struct Block {
    positions: Vec<Position>,
    color: Color,
}

pub struct World {
    pub block: Block,
    pub next_block: Option<Block>,
    pub board: Board,

    pub fall_rate_millis: u128, // elapsed ms before blocks drop to next row

    pub block_drop_clock: Instant,
}

pub fn initialise() -> World {
    let mut board = [[None; BOARD_SIZE.x as usize]; BOARD_SIZE.y as usize];
    let starting_block = Block {
        positions: vec![
            Position { y: 0, x: 0 },
            Position { y: 0, x: 1 },
            Position { y: 0, x: 2 },
            Position { y: 1, x: 1 },
        ],
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
        next_block: None,
        fall_rate_millis: 500,
        block_drop_clock: Instant::now(),
    }
}

fn generate_block() -> Block {
    Block {
        positions: vec![
            Position { y: 0, x: 0 },
            Position { y: 0, x: 1 },
            Position { y: 0, x: 2 },
            Position { y: 0, x: 3 },
        ],
        color: Color {
            r: 50,
            g: 50,
            b: 255,
        },
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

pub fn update(event: &Option<InputEvent>, mut world: &mut World) {
    if let Some(event) = event {
        match event {
            InputEvent::Left => {
                handle_move(Delta { y: 0, x: -1 }, &mut world);
            }

            InputEvent::Right => {
                handle_move(Delta { y: 0, x: 1 }, &mut world);
            }
            _ => {}
        }
    }

    if world.block_drop_clock.elapsed().as_millis() > world.fall_rate_millis {
        handle_move(Delta { y: 1, x: 0 }, &mut world);

        if let Some(block) = &world.next_block {
            world.block = block.clone();
            world.next_block = None;
        }
        world.block_drop_clock = Instant::now();
    }
}

pub fn handle_move(delta: Delta, mut world: &mut World) {
    fn new_positions_from_delta(delta: Delta, block: &Block, board: &Board) -> Vec<Position> {
        let mut new_positions: Vec<Position> = Vec::new();
        for position in block.positions.iter() {
            let new_position = *position + delta;
            if !can_move_here(&board, new_position) {
                new_positions.clear();
                return new_positions;
            }
            new_positions.push(new_position);
        }
        new_positions
    }

    unpaint_positions(&mut world.board, &world.block.positions);

    let new_positions = new_positions_from_delta(delta, &world.block, &world.board);
    if !new_positions.is_empty() {
        // Need to check if the block has finished falling before it's new positions
        // are painted to the board. Or internal block position will collide with
        // itself.
        if block_finished_falling(&world.board, &new_positions) {
            world.next_block = Some(generate_block());
        }
        world.block.positions = new_positions;
    }
    paint_positions(&mut world.board, &world.block.positions, world.block.color);
}

fn block_finished_falling(board: &Board, positions: &Vec<Position>) -> bool {
    for position in positions.iter() {
        // Check at bottom of board
        if position.y == BOARD_SIZE.y - 1 {
            return true;
        }

        // Check if anything is under the position
        if is_occupied(board, *position + Delta { x: 0, y: 1 }) {
            return true;
        }
    }
    false
}

fn can_move_here(board: &Board, p: Position) -> bool {
    if !(0..BOARD_SIZE.x).contains(&p.x) {
        return false;
    }
    if !(0..BOARD_SIZE.y).contains(&p.y) {
        return false;
    }
    return !is_occupied(board, p);
}

fn is_occupied(board: &Board, position: Position) -> bool {
    match board[position.y as usize][position.x as usize] {
        Some(_) => true,
        None => false,
    }
}
