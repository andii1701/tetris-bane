/*

The way the game works.

The "board" is represented by a double subscript array of Vecs.  Each
position on the board is an Option<Color>. If the Color is None then
the position is blank otherwise that Color is drawn in the positions.

Each Tetris Block consists of a Vec of positions and a Color. A
Position has a X and Y component. The block positions are "painted" on
the board each game loop.

During the each game loop when the Block is moved or rotated by the
player or the block "falls". The block is removed from the board, then
the Block's positions are updated and each position is checked to see
if the move is possible. If the move is possible the new positions are
assigned to the Block. Otherwise the new positions are
discarded. After movement logic is completed the Block's positions are
re-painted on the Board.

The advantage of this approach is that each position in the Block can
be treated independently with concern for other positions in the
block. It also make rendering simple as the render only has to render
the board. The disadvantage is that you have to remember to unpaint
and re-paint the block before and after each move is attempted.

Before the block "falls" one square a check is done to see if the
Block has finished falling. If the block has finished falling a new
block is spawned, complete lines are removed.

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
        // the elapsed time check as it gives the player a chance
        // to quickly move the block at the last split second and "wedge" it into
        // gaps.
        if has_block_finished_falling(&mut world.board, &world.block) {
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

fn handle_move(delta: Delta, mut block: &mut Block, mut board: &mut Board) {
    // NOTE: Need to remove block from board, otherwise positions within the block
    // collide with other positions in the same block.
    unpaint_positions(&mut board, &block.positions);

    let new_positions: Vec<Position> = block.positions.iter().map(|&p| p + delta).collect();
    if positions_empty_on_board(&new_positions, &board) {
        block.positions = new_positions;
    }

    paint_positions(&mut board, &block.positions, block.color);
}

fn has_block_finished_falling(mut board: &mut Board, block: &Block) -> bool {
    // NOTE: Need to remove block from board, otherwise positions within the block
    // collide with other positions in the same block.
    unpaint_positions(&mut board, &block.positions);

    let is_finished_falling = block.positions.iter().any(|&p| {
        // Check at bottom of board.
        if p.y == BOARD_SIZE.y - 1 {
            return true;
        }
        // Check if anything is under the position.
        if is_occupied(board, p + Delta { x: 0, y: 1 }) {
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
        .all(|&p| can_move_here(&world.board, p))
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
