use crate::piece::Colors::*;
use crate::piece::PieceTypes::*;
use crate::Piece;

///Defines how the board is constructed
pub struct BoardSquare {
    ///A board piece has a piece (or not) and a location telling us where it is located (duh)
    piece: Option<Piece>,
    location: (usize, usize),
    //movelog: (usize, string, string), maybe not have it stored in board
}

impl BoardSquare {
    ///Constructs an ordinary square on the board without a piece
    pub fn new_square(row: usize, rank: usize, piece: Option<Piece>) -> Self {
        Self {
            location: (row, rank),
            piece: piece,
        }
    }
    ///Decalres the array, or vector, that is the playing board and generates the pieces
    pub fn new_board() -> Vec<BoardSquare> {
        //Sorry for the format, but Rust formatting want it this way...
        let mut board_vector: Vec<BoardSquare> = Vec::new();
        //Most unelegant but it works, the square 0,0 will be corrseponding to A1, etc
        board_vector.push(BoardSquare::new_square(0, 0, Some(Piece::new(Rook, White))));
        board_vector.push(BoardSquare::new_square(
            0,
            1,
            Some(Piece::new(Knight, White)),
        ));
        board_vector.push(BoardSquare::new_square(
            0,
            2,
            Some(Piece::new(Bishop, White)),
        ));
        board_vector.push(BoardSquare::new_square(
            0,
            3,
            Some(Piece::new(Queen, White)),
        ));
        board_vector.push(BoardSquare::new_square(0, 4, Some(Piece::new(King, White))));
        board_vector.push(BoardSquare::new_square(
            0,
            5,
            Some(Piece::new(Bishop, White)),
        ));
        board_vector.push(BoardSquare::new_square(
            0,
            6,
            Some(Piece::new(Knight, White)),
        ));
        board_vector.push(BoardSquare::new_square(0, 7, Some(Piece::new(Rook, White))));
        //Generates the white pawns in front of the pieces
        for z in 0..8 {
            board_vector.push(BoardSquare::new_square(1, z, Some(Piece::new(Pawn, White))));
        }
        //Generates empty squares
        for x in 2..6 {
            for y in 0..8 {
                board_vector.push(BoardSquare::new_square(x, y, None));
            }
        }
        //Generates the black pawns on the other side of the board
        for i in 0..8 {
            board_vector.push(BoardSquare::new_square(6, i, Some(Piece::new(Pawn, Black))));
        }
        board_vector.push(BoardSquare::new_square(7, 0, Some(Piece::new(Rook, Black))));
        board_vector.push(BoardSquare::new_square(
            7,
            1,
            Some(Piece::new(Knight, Black)),
        ));
        board_vector.push(BoardSquare::new_square(
            7,
            2,
            Some(Piece::new(Bishop, Black)),
        ));
        board_vector.push(BoardSquare::new_square(
            7,
            3,
            Some(Piece::new(Queen, Black)),
        ));
        board_vector.push(BoardSquare::new_square(7, 4, Some(Piece::new(King, Black))));
        board_vector.push(BoardSquare::new_square(
            7,
            5,
            Some(Piece::new(Bishop, Black)),
        ));
        board_vector.push(BoardSquare::new_square(
            7,
            6,
            Some(Piece::new(Knight, Black)),
        ));
        board_vector.push(BoardSquare::new_square(7, 7, Some(Piece::new(Rook, Black))));
        return board_vector;
    }
    ///Takes the current board and calculates all possible moves for each different piece
    pub fn calculate_moves(current_board: Vec<BoardSquare>) {
        for x in 0..64 {
            if current_board[x].piece != None {}
        }
    }
}
