use crate::game::{utils::GameData, pieces::Piece, utils::Position};
use raylib::prelude::*;

pub const BOARD_SIZE: i32 = 504;
pub const BLOCK_SIZE: i32 = BOARD_SIZE / 8;

pub fn draw(mut rl: RaylibHandle, thread: &RaylibThread, game_data: &GameData) -> RaylibHandle {
    let mut d = rl.begin_drawing(thread);

    d = draw_board(d, game_data);

    drop(d);
    rl
}

fn draw_board(mut d: RaylibDrawHandle, game_data: &GameData) -> RaylibDrawHandle {
    d.clear_background(Color::RAYWHITE);
    for i in 0..8 {
        for j in 0..8 {
            d.draw_rectangle(
                i * BLOCK_SIZE,
                j * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                if (i % 2) == (j % 2) { Color::WHITE } else { Color::BLACK },
            );
            let path = Piece::get_image_path(game_data, Position{x: i as usize, y: j as usize});
        }
    }
    d
}
