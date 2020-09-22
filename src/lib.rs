use crate::piece::PieceType::*;
use std::fmt;
pub mod piece;
pub use piece::Colors;
pub use piece::Colors::*;
pub use piece::Piece;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

pub struct Game {
    ///Save board, active color, ...
    state: GameState,
    acting_color: Colors,
    board: [Option<Piece>; 64],
}

impl Game {
    ///Initialises a new board with pieces in start positions
    pub fn new() -> Game {
        Game {
            ///Initialises a new board and set the active color to white
            state: GameState::InProgress,
            acting_color: White,
            board: Game::new_board(),
        }
    }
    ///Fills the array that is the board
    fn new_board() -> [Option<Piece>; 64] {
        //Most unelegant but it works, the square begins at A1 and follows with A2, A3... etc
        let mut board: [Option<Piece>; 64] = [None; 64];
        board[0] = Some(Piece::new(Rook, White));
        board[1] = Some(Piece::new(Knight, White));
        board[2] = Some(Piece::new(Bishop, White));
        board[3] = Some(Piece::new(Queen, White));
        board[4] = Some(Piece::new(King, White));
        board[5] = Some(Piece::new(Bishop, White));
        board[6] = Some(Piece::new(Knight, White));
        board[7] = Some(Piece::new(Rook, White));
        //Generates the white pawns in front of the pieces
        for i in 8..16 {
            board[i] = Some(Piece::new(Pawn, White));
        }
        //Might need to generate empty squares?

        //Generates the black pawns on the other side of the board
        for j in 48..56 {
            board[j] = Some(Piece::new(Pawn, Black));
        }
        //Generates the black pieces
        board[56] = Some(Piece::new(Rook, Black));
        board[57] = Some(Piece::new(Knight, Black));
        board[58] = Some(Piece::new(Bishop, Black));
        board[59] = Some(Piece::new(Queen, Black));
        board[60] = Some(Piece::new(King, Black));
        board[61] = Some(Piece::new(Bishop, Black));
        board[62] = Some(Piece::new(Knight, Black));
        board[63] = Some(Piece::new(Rook, Black));
        //Returns this
        board
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
    pub fn get_possible_moves(&mut self, _position: String) -> Option<Vec<String>> {
        None
    }
    ///The standard template for a normal move
    fn normal_move(&mut self, index: usize, offset: Vec<(i8, i8)>) {
        //loops through the different directions in the given offset
        for direction in offset {
            //Gives all possible moves from the given piece, it goes up to 7 since it is not possible to move further on the board
            for scalar in 1..8 {
                //Since the board is 8x8 the possible moves loop around after 8 steps in the array
                let possible_move = (direction.1 + 8 * direction.0) * scalar;
                //Check so that the move is inside the board
                if (possible_move + index as i8) <= 63 && (possible_move + index as i8) >= 0 {
                    if self.board[index + possible_move as usize].is_some() {
                        if self.board[index + possible_move as usize]
                            .as_ref()
                            .unwrap()
                            .color
                            != self.acting_color
                        {
                            //Add move
                            self.board[index].unwrap().add_move(possible_move as isize);
                        } else {
                            break;
                        }
                        break;
                    } else {
                        self.board[index].unwrap().add_move(possible_move as isize);
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
    //Checks if the king is in check
    //fn check_check{}
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
