use crate::game::GameData;
use crate::game::utils::{GameState, Position};

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
    // Valid move check
    pub fn valid_move(&self, move_position: Position, data: &GameData) -> bool {
        // Ensure they're moving the correct coloured piece
        let white = self.white();
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
        match self {
            Piece::King { white: _ } => true,
            Piece::Queen { white: _ } => true,
            Piece::Bishop { white: _ } => true,
            Piece::Knight { white: _ } => true,
            Piece::Rook { white: _ } => true,
            Piece::Pawn { white: _ } => true,
        }
    }

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


    // TODO: segment this into the raylib functions
    pub fn get_image_path(&self) -> &str {
        match self {
            Piece::King { white: white } => {
                if *white {
                    "king_w.png"
                } else {
                    "king_b.png"
                }
            }
            Piece::Queen { white: white } => {
                if *white {
                    "queen_w.png"
                } else {
                    "queen_b.png"
                }
            }
            Piece::Bishop { white: white } => {
                if *white {
                    "bishop_w.png"
                } else {
                    "bishop_b.png"
                }
            }
            Piece::Knight { white: white } => {
                if *white {
                    "knight_w.png"
                } else {
                    "knight_b.png"
                }
            }
            Piece::Rook { white: white } => {
                if *white {
                    "rook_w.png"
                } else {
                    "rook_b.png"
                }
            }
            Piece::Pawn { white: white } => {
                if *white {
                    "pawn_w.png"
                } else {
                    "pawn_b.png"
                }
            }
        }
    }
}
