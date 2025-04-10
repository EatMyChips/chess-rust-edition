use raylib::core::texture::Texture2D;
use crate::game::pieces::Piece;
use std::collections::HashMap;

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct GameData {
    board: [[Option<Piece>; 8]; 8],
    game_state: GameState,
    pub textures: HashMap<String, Texture2D>,
}
impl GameData {
    pub fn new() -> GameData {
        let board = create_board();
        let game_state = GameState::WhiteTurn;
        let textures: HashMap<String, Texture2D> = HashMap::new();

        Self { board, game_state, textures}
    }

    pub fn get_piece_at(&self, position: &Position) -> &Option<Piece> {
        &self.board[position.x][position.y]
    }

    pub fn set_state(&mut self, state: GameState) {
        self.game_state = state;
    }

    pub fn get_state(&self) -> &GameState {
        &self.game_state
    }
}

pub enum GameState {
    WhiteTurn,
    BlackTurn,
    InvalidMove,
    WhiteWin,
    BlackWin,
    Draw,
}

fn create_board() -> [[Option<Piece>; 8]; 8] {
    let mut board = [[None; 8]; 8];
    // Create kings
    board[0][4] = Some(Piece::King { white: false });
    board[7][4] = Some(Piece::King { white: true });

    // Create Queens
    board[0][3] = Some(Piece::Queen { white: false });
    board[7][3] = Some(Piece::Queen { white: true });

    // Create Bishops
    board[0][2] = Some(Piece::Bishop { white: false });
    board[7][2] = Some(Piece::Bishop { white: true });
    board[0][5] = Some(Piece::Bishop { white: false });
    board[7][5] = Some(Piece::Bishop { white: true });

    // Create Knights
    board[0][1] = Some(Piece::Knight { white: false });
    board[7][1] = Some(Piece::Knight { white: true });
    board[0][6] = Some(Piece::Knight { white: false });
    board[7][6] = Some(Piece::Knight { white: true });

    // Create Rook
    board[0][0] = Some(Piece::Rook { white: false });
    board[7][0] = Some(Piece::Rook { white: true });
    board[0][7] = Some(Piece::Rook { white: false });
    board[7][7] = Some(Piece::Rook { white: true });

    for i in 0..8 {
        board[1][i] = Some(Piece::Pawn { white: false });
        board[6][i] = Some(Piece::Pawn { white: true });
    }

    board
}
