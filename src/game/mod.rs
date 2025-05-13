use pieces::Piece;
use raylib::core::texture::Texture2D;
use std::collections::HashMap;
use utils::{Position, create_board, GameState};

pub mod pieces;
pub(crate) mod utils;

pub struct GameData {
    board: [[Option<Piece>; 8]; 8],
    game_state: GameState,
    pub textures: HashMap<String, Texture2D>,
}

impl GameData {
    pub fn new() -> GameData {
        let board = create_board();
        let game_state = GameState::WhiteTurn;
        let textures = HashMap::new();

        Self { board, game_state, textures}
    }

    pub fn player_turn(
        &mut self,
        piece_position: Position,
        move_position: Position,
    ) {
        match self.board[piece_position.x][piece_position.y] {
            Some(piece) if piece.valid_move(move_position, &self) => {}
            _ => {
                self.game_state = GameState::InvalidMove;
                return
            }
        };

        // TODO::If (opponent in check?)
        // Is game over?

        // TODO::Else
        // Return data (make sure game_condition is either WhiteTurn or BlackTurn)
    }

    pub fn get_piece_at(&self, position: &Position) -> &Option<Piece> {
        &self.board[position.x][position.y]
    }

    pub fn get_state(&self) -> &GameState {
        &self.game_state
    }
}
