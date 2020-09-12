// SDL layer for the game. Responsible for passing a rendered surface and sounds to be played
// back to main loop.

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub const BLOCK_SIZE: i32 = 25;
pub const GAP: i32 = 1;

use crate::game;

use crate::block;

pub fn update_and_render(
    canvas: &mut WindowCanvas,
    event: &Option<game::Input>,
    world: &mut game::World,
) {
    // update
    game::update(event, world);

    // render
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Draw board
    let (canvas_width, canvas_height) = canvas.output_size().unwrap();
    let canvas_mid = block::Position {
        x: (canvas_width as f32 / 2.) as i32,
        y: (canvas_height as f32 / 2.) as i32,
    };
    let board_width = (BLOCK_SIZE + GAP) * game::BOARD_SIZE.x;
    let board_height = (BLOCK_SIZE + GAP) * game::BOARD_SIZE.y;
    let board_origin = block::Position {
        x: canvas_mid.x - (board_width as f32 / 2.) as i32,
        y: canvas_mid.y - (board_height as f32 / 2.) as i32,
    };
    (0..game::BOARD_SIZE.y).for_each(|y| {
        (0..game::BOARD_SIZE.x).for_each(|x| {
            match world.board[y as usize][x as usize] {
                Some(color) => canvas.set_draw_color(game_color_to_sdl_color(color)),
                None => canvas.set_draw_color(Color::RGB(50, 50, 50)),
            }
            canvas
                .fill_rect(Rect::new(
                    board_origin.x + (BLOCK_SIZE + GAP) * x,
                    board_origin.y + (BLOCK_SIZE + GAP) * y,
                    BLOCK_SIZE as u32,
                    BLOCK_SIZE as u32,
                ))
                .unwrap();
        })
    });
    // Draw active block on the board
    canvas.set_draw_color(game_color_to_sdl_color(world.block.color));
    world.block.positions.iter().for_each(|&p| {
        canvas
            .fill_rect(Rect::new(
                board_origin.x + (BLOCK_SIZE + GAP) * p.x,
                board_origin.y + (BLOCK_SIZE + GAP) * p.y,
                BLOCK_SIZE as u32,
                BLOCK_SIZE as u32,
            ))
            .unwrap();
    });
}

fn game_color_to_sdl_color(color: block::Color) -> Color {
    Color::RGB(color.r, color.g, color.b)
}
