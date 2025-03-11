use crate::pieces::Piece;

pub fn create_pieces(mut board: [[Option<Piece>; 8]; 8]) -> [[Option<Piece>; 8]; 8] {
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
