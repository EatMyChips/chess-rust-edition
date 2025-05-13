use std::fs;
use raylib::ffi::{GetMouseX, GetMouseY, MouseButton::MOUSE_BUTTON_LEFT};
use crate::game::{GameData, utils::Position};
use raylib::prelude::*;

pub const BOARD_SIZE: i32 = 504;
pub const BLOCK_SIZE: i32 = BOARD_SIZE / 8;

pub struct RaylibData {
    pub rl: RaylibHandle,
    thread: RaylibThread,
}

impl RaylibData {
    pub fn initialize() -> Self {
        let (mut rl, thread) = init()
            .size(BOARD_SIZE, BOARD_SIZE)
            .title("Chess")
            .build();
        Self { rl, thread }
    }

    pub fn load_textures(&mut self, game_data: &mut GameData) {
        let assets_path = "assets";

        for entry in fs::read_dir(assets_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let tex = self.rl.load_texture(&self.thread, path.to_str().unwrap()).unwrap();
                game_data.textures.insert(file_name.to_string(), tex);
            }
        }
    }

    pub fn draw(&mut self, game_data: &GameData) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d = draw_board(d, game_data);
        drop(d);
    }

    pub unsafe fn select_piece(&mut self, game_data: &mut GameData) {
        if(self.rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT)){
            let x = GetMouseX();
            let y = GetMouseY();
        }
    }
}

fn draw_board<'a>(mut d: RaylibDrawHandle<'a>, game_data: &'a GameData) -> RaylibDrawHandle<'a> {
    d.clear_background(Color::RAYWHITE);
    for i in 0..8 {
        for j in 0..8 {
            d.draw_rectangle(
                i * BLOCK_SIZE,
                j * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                if (i % 2) == (j % 2) {
                    Color::WHITE
                } else {
                    Color::GRAY
                },
            );
            d = draw_pieces(
                d,
                game_data,
                Position {
                    x: i as usize,
                    y: j as usize,
                },
            );
        }
    }
    d
}

fn draw_pieces<'a>(
    mut d: RaylibDrawHandle<'a>,
    game_data: &'a GameData,
    position: Position,
) -> RaylibDrawHandle<'a> {
    let path = match game_data.get_piece_at(&position) {
        Some(piece) => piece.get_image_path(),
        None => return d,
    };
    d.draw_texture(game_data.textures.get(path).unwrap(), position.x as i32 * BLOCK_SIZE, position.y as i32 * BLOCK_SIZE, Color::WHITE);
    d
}