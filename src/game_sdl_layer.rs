// SDL layer for the game. Responsible for passing a rendered surface and sounds to be played
// back to main loop.

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;

pub fn update_and_render(surface: &mut Surface) {
    let sur = Surface::new(1024, 768, PixelFormatEnum::RGBA32).unwrap();
    let mut canvas_surface = Canvas::from_surface(sur).unwrap();
    canvas_surface.set_draw_color(Color::RGB(0, 0, 0));
    canvas_surface.clear();
    canvas_surface.set_draw_color(Color::RGB(255, 0, 0));
    canvas_surface
        .fill_rect(Rect::new(10, 100, 100, 100))
        .unwrap();
    *surface = canvas_surface.into_surface();
}
