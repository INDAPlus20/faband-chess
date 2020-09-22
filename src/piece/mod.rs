pub mod colors;
pub mod piece_types;
pub use colors::Colors;
pub use piece_types::PieceType;

#[derive(Copy, Clone, Debug, PartialEq)]

///Determines how a piece is structured
pub struct Piece {
    pub color: Colors,
    pub piece_type: PieceType,
    //The possible moves are stored in a vector and are just added onto the indexing later
    pub possible_moves: Vec<isize>,
}

impl Piece {
    ///Constructs a new piece of given color and type.
    pub fn new(rank: PieceType, color: Colors) -> Piece {
        Piece {
            color: color,
            piece_type: rank,
            possible_moves: Vec::new(),
        }
    }
    pub fn add_move(&mut self, moves: isize) {
        self.possible_moves.push(moves);
    }
    //fix promotion for pawn
}
