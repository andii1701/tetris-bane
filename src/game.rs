use std::ops::Add;
use std::time::Instant;

use crate::block;
use crate::block::{Block, Delta, Position};

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
    pub block_orientation: u8,
    pub board: Board,

    pub fall_rate_millis: u128, // elapsed ms before blocks drop to next row

    pub block_drop_clock: Instant,

    pub score: i32,
}

pub fn initialise() -> World {
    let mut board = vec![vec![None; BOARD_SIZE.x as usize]; BOARD_SIZE.y as usize];
    let starting_block = block::spawn();
    paint_positions(&mut board, &starting_block.positions, starting_block.color);
    World {
        board: board,
        block: starting_block,
        block_orientation: 0,
        fall_rate_millis: DEFAULT_FALL_RATE,
        block_drop_clock: Instant::now(),
        score: 0,
    }
}

fn paint_positions(board: &mut Board, positions: &Vec<Position>, color: block::Color) {
    positions
        .iter()
        .for_each(|p| board[p.y as usize][p.x as usize] = Some(color));
}

fn unpaint_positions(board: &mut Board, positions: &Vec<Position>) {
    positions
        .iter()
        .for_each(|p| board[p.y as usize][p.x as usize] = None);
}

pub fn update(event: &Option<Input>, world: &mut World) {
    // NOTE: Don't accept user input if a new block is spawned.
    if let Some(event) = event {
        match event {
            // NOTE: DownKeyUp needs to be first in the match call otherwise
            // the DownKeyUp event will be missed if the user is holding down
            // another key.
            Input::DownKeyUp => {
                world.fall_rate_millis = DEFAULT_FALL_RATE;
            }
            Input::LeftKeyDown => {
                handle_move(Delta { y: 0, x: -1 }, &mut world.block, &mut world.board);
            }
            Input::RightKeyDown => {
                handle_move(Delta { y: 0, x: 1 }, &mut world.block, &mut world.board);
            }
            Input::UpKeyDown => {
                handle_rotate(world);
            }
            Input::DownKeyDown => {
                world.fall_rate_millis = FAST_FALL_RATE;
            }
        }
    }

    if world.block_drop_clock.elapsed().as_millis() > world.fall_rate_millis {
        world.block_drop_clock = Instant::now();

        // NOTE: We want to handle if the block has finished falling in
        // the elapsed time check as gives the user an chance
        // to quickly move the block at the last split second and "wedge" it into
        // gaps.
        if has_block_finished_falling(&mut world.board, &world.block) {
            world.block = block::spawn();
            world.block_orientation = 0;
            world.fall_rate_millis = DEFAULT_FALL_RATE;
            world.score += delete_full_lines(world);
            return;
        }
        // Move block one square down.
        handle_move(Delta { y: 1, x: 0 }, &mut world.block, &mut world.board);
    }
}

fn handle_move(delta: Delta, mut block: &mut Block, mut board: &mut Board) {
    // NOTE: Need to remove block from board, otherwise positions within the block
    // collide with other positions in the same block.
    unpaint_positions(&mut board, &block.positions);

    let new_positions: Vec<Position> = block.positions.iter().map(|p| *p + delta).collect();
    if new_positions.iter().all(|p| can_move_here(&board, *p)) {
        block.positions = new_positions;
    }

    paint_positions(&mut board, &block.positions, block.color);
}

fn has_block_finished_falling(mut board: &mut Board, block: &Block) -> bool {
    // NOTE: Need to remove block from board, otherwise positions within the block
    // collide with other positions in the same block.
    unpaint_positions(&mut board, &block.positions);

    let is_finished_falling = block.positions.iter().any(|p| {
        // Check at bottom of board.
        if p.y == BOARD_SIZE.y - 1 {
            return true;
        }
        // Check if anything is under the position.
        if is_occupied(board, *p + Delta { x: 0, y: 1 }) {
            return true;
        }
        false
    });

    paint_positions(&mut board, &block.positions, block.color);
    is_finished_falling
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
    // NOTE: Need to remove block from board, otherwise positions within the block
    // collide with other positions in the same block.
    unpaint_positions(&mut world.board, &world.block.positions);
    let new_positions = block::rotate_block(&mut world.block, world.block_orientation);
    if new_positions
        .iter()
        .all(|p| can_move_here(&world.board, *p))
    {
        world.block.positions = new_positions;
        world.block_orientation = (world.block_orientation + 1) % 4;
    }
    paint_positions(&mut world.board, &world.block.positions, world.block.color);
}

// Deletes full lines on board and returns te number of lines
// deleted.
fn delete_full_lines(world: &mut World) -> i32 {
    let mut count = 0;

    let full_rows: Vec<usize> = world
        .board
        .iter()
        .enumerate()
        .filter(|(_, r)| is_row_full(r.to_vec()))
        .map(|(i, _)| i)
        .collect();

    for index in &full_rows {
        // Remove full row
        world.board.remove(*index);
        // insert new blank row at the top of the board
        world.board.insert(0, vec![None; BOARD_SIZE.x as usize]);
    }

    count += full_rows.len();
    count as i32
}

fn is_row_full(row: Vec<Option<block::Color>>) -> bool {
    !row.iter().any(|p| p.is_none())
}
