pub mod colors;
pub mod piece_types;
pub use colors::Colors;
pub use piece_types::PieceType;

#[derive(Copy, Clone, Debug, PartialEq)]

///Determines how a piece is structured
pub struct Piece {
    pub color: Colors,
    pub piece_type: PieceType,
}

impl Piece {
    ///Constructs a new piece of given color and type.
    pub fn new(rank: PieceType, color: Colors) -> Piece {
        Piece {
            color: color,
            piece_type: rank,
        }
    }
    //fix promotion for pawn
}
