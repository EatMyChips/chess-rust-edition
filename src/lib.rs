use std::collections::HashMap;
use crate::raylib_functions::{RaylibData};

pub mod game;
mod raylib_functions;

pub fn run_raylib() {
    // create game data
    let mut game = game::GameData::new();
    
    // Initialise raylib window
    let mut raylib_data = RaylibData::initialize();
    raylib_data.load_textures(&mut game);

    // While window is open
    while !raylib_data.rl.window_should_close() {
        // Draw board
        raylib_data.draw(&game)
    }
}
