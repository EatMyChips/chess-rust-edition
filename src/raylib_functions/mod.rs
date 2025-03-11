use crate::{GameData, BLOCK_SIZE};
use raylib::prelude::*;

pub fn draw(mut rl: RaylibHandle, thread: &RaylibThread, game_data: &GameData) -> RaylibHandle {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::RAYWHITE);
    d = draw_board(d, game_data);

    drop(d);
    rl
}

fn draw_board<'a>(mut d: RaylibDrawHandle<'a>, game_data: &'a GameData) -> RaylibDrawHandle<'a>{
    for i in 0..8 {
        for j in 0..8 {
            let is_white = (i % 2) == (j % 2);
            let color: Color = Color::new(255 * is_white as u8, 255 * is_white as u8, 255 * is_white as u8, 255);
            d.draw_rectangle(
                i * BLOCK_SIZE,
                j * BLOCK_SIZE,
                crate::BLOCK_SIZE,
                crate::BLOCK_SIZE,
                color,
            )
        }
    }
    d
}
