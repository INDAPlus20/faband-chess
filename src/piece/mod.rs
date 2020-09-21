mod colors;
mod piece_types;
pub use colors::Colors;
pub use piece_types::PieceTypes;

#[derive(Copy, Clone, Debug, PartialEq)]

///Determines how a piece is structured
pub struct Piece {
    color: Colors,
    rank: PieceTypes,
    //possible_moves:
    //fix a vector for possible moves
}

impl Piece {
    ///Constructs a new piece of given color and type.
    pub fn new(rank: PieceTypes, color: Colors) -> Piece {
        Piece {
            color: color,
            rank: rank,
        }
    }
    //fix promotion for pawn
}
