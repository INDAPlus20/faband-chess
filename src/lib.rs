use crate::piece::Colors::*;
use crate::piece::PieceTypes::*;
use std::fmt;
mod piece;
pub use piece::Piece;

#[derive(Copy, Clone, Debug, PartialEq)]
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
    ///Decalres the array, or vector, that is the playing board and generates the pieces.
    pub fn new_board() -> Vec<BoardSquare> {
        let mut board_vector: Vec<BoardSquare> = Vec::new();
        for x in 1..9 {
            if x == 1 {
                //Most unelegant but it works
                board_vector.push(BoardSquare::new_square(1, 1, Some(Piece::new(Rook, White))));
                board_vector.push(BoardSquare::new_square(
                    1,
                    2,
                    Some(Piece::new(Knight, White)),
                ));
                board_vector.push(BoardSquare::new_square(
                    1,
                    2,
                    Some(Piece::new(Bishop, White)),
                ));
                board_vector.push(BoardSquare::new_square(
                    1,
                    2,
                    Some(Piece::new(Queen, White)),
                ));
                board_vector.push(BoardSquare::new_square(1, 2, Some(Piece::new(King, White))));
                board_vector.push(BoardSquare::new_square(
                    1,
                    2,
                    Some(Piece::new(Bishop, White)),
                ));
                board_vector.push(BoardSquare::new_square(
                    1,
                    2,
                    Some(Piece::new(Knight, White)),
                ));
                board_vector.push(BoardSquare::new_square(1, 2, Some(Piece::new(Rook, White))));
            } else if x == 2 {
                for z in 1..9 {
                    board_vector.push(BoardSquare::new_square(2, z, Some(Piece::new(Pawn, White))));
                }
            } else if x == 7 {
                for i in 1..9 {
                    board_vector.push(BoardSquare::new_square(7, i, Some(Piece::new(Pawn, Black))));
                }
            } else if x == 8 {
                board_vector.push(BoardSquare::new_square(1, 1, Some(Piece::new(Rook, Black))));
                board_vector.push(BoardSquare::new_square(
                    1,
                    2,
                    Some(Piece::new(Knight, Black)),
                ));
                board_vector.push(BoardSquare::new_square(
                    1,
                    2,
                    Some(Piece::new(Bishop, Black)),
                ));
                board_vector.push(BoardSquare::new_square(
                    1,
                    2,
                    Some(Piece::new(Queen, Black)),
                ));
                board_vector.push(BoardSquare::new_square(1, 2, Some(Piece::new(King, Black))));
                board_vector.push(BoardSquare::new_square(
                    1,
                    2,
                    Some(Piece::new(Bishop, Black)),
                ));
                board_vector.push(BoardSquare::new_square(
                    1,
                    2,
                    Some(Piece::new(Knight, Black)),
                ));
                board_vector.push(BoardSquare::new_square(1, 2, Some(Piece::new(Rook, Black))));
            } else {
                for y in 1..8 {
                    board_vector.push(BoardSquare::new_square(x, y, None));
                }
            }
        }
        return board_vector;
    }
}

pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

pub struct Game {
    ///Save board, active color, ...
    state: GameState,
    board: Vec<BoardSquare>,
}

impl Game {
    ///Initialises a new board with pieces in start positions
    pub fn new() -> Game {
        Game {
            ///Initialises a new board and set the active color to white
            state: GameState::InProgress,
            board: BoardSquare::new_board(),
        }
    }
    ///If the current game state is in progress and the move is legal
    ///then move a piece and return the resulting state of the game
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        None
    }

    ///Set the piece type that a peasant becomes following a promotion
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    ///Get the current game state
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    ///If a piece is standing on a tile, return all possible moves for
    ///that piece. Include rules for check. Include special moves (en passent, etc)
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
        None
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
