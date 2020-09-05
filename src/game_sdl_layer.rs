// SDL layer for the game. Responsible for passing a rendered surface and sounds to be played
// back to main loop.

use crate::game::{InputEvent, World, GRID_SIZE};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn update_and_render(canvas: &mut WindowCanvas, event: &Option<InputEvent>, world: &mut World) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    if let Some(event) = event {
        match event {
            InputEvent::Left => world.position.x -= GRID_SIZE as i32,
            InputEvent::Right => world.position.x += GRID_SIZE as i32,
            InputEvent::Up => world.position.y -= GRID_SIZE as i32,
            InputEvent::Down => world.position.y += GRID_SIZE as i32,
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
