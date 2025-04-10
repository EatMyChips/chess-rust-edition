pub mod game;
mod raylib_functions;

pub fn run_raylib() {
    // create game data
    let game_data = game::utils::GameData::new();

    // Initialise raylib window
    let (mut rl, thread) = raylib::init()
        .size(raylib_functions::BOARD_SIZE, raylib_functions::BOARD_SIZE)
        .title("Chess")
        .build();

    // While window is open
    while !rl.window_should_close() {
        // Draw board
        rl = raylib_functions::draw(rl, &thread, &game_data)
    }
}
