// SDL layer for the game. Responsible for passing a rendered surface and sounds to be played
// back to main loop.

use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::game::{InputEvent, Position, World, BLOCK_SIZE, BOARD_SIZE, GAP};

pub fn update_and_render(canvas: &mut WindowCanvas, event: &Option<InputEvent>, world: &mut World) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    if let Some(event) = event {
        match event {
            InputEvent::Left => world.position.x -= 1,
            InputEvent::Right => world.position.x += 1,
            _ => {}
        }
    }

    if world.block_drop_clock.elapsed().as_millis() > world.fall_rate_millis {
        world.position.y += 1;
        world.block_drop_clock = Instant::now();
    }

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
