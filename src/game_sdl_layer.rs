// SDL layer for the game. Responsible for passing a rendered surface and sounds to be played
// back to main loop.

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::game;
use crate::game::{InputEvent, Position, World, BLOCK_SIZE, BOARD_SIZE, GAP};

pub fn update_and_render(canvas: &mut WindowCanvas, event: &Option<InputEvent>, world: &mut World) {
    // update
    game::update(event, world);

    // render

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Draw board
    canvas.set_draw_color(Color::RGB(50, 50, 50));
    let (canvas_width, canvas_height) = canvas.output_size().unwrap();
    let canvas_mid = Position {
        x: (canvas_width as f32 / 2.) as i32,
        y: (canvas_height as f32 / 2.) as i32,
    };
    let board_width = (BLOCK_SIZE + GAP) * BOARD_SIZE.x;
    let board_height = (BLOCK_SIZE + GAP) * BOARD_SIZE.y;
    let board_origin = Position {
        x: canvas_mid.x - (board_width as f32 / 2.) as i32,
        y: canvas_mid.y - (board_height as f32 / 2.) as i32,
    };
    for y in 0..BOARD_SIZE.y {
        for x in 0..BOARD_SIZE.x {
            canvas
                .fill_rect(Rect::new(
                    board_origin.x + (BLOCK_SIZE + GAP) * x,
                    board_origin.y + (BLOCK_SIZE + GAP) * y,
                    BLOCK_SIZE as u32,
                    BLOCK_SIZE as u32,
                ))
                .unwrap();
        }
    }

    // draw block
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas
        .fill_rect(Rect::new(
            board_origin.x + (BLOCK_SIZE + GAP) * world.position.x,
            board_origin.y + (BLOCK_SIZE + GAP) * world.position.y,
            BLOCK_SIZE as u32,
            BLOCK_SIZE as u32,
        ))
        .unwrap();
}
