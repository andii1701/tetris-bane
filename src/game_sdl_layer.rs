use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;

use crate::game;

use crate::block;

// offset from bottom of canvas
const SCORE_OFFSET_X: u32 = 70;

const BLOCK_SIZE: i32 = 25;
const GAP: i32 = 1;
const TEXT_COLOR: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
};

pub fn update_and_render(
    canvas: &mut WindowCanvas,
    font: &Font,
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
    let canvas_mid = Point::new(
        (canvas_width as f32 / 2.) as i32,
        (canvas_height as f32 / 2.) as i32,
    );
    let board_width = (BLOCK_SIZE + GAP) * game::BOARD_SIZE.x;
    let board_height = (BLOCK_SIZE + GAP) * game::BOARD_SIZE.y;
    let board_origin = Point::new(
        canvas_mid.x - (board_width as f32 / 2.) as i32,
        canvas_mid.y - (board_height as f32 / 2.) as i32,
    );
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

    // Draw score board
    let texture_creator = canvas.texture_creator();
    let font_surface = font
        .render(&format!("{}", world.score))
        .blended(TEXT_COLOR)
        .unwrap();
    let texture = font_surface.as_texture(&texture_creator).unwrap();
    let mut score_rect = font_surface.rect();
    let score_board_position = Point::new(
        canvas_mid.x - (score_rect.width() as f32 / 2.) as i32,
        (canvas_height - SCORE_OFFSET_X) as i32,
    );

    score_rect.reposition(score_board_position);
    canvas.copy(&texture, None, score_rect).unwrap();
}

fn game_color_to_sdl_color(color: block::Color) -> Color {
    Color::RGB(color.r, color.g, color.b)
}
