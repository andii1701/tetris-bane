// SDL layer for the game. Responsible for passing a rendered surface and sounds to be played
// back to main loop.

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub enum InputEvent {
    Left,
    Right,
    Up,
    Down,
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn update_and_render(
    canvas: &mut WindowCanvas,
    event: &Option<InputEvent>,
    position: &mut Position,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    if let Some(event) = event {
        match event {
            InputEvent::Left => position.x -= 10,
            InputEvent::Right => position.x += 10,
            InputEvent::Up => position.y -= 10,
            InputEvent::Down => position.y += 10,
        }
    }

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas
        .fill_rect(Rect::new(position.x, position.y, 100, 100))
        .unwrap();
}
