use crate::game::utils::{GameData, GameState, Position};

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
    fn white(&self) -> bool {
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
        let selected_piece: &Piece = match data.get_piece_at(piece_position) {
            Some(piece) => piece,
            None => return false,
        };

        let white = selected_piece.white();

        // Ensure they're moving the correct coloured piece
        match &data.get_state() {
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

        //TODO
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

    pub fn get_image_path(game_data: &GameData, position: Position) -> String{
        let selected_piece: &Piece = match game_data.get_piece_at(position) {
            Some(piece) => piece,
            None => return ""
        };
        match selected_piece{
            Piece::King { white: white } => "",
            Piece::Queen { white: white } => "",
            Piece::Bishop { white: white } => "",
            Piece::Knight { white: white } => "",
            Piece::Rook { white: white } => "",
            Piece::Pawn { white: white } => "",
        }
    }
}
