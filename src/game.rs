use std::ops::Add;
use std::time::Instant;

use crate::block;
use crate::block::{Delta, Position};

type Dimension = Position;

pub const BOARD_SIZE: Dimension = Dimension { x: 10, y: 20 };
pub const FAST_FALL_RATE: u128 = 25; // milliseconds
pub const DEFAULT_FALL_RATE: u128 = 500; // milliseconds

type Board = Vec<Vec<Option<block::Color>>>;

pub enum Input {
    LeftKeyDown,
    RightKeyDown,
    UpKeyDown,
    DownKeyDown,
    DownKeyUp,
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

pub struct World {
    pub block: block::Block,
    pub next_block: Option<block::Block>,
    pub block_orientation: u8,
    pub board: Board,

    pub fall_rate_millis: u128, // elapsed ms before blocks drop to next row

    pub block_drop_clock: Instant,
}

pub fn initialise() -> World {
    let mut board = vec![vec![None; BOARD_SIZE.x as usize]; BOARD_SIZE.y as usize];
    let starting_block = block::spawn();
    paint_positions(&mut board, &starting_block.positions, starting_block.color);
    World {
        board: board,
        block: starting_block,
        block_orientation: 0,
        next_block: None,
        fall_rate_millis: DEFAULT_FALL_RATE,
        block_drop_clock: Instant::now(),
    }
}

fn paint_positions(board: &mut Board, positions: &Vec<Position>, color: block::Color) {
    for p in positions.iter() {
        board[p.y as usize][p.x as usize] = Some(color);
    }
}

fn unpaint_positions(board: &mut Board, positions: &Vec<Position>) {
    for p in positions.iter() {
        board[p.y as usize][p.x as usize] = None;
    }
}

pub fn update(event: &Option<Input>, mut world: &mut World) {
    if world.next_block.is_none() {
        // Note: Don't accept user input if a new block is spawned.
        if let Some(event) = event {
            match event {
                // NOTE: DownKeyUp needs to be first in the match call otherwise
                // the DownKeyUp event will be missed if the user is holding down
                // another key.
                Input::DownKeyUp => {
                    world.fall_rate_millis = DEFAULT_FALL_RATE;
                }
                Input::LeftKeyDown => {
                    handle_move(Delta { y: 0, x: -1 }, world);
                }
                Input::RightKeyDown => {
                    handle_move(Delta { y: 0, x: 1 }, world);
                }
                Input::UpKeyDown => {
                    handle_rotate(world);
                }
                Input::DownKeyDown => {
                    world.fall_rate_millis = FAST_FALL_RATE;
                }
            }
        }
    }

    if world.block_drop_clock.elapsed().as_millis() > world.fall_rate_millis {
        if let Some(block) = &world.next_block {
            world.block = block.clone();
            world.next_block = None;
            world.block_orientation = 0;
        }
        handle_move(Delta { y: 1, x: 0 }, world);

        delete_full_lines(world);
        world.block_drop_clock = Instant::now();
    }
}

fn handle_move(delta: Delta, mut world: &mut World) {
    // NOTE: Need to remove block from board, otherwise positions within the block
    // collide with other positions in the same block.
    unpaint_positions(&mut world.board, &world.block.positions);

    let new_positions = new_positions_from_delta(delta, &world.block, &world.board);
    if !new_positions.is_empty() {
        if block_finished_falling(&world.board, &new_positions) {
            world.next_block = Some(block::spawn());
        }
        world.block.positions = new_positions;
    }

    paint_positions(&mut world.board, &world.block.positions, world.block.color);
}

// Returns empty vec if block cannot be moved to the delta position.
fn new_positions_from_delta(delta: Delta, block: &block::Block, board: &Board) -> Vec<Position> {
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

fn block_finished_falling(board: &Board, positions: &Vec<Position>) -> bool {
    for position in positions.iter() {
        // Check at bottom of board.
        if position.y == BOARD_SIZE.y - 1 {
            return true;
        }

        // Check if anything is under the position.
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

fn handle_rotate(world: &mut World) {
    unpaint_positions(&mut world.board, &world.block.positions);
    let new_positions = block::rotate_block(&mut world.block, world.block_orientation);
    for p in new_positions.iter() {
        if !can_move_here(&world.board, *p) {
            paint_positions(&mut world.board, &world.block.positions, world.block.color);
            return;
        }
    }
    world.block.positions = new_positions;
    world.block_orientation = (world.block_orientation + 1) % 4;
    paint_positions(&mut world.board, &world.block.positions, world.block.color);
}

// Deletes full lines on board and returns te number of lines
// deleted
fn delete_full_lines(world: &mut World) -> i32 {
    let mut count = 0;
    for (y, row) in world.board.to_vec().iter().enumerate() {
        if is_row_full(row.to_vec()) {
            world.board.remove(y);
            world.board.insert(0, vec![None; BOARD_SIZE.x as usize]);
            count += 1;
        }
    }

    count
}

fn is_row_full(row: Vec<Option<block::Color>>) -> bool {
    !row.iter().any(|p| p.is_none())
}
