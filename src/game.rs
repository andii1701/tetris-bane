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

Before the block "falls" a check is done to see if the Block has
finished falling. If the block has finished falling a new block is
spawned, complete lines are removed.

If a block cannot be spawned because the board is full. The block is
still painted on the board to indicate to the player how the game was
lost. Then the game ends.

The board has one extra hidden line at the top. This is done to
prevent an end game where an enpty line is on the top of the board. It
also has a nice side effect of the block appeeearing to drop into the
board.
*/

use std::ops::Add;
use std::time;

use crate::block;
use crate::block::{Block, Delta, Position};

use crate::menu;

type Dimension = Position;

pub const BANE_BOARD_SIZE: Dimension = Dimension { x: 18, y: 24 + 1 };
pub const CLASSIC_BOARD_SIZE: Dimension = Dimension { x: 10, y: 20 + 1 };
pub const FAST_FALL_RATE: u128 = 25; // milliseconds
pub const DEFAULT_FALL_RATE: u128 = 500; // milliseconds
pub const GAME_OVER_PAUSE: u128 = 1000; // milliseconds

type Board = Vec<Vec<Option<block::Color>>>;

pub enum Mode {
    Classic { label: String },
    Chill { label: String },
    Bane { label: String },
}

#[derive(PartialEq, Eq)]
pub enum State {
    Play,
    Menu,
    Quit,
    GameOver,
    Paused,
}

pub enum Input {
    LeftKeyDown,
    RightKeyDown,
    UpKeyDown,
    DownKeyDown,
    DownKeyUp,
    ReturnKeyDown,
    SpaceKeyDown,
    SpaceKeyUp,
    SKeyDown,
    SKeyUp,
    EscKeyDown,
    PKeyDown,
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
    pub game: Game,
    pub menu: menu::Menu,
    pub state: State,
    pub music_file: String,
}

pub struct Game {
    pub block: block::Block,
    pub block_orientation: u8,
    pub board: Board,
    pub fall_rate_millis: u128, // elapsed ms before blocks drop to next row
    pub block_drop_clock: time::Instant,
    pub score: i32,
}

pub fn initialise_world() -> World {
    let menu = menu::initialise();
    World {
        game: initialise_game(&menu.modes[menu.mode_selected]),
        menu: menu,
        state: State::Menu,
        music_file: "".to_string(),
    }
}

pub fn initialise_game(mode: &Mode) -> Game {
    let board_size = match mode {
        Mode::Bane { label: _ } => BANE_BOARD_SIZE,
        Mode::Classic { label: _ } | Mode::Chill { label: _ } => CLASSIC_BOARD_SIZE,
    };

    Game {
        board: vec![vec![None; board_size.x as usize]; board_size.y as usize],
        block: block::spawn(&mode),
        block_drop_clock: time::Instant::now(),
        fall_rate_millis: DEFAULT_FALL_RATE,
        score: 0,
        block_orientation: 0,
    }
}

pub fn update(event: &Option<Input>, world: &mut World) {
    let mut game = &mut world.game;
    if let Some(event) = event {
        match event {
            // NOTE: DownKeyUp needs to be first in the match call otherwise
            // the DownKeyUp event will be missed if the user is holding down
            // another key.
            Input::SKeyUp | Input::SpaceKeyUp | Input::DownKeyUp => {
                game.fall_rate_millis = DEFAULT_FALL_RATE;
            }
            Input::LeftKeyDown => {
                game.block.positions = move_block(&game.block, &game.board, Delta { y: 0, x: -1 });
            }
            Input::RightKeyDown => {
                game.block.positions = move_block(&game.block, &game.board, Delta { y: 0, x: 1 });
            }
            Input::UpKeyDown => {
                let (positions, orientation) =
                    rotate_block(&game.block, &game.board, game.block_orientation);
                game.block.positions = positions;
                game.block_orientation = orientation;
            }
            Input::DownKeyDown | Input::SpaceKeyDown | Input::SKeyDown => {
                game.fall_rate_millis = FAST_FALL_RATE;
            }
            Input::EscKeyDown | Input::PKeyDown => {
                world.state = State::Paused;
                world.menu.items =
                    menu::paused_menu_items(world.menu.music_toggle, world.menu.music_volume);
                world.menu.item_selected = 0;
                world.menu.title = "Paused".to_string();
            }
            _ => {}
        }
    }

    if game.block_drop_clock.elapsed().as_millis() > game.fall_rate_millis {
        game.block_drop_clock = time::Instant::now();

        // Having the game over state allows the player to
        // soak briefly in thier defeat. Rather then a sudden loss.
        if world.state == State::GameOver {
            world.state = State::Menu;
            return;
        }

        // NOTE: We want to handle the case when the block has finished falling in
        // the elapsed time. As this gives the player a chance
        // to quickly move the block at the last split second and "wedge" it into
        // gaps.
        if has_block_finished_falling(&game.board, &game.block) {
            game.board = paint_positions(&game.board, &game.block.positions, game.block.color);

            let spawned_block = block::spawn(&world.menu.modes[world.menu.mode_selected]);
            if !positions_empty_on_board(&spawned_block.positions, &game.board) {
                world.state = State::GameOver;
                world.menu.items = menu::menu_items(
                    &world.menu.modes,
                    world.menu.mode_selected,
                    world.menu.music_toggle,
                    world.menu.music_volume,
                );
                world.menu.item_selected = 0;
                game.fall_rate_millis = GAME_OVER_PAUSE;
            } else {
                game.block = spawned_block;
                game.block_orientation = 0;
                game.fall_rate_millis = DEFAULT_FALL_RATE;
            }
            let (board, score) = delete_full_lines(&game.board);
            game.board = board;
            game.score += score;
            return;
        }
        // Move block one square down.
        game.block.positions = move_block(&game.block, &game.board, Delta { y: 1, x: 0 });
    }
}

fn paint_positions(board: &Board, positions: &Vec<Position>, color: block::Color) -> Board {
    let mut board = board.clone();
    positions
        .iter()
        .for_each(|p| board[p.y as usize][p.x as usize] = Some(color));
    board
}

fn move_block(block: &Block, board: &Board, delta: Delta) -> Vec<Position> {
    let new_positions: Vec<Position> = block.positions.iter().map(|&p| p + delta).collect();
    if positions_empty_on_board(&new_positions, &board) {
        return new_positions;
    }
    block.positions.clone()
}

fn has_block_finished_falling(board: &Board, block: &Block) -> bool {
    block.positions.iter().any(|&p| {
        // Check at bottom of board.
        if p.y == board.len() as i32 - 1 {
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
    if !(0..board[0].len()).contains(&(p.x as usize)) {
        return false;
    }
    if !(0..board.len()).contains(&(p.y as usize)) {
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

fn rotate_block(block: &Block, board: &Board, orientation: u8) -> (Vec<Position>, u8) {
    let new_positions = block::rotate_block(&block, orientation);
    if new_positions.iter().all(|&p| can_move_here(&board, p)) {
        return (new_positions, (orientation + 1) % 4);
    }
    (block.positions.clone(), orientation)
}

// Deletes full lines on board and returns te number of lines
// deleted.
fn delete_full_lines(board: &Board) -> (Board, i32) {
    let mut board = board.clone();
    let mut count = 0;

    let full_row_indexes: Vec<usize> = board
        .iter()
        .enumerate()
        .filter(|(_, r)| is_row_full(r.to_vec()))
        .map(|(i, _)| i)
        .collect();

    full_row_indexes.iter().for_each(|&i| {
        // Remove full row
        board.remove(i);
        // insert new blank row at the top of the board
        board.insert(0, vec![None; board[0].len()]);
    });

    count += full_row_indexes.len();
    (board, count as i32)
}

fn positions_empty_on_board(positions: &Vec<Position>, board: &Board) -> bool {
    positions.iter().all(|&p| can_move_here(&board, p))
}

fn is_row_full(row: Vec<Option<block::Color>>) -> bool {
    !row.iter().any(|p| p.is_none())
}
