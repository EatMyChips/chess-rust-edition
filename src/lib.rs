use std::collections::HashMap;
use std::fs;
use raylib::ffi::Texture2D;
use crate::raylib_functions::load_textures;

pub mod game;
mod raylib_functions;

pub fn run_raylib() {
    // Initialise raylib window
    let (mut rl, thread) = raylib::init()
        .size(raylib_functions::BOARD_SIZE, raylib_functions::BOARD_SIZE)
        .title("Chess")
        .build();

    // create game data
    let mut game_data = game::utils::GameData::new(HashMap::new());
    rl = load_textures(rl, &thread, &mut game_data);

    // While window is open
    while !rl.window_should_close() {
        // Draw board
        rl = raylib_functions::draw(rl, &thread, &game_data)
    }
}
