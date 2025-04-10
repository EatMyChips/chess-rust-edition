use pieces::Piece;
use utils::{GameData, GameState, Position};

pub mod pieces;
pub mod utils;

pub fn player_turn(
    mut data: GameData,
    piece_position: Position,
    move_position: Position,
) -> GameData {
    match data.get_piece_at(&piece_position) {
        Some(piece) if piece.valid_move(move_position, &data) => {}
        _ => {
            data.set_state(GameState::InvalidMove);
            return data;
        }
    };

    // TODO::If (opponent in check?)
    // Is game over?

    // TODO::Else
    // Return data (make sure game_condition is either WhiteTurn or BlackTurn)
    data
}

//fn make_move(mut data: GameData, piece_position: Position, move_position: Position) {}
