use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::game;
use crate::menu;

use crate::block;

const GAME_FONT_PATH: &str = "assets/fonts/muli/Muli.ttf";
const SETTINGS_FONT_PATH: &str = "assets/fonts/JetBrainsMono-2.001/ttf/JetBrainsMono-Regular.ttf";

const BLOCK_SIZE: i32 = 25;
const GAP: i32 = 1;

const DEFAULT_TEXT_COLOR: Color = Color {
    r: 120,
    g: 120,
    b: 120,
    a: 255,
};

const BOARD_COLOR: Color = Color {
    r: 40,
    g: 40,
    b: 40,
    a: 255,
};

pub struct GameFonts<'ttf> {
    score: Font<'ttf, 'static>,
    title: Font<'ttf, 'static>,
    settings: Font<'ttf, 'static>,
}

pub fn initialise_fonts(ttf_context: &Sdl2TtfContext) -> GameFonts {
    let game_font_path: &Path = Path::new(GAME_FONT_PATH);
    let score_font = ttf_context.load_font(game_font_path, 22).unwrap();
    let title_font = ttf_context.load_font(game_font_path, 50).unwrap();

    let settings_font_path: &Path = Path::new(SETTINGS_FONT_PATH);
    let settings_font = ttf_context.load_font(settings_font_path, 30).unwrap();
    GameFonts {
        score: score_font,
        title: title_font,
        settings: settings_font,
    }
}

pub fn update_and_render(
    mut canvas: &mut WindowCanvas,
    fonts: &GameFonts,
    event: &Option<game::Input>,
    mut world: &mut game::World,
) {
    match world.state {
        game::State::Menu | game::State::Paused => {
            menu::update(event, &mut world);
            render_menu(&mut canvas, fonts, &world.menu);
        }
        game::State::Play | game::State::GameOver => {
            match game::update(event, world) {
                game::State::Paused => {
                    world.state = game::State::Paused;
                    world.menu.items =
                        menu::paused_menu_items(world.menu.music_toggle, world.menu.music_volume);
                    world.menu.item_selected = 0;
                    world.menu.title = "Paused".to_string();
                }
                game::State::Menu => {
                    world.state = game::State::Menu;
                }
                game::State::GameOver => {
                    world.menu.items = menu::menu_items(
                        &world.game,
                        world.menu.music_toggle,
                        world.menu.music_volume,
                    );
                    world.menu.item_selected = 0;
                    world.state = game::State::GameOver;
                }
                _ => {}
            }
            render_game(&mut canvas, fonts, &world.game);
        }

        game::State::Quit => {}
    }
}

fn render_game(canvas: &mut WindowCanvas, fonts: &GameFonts, game: &game::Game) {
    // render
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let (canvas_width, canvas_height) = canvas.output_size().unwrap();
    let canvas_mid = Point::new(
        (canvas_width as f32 / 2.) as i32,
        (canvas_height as f32 / 2.) as i32,
    );

    let board_width = (BLOCK_SIZE + GAP) * game.board[0].len() as i32;
    let board_height = (BLOCK_SIZE + GAP) * game.board.len() as i32;
    let board_origin = Point::new(
        canvas_mid.x - (board_width as f32 / 2.) as i32,
        canvas_mid.y - (board_height as f32 / 2.) as i32,
    );
    // Draw board
    {
        // Don't draw the top row
        (1..game.board.len()).for_each(|y| {
            (0..game.board[0].len()).for_each(|x| {
                match game.board[y as usize][x as usize] {
                    Some(color) => canvas.set_draw_color(game_color_to_sdl_color(color)),
                    None => canvas.set_draw_color(BOARD_COLOR),
                }
                canvas
                    .fill_rect(Rect::new(
                        board_origin.x + (BLOCK_SIZE + GAP) * x as i32,
                        board_origin.y + (BLOCK_SIZE + GAP) * y as i32,
                        BLOCK_SIZE as u32,
                        BLOCK_SIZE as u32,
                    ))
                    .unwrap();
            })
        });
    }
    // Draw active block on the board
    {
        canvas.set_draw_color(game_color_to_sdl_color(game.block.color));

        game.block
            .positions
            .iter()
            .filter(|p| p.y != 0) // Don't draw if on the top row
            .for_each(|&p| {
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

    // Draw score board
    {
        let texture_creator = canvas.texture_creator();
        let font_surface = fonts
            .score
            .render(&format!("{}", game.score))
            .blended(DEFAULT_TEXT_COLOR)
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
}

fn render_menu(canvas: &mut WindowCanvas, fonts: &GameFonts, menu: &menu::Menu) {
    let selected_text_color: Color = Color {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
    };

    // render
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let (canvas_width, canvas_height) = canvas.output_size().unwrap();

    let texture_creator = canvas.texture_creator();

    // Draw title
    let title_offset_from_center = 150;

    let font_surface = fonts
        .title
        .render(&menu.title)
        .blended(DEFAULT_TEXT_COLOR)
        .unwrap();
    let texture = font_surface.as_texture(&texture_creator).unwrap();
    let mut title_rect = font_surface.rect();
    let title_origin = Point::new(
        ((canvas_width as f32 / 2.) - (title_rect.width() as f32 / 2.)) as i32,
        ((canvas_height as f32 / 2.) - (title_rect.height() as f32 / 2.)) as i32
            - title_offset_from_center,
    );
    title_rect.reposition(title_origin);
    canvas.copy(&texture, None, title_rect).unwrap();

    // Draw menu
    let mut text_offset = 50;
    menu.items.iter().enumerate().for_each(|(index, item)| {
        let color = if index == menu.item_selected as usize {
            selected_text_color
        } else {
            DEFAULT_TEXT_COLOR
        };

        let label = match item {
            menu::Item::Play { label }
            | menu::Item::Quit { label }
            | menu::Item::Mode { label }
            | menu::Item::Resume { label }
            | menu::Item::EndGame { label }
            | menu::Item::Music { label }
            | menu::Item::MusicVolume { label } => label,
        };

        let font_surface = fonts.settings.render(&label).blended(color).unwrap();
        let texture = font_surface.as_texture(&texture_creator).unwrap();
        let mut menu_rect = font_surface.rect();
        let menu_origin = Point::new(
            ((canvas_width as f32 / 2.) - (menu_rect.width() as f32 / 2.)) as i32,
            title_rect.y + title_rect.height() as i32 + text_offset,
        );

        menu_rect.reposition(menu_origin);
        canvas.copy(&texture, None, menu_rect).unwrap();
        text_offset += 50;
    });
}

fn game_color_to_sdl_color(color: block::Color) -> Color {
    Color::RGB(color.r, color.g, color.b)
}
