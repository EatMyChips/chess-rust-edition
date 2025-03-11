use crate::position::Position;
use crate::{GameData, GameState}; // Position struct

// Grouped data for sending to the pieces

// Crete the various types of pieces
#[derive(Debug, Copy, Clone)]

pub enum Piece {
    King { white: bool },
    Queen { white: bool },
    Bishop { white: bool },
    Knight { white: bool },
    Rook { white: bool },
    Pawn { white: bool },
}

// Universal piece functionality
impl Piece {
    // Get bool data from each piece
    fn is_white(&self) -> bool {
        match self {
            Piece::King { white }
            | Piece::Queen { white }
            | Piece::Bishop { white }
            | Piece::Knight { white }
            | Piece::Rook { white }
            | Piece::Pawn { white } => *white,
        }
    }

    // Valid move check
    pub fn valid_move(piece_position: Position, move_position: Position, data: &GameData) -> bool {
        // Null check
        let selected_piece: Piece = match data.board[piece_position.x][piece_position.y] {
            Some(piece) => piece,
            None => return false,
        };

        // Get bool data
        let white = Piece::is_white(&selected_piece);

        // Ensure they're moving the correct coloured piece
        match &data.game_state {
            GameState::WhiteTurn => {
                if !white {
                    return false;
                }
            }
            GameState::BlackTurn => {
                if white {
                    return false;
                }
            }
            _ => return false,
        }

        // Checks if the move is valid for each piece type
        match selected_piece {
            Piece::King { white: _ } => true,
            Piece::Queen { white: _ } => true,
            Piece::Bishop { white: _ } => true,
            Piece::Knight { white: _ } => true,
            Piece::Rook { white: _ } => true,
            Piece::Pawn { white: _ } => true,
        }
    }
}
