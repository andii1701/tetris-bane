/*

The way the game works.

The "board" is represented by a double subscript array of Vecs. Each
position on the board is an Option<Color>. If the Color is None then
the position is blank otherwise that Color is drawn in the sdl_layer.

Each Tetris Block consists of a Vec of positions and a Color. A
Position has a X and Y component.

During each game loop when the Block is moved or rotated by the player
or the block "falls", the block's new positions are checked to see if
the move can be made, if the move is possible the Block is assigned
the new positions, otherwise the new positions are discarded.

Before the block "falls" one square a check is done to see if the
Block has finished falling. If the block has finished falling a new
block is spawned, complete lines are removed and game end check is
made.

*/

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
    World {
        board: vec![vec![None; BOARD_SIZE.x as usize]; BOARD_SIZE.y as usize],
        block: block::spawn(),
        block_orientation: 0,
        fall_rate_millis: DEFAULT_FALL_RATE,
        block_drop_clock: Instant::now(),
        score: 0,
    }
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
                world.block_orientation =
                    handle_rotate(&world.board, &mut world.block, world.block_orientation);
            }
            Input::DownKeyDown => {
                world.fall_rate_millis = FAST_FALL_RATE;
            }
        }
    }

    if world.block_drop_clock.elapsed().as_millis() > world.fall_rate_millis {
        world.block_drop_clock = Instant::now();

        // NOTE: We want to handle if the block has finished falling in
        // the elapsed time check as it gives the player a chance
        // to quickly move the block at the last split second and "wedge" it into
        // gaps.
        if has_block_finished_falling(&mut world.board, &world.block) {
            paint_positions(&mut world.board, &world.block.positions, world.block.color);

            let spawned_block = block::spawn();
            if !positions_empty_on_board(&spawned_block.positions, &world.board) {
                println!("Game Over!");
                // Paint the new block on the board to show how the player lost. If this
                // does not happen the game will end with an empty line
                paint_positions(
                    &mut world.board,
                    &spawned_block.positions,
                    spawned_block.color,
                );
            } else {
                world.block = spawned_block;
                world.block_orientation = 0;
                world.fall_rate_millis = DEFAULT_FALL_RATE;
            }
            world.score += delete_full_lines(world);
            return;
        }
        // Move block one square down.
        handle_move(Delta { y: 1, x: 0 }, &mut world.block, &mut world.board);
    }
}

fn paint_positions(board: &mut Board, positions: &Vec<Position>, color: block::Color) {
    positions
        .iter()
        .for_each(|p| board[p.y as usize][p.x as usize] = Some(color));
}

fn handle_move(delta: Delta, block: &mut Block, board: &Board) {
    let new_positions: Vec<Position> = block.positions.iter().map(|&p| p + delta).collect();
    if positions_empty_on_board(&new_positions, &board) {
        block.positions = new_positions;
    }
}

fn has_block_finished_falling(board: &Board, block: &Block) -> bool {
    block.positions.iter().any(|&p| {
        // Check at bottom of board.
        if p.y == BOARD_SIZE.y - 1 {
            return true;
        }
        // Check if anything is under the position.
        if is_occupied(board, p + Delta { x: 0, y: 1 }) {
            return true;
        }
        false
    })
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

fn handle_rotate(board: &Board, mut block: &mut Block, orientation: u8) -> u8 {
    let mut orientation = orientation;
    let new_positions = block::rotate_block(&mut block, orientation);
    if new_positions.iter().all(|&p| can_move_here(&board, p)) {
        block.positions = new_positions;
        orientation = (orientation + 1) % 4;
    }
    orientation
}

// Deletes full lines on board and returns te number of lines
// deleted.
fn delete_full_lines(world: &mut World) -> i32 {
    let mut count = 0;

    let full_row_indexes: Vec<usize> = world
        .board
        .iter()
        .enumerate()
        .filter(|(_, r)| is_row_full(r.to_vec()))
        .map(|(i, _)| i)
        .collect();

    full_row_indexes.iter().for_each(|&i| {
        // Remove full row
        world.board.remove(i);
        // insert new blank row at the top of the board
        world.board.insert(0, vec![None; BOARD_SIZE.x as usize]);
    });

    count += full_row_indexes.len();
    count as i32
}

fn positions_empty_on_board(positions: &Vec<Position>, board: &Board) -> bool {
    positions.iter().all(|&p| can_move_here(&board, p))
}

fn is_row_full(row: Vec<Option<block::Color>>) -> bool {
    !row.iter().any(|p| p.is_none())
}
