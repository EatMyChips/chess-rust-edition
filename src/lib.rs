use std::collections::HashMap;
use std::fs;
use raylib::ffi::Texture2D;

pub mod game;
mod raylib_functions;

pub fn run_raylib() {
    // Initialise raylib window
    let (mut rl, thread) = raylib::init()
        .size(raylib_functions::BOARD_SIZE, raylib_functions::BOARD_SIZE)
        .title("Chess")
        .build();

    // create game data
    let mut game_data = game::utils::GameData::new();

    let assets_path = "assets";

    for entry in fs::read_dir(assets_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let tex = rl.load_texture(&thread, path.to_str().unwrap()).unwrap();
            game_data.textures.insert(file_name.to_string(), tex);
        }
    }

    // While window is open
    while !rl.window_should_close() {
        // Draw board
        rl = raylib_functions::draw(rl, &thread, &game_data)
    }
}
