#[derive(Copy, Clone, Debug, PartialEq)]
///All types pieces that exist on the board (listed in order of value in my opinion)
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
