// SDL layer for the game. Responsible for passing a rendered surface and sounds to be played
// back to main loop.

use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::game::{InputEvent, World, GRID_SIZE};

pub fn update_and_render(canvas: &mut WindowCanvas, event: &Option<InputEvent>, world: &mut World) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    if let Some(event) = event {
        match event {
            InputEvent::Left => world.position.x -= GRID_SIZE as i32,
            InputEvent::Right => world.position.x += GRID_SIZE as i32,
            _ => {}
        }
    }

    if world.block_drop_clock.elapsed().as_millis() > world.fall_rate_millis {
        world.position.y += GRID_SIZE as i32;
        world.block_drop_clock = Instant::now();
    }

    // Draw board
    canvas.set_draw_color(Color::RGB(50, 50, 50));
    for y in 0..20 {
        for x in 0..10 {
            canvas
                .fill_rect(Rect::new(
                    100 + (GRID_SIZE as i32 + 2) * x,
                    100 + (GRID_SIZE as i32 + 2) * y,
                    GRID_SIZE,
                    GRID_SIZE,
                ))
                .unwrap();
        }
    }

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas
        .fill_rect(Rect::new(
            world.position.x,
            world.position.y,
            GRID_SIZE,
            GRID_SIZE,
        ))
        .unwrap();
}
