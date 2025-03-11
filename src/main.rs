use crate::position::Position;
use pieces::Piece;
use raylib::prelude::*;

mod initialization;
mod pieces;
mod position;
mod raylib_functions;

const BOARD_SIZE: i32 = 504;
const BLOCK_SIZE: i32 = BOARD_SIZE / 8;

struct GameData {
    board: [[Option<Piece>; 8]; 8],
    game_state: GameState,
}

enum GameState {
    WhiteTurn,
    BlackTurn,
    InvalidMove,
    WhiteWin,
    BlackWin,
    Draw,
}

fn start_game() -> GameData {
    // Initialize board object
    let mut board: [[Option<Piece>; 8]; 8] = [[const { None }; 8]; 8];

    // Add pieces
    board = initialization::create_pieces(board);

    // Returns game board
    GameData {
        board: board,
        game_state: GameState::WhiteTurn,
    }
}

fn player_turn(mut data: GameData, piece_position: Position, move_position: Position) -> GameData {
    // Valid move check
    if !Piece::valid_move(piece_position, move_position, &data) {
        data.game_state = GameState::InvalidMove;
        return data;
    }

    // TODO::If (opponent in check?)
    // Is game over?

    // TODO::Else
    // Return data (make sure game_condition is either WhiteTurn or BlackTurn)
    data
}

fn make_move(mut data: GameData, piece_position: Position, move_position: Position) {}

fn run_raylib_game(){
    // Initialise Chess
    let game_data = start_game();

    // Initialise raylib window
    let (mut rl, thread) = raylib::init()
        .size(BOARD_SIZE, BOARD_SIZE)
        .title("Chess")
        .build();

    // While window is open
    while !rl.window_should_close() {

        // Draw board
        rl = raylib_functions::draw(rl, &thread, &game_data)
    }
}

fn main() {
    run_raylib_game();
}
