use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::game;

use crate::block;

const GAME_FONT_PATH: &str = "assets/fonts/muli/Muli.ttf";

const BLOCK_SIZE: i32 = 25;
const GAP: i32 = 1;
const TEXT_COLOR: Color = Color {
    r: 70,
    g: 70,
    b: 70,
    a: 255,
};

const BOARD_COLOR: Color = Color {
    r: 50,
    g: 50,
    b: 50,
    a: 255,
};

pub struct GameFonts<'ttf> {
    score: Font<'ttf, 'static>,
    title: Font<'ttf, 'static>,
    menu: Font<'ttf, 'static>,
}

pub fn initialise_fonts(ttf_context: &Sdl2TtfContext) -> GameFonts {
    let game_font_path: &Path = Path::new(GAME_FONT_PATH);
    let score_font = ttf_context.load_font(game_font_path, 22).unwrap();
    let title_font = ttf_context.load_font(game_font_path, 30).unwrap();
    let menu_font = ttf_context.load_font(game_font_path, 25).unwrap();
    GameFonts {
        score: score_font,
        title: title_font,
        menu: menu_font,
    }
}

pub fn update_and_render(
    mut canvas: &mut WindowCanvas,
    fonts: &GameFonts,
    event: &Option<game::Input>,
    mut world: &mut game::World,
) {
    let menu_mode = false;
    if menu_mode {
        // menu::update
        render_menu(&mut canvas, &fonts, &mut world);
    } else {
        game::update(event, world);
        render_game(&mut canvas, &fonts, &mut world);
    };
    // update
}

fn render_game(canvas: &mut WindowCanvas, fonts: &GameFonts, world: &mut game::World) {
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
                None => canvas.set_draw_color(BOARD_COLOR),
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
    let font_surface = fonts
        .score
        .render(&format!("{}", world.score))
        .blended(TEXT_COLOR)
        .unwrap();
    let texture = font_surface.as_texture(&texture_creator).unwrap();
    let mut score_rect = font_surface.rect();
    let score_board_origin = Point::new(
        board_origin.x + board_width - score_rect.width() as i32 - 1,
        board_origin.y + board_height,
    );

    score_rect.reposition(score_board_origin);
    canvas.copy(&texture, None, score_rect).unwrap();
}

fn render_menu(canvas: &mut WindowCanvas, fonts: &GameFonts, _world: &mut game::World) {
    // render
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let (canvas_width, canvas_height) = canvas.output_size().unwrap();

    let texture_creator = canvas.texture_creator();

    // Draw title

    let font_surface = fonts
        .title
        .render(&"Tetris Bane")
        .blended(TEXT_COLOR)
        .unwrap();
    let texture = font_surface.as_texture(&texture_creator).unwrap();
    let mut title_rect = font_surface.rect();
    let title_origin = Point::new(
        ((canvas_width as f32 / 2.) - (title_rect.width() as f32 / 2.)) as i32,
        ((canvas_height as f32 / 2.) - (title_rect.height() as f32 / 2.)) as i32,
    );
    title_rect.reposition(title_origin);
    canvas.copy(&texture, None, title_rect).unwrap();

    //Draw menu

    let font_surface = fonts.menu.render(&"Play").blended(TEXT_COLOR).unwrap();
    let texture = font_surface.as_texture(&texture_creator).unwrap();
    let mut menu_rect = font_surface.rect();
    let menu_origin = Point::new(
        ((canvas_width as f32 / 2.) - (menu_rect.width() as f32 / 2.)) as i32,
        title_rect.y + title_rect.height() as i32 + 10,
    );

    menu_rect.reposition(menu_origin);
    canvas.copy(&texture, None, menu_rect).unwrap();
}

fn game_color_to_sdl_color(color: block::Color) -> Color {
    Color::RGB(color.r, color.g, color.b)
}
