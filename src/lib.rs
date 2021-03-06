use crate::piece::PieceType::*;
use crate::GameState::*;
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
    possible_moves: Vec<(usize, i8)>, //Vector for moves that can be made by various pieces
}

impl Game {
    ///Initialises a new board with pieces in start positions
    pub fn new() -> Game {
        Game {
            ///Initialises a new board and set the active color to white
            state: GameState::InProgress,
            acting_color: White,
            board: Game::new_board(),
            possible_moves: {
                let bad_board: [Option<Piece>; 64] = Game::new_board();
                Game::calculate_possible_moves(&bad_board, White) //This is probably the worst thing in this code oh god
            },
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
    pub fn make_move(
        reference: &mut Self,
        ref_board: [Option<Piece>; 64],
        _from: String,
        _to: String,
    ) -> [Option<Piece>; 64] {
        //GO AND FIX THE POSSIBLE MOVES FOR PAWNS
        //CHECK SO THAT THE INPU MOVES ARE LEGAL AKA IN THE POSSIBLE MOVES VECTOR
        let from_index: usize = Game::string_to_index(_from);
        let mut board: [Option<Piece>; 64] = ref_board;
        let to_index = Game::string_to_index(_to.clone());
        //Checks if it is a promotion move
        if board[from_index].is_some()
            && board[from_index].unwrap().piece_type == Pawn
            && reference.acting_color == White
            && (to_index < 64 && to_index >= 56)
        {
            //Expects the _to to be a piece reference, not a square one
            board[to_index] =
                Game::set_promotion(board, reference.acting_color, from_index + 8, _to);
            board[from_index] = None;
        } else if board[from_index].is_some()
            && board[from_index].unwrap().piece_type == Pawn
            && reference.acting_color == Black
            && to_index < 8
        {
            //Expects the _to to be a piece reference, not a square one
            board[to_index] =
                Game::set_promotion(board, reference.acting_color, from_index - 8, _to);
            board[from_index] = None;
        } else {
            //Standard movement
            board[to_index] = Some(Piece::new(
                board[from_index].unwrap().piece_type,
                board[from_index].unwrap().color,
            ));
            board[from_index] = None;
        }
        //New acting color
        if reference.acting_color == White {
            reference.acting_color = Black
        } else {
            reference.acting_color = White
        }
        //Checks if there is a check in the current board
        if Game::check_check(reference.acting_color, board, board) {
            let v = Game::calculate_possible_moves(&board, reference.acting_color);
            if v.is_empty() {
                //Checkmate ~ish
                reference.state = GameOver;
            } else {
                reference.state = Check;
            }
        } else {
            reference.state = InProgress;
        }
        return board;
    }

    ///Set the piece type that a peasant becomes following a promotion
    fn set_promotion(
        board: [Option<Piece>; 64],
        acting_color: Colors,
        index: usize,
        _piece: String,
    ) -> Option<Piece> {
        let promoting_piece = match &_piece[0..1] {
            "Q" => Queen,
            "Kn" => Knight,
            "R" => Rook,
            "B" => Bishop,
            _default => Pawn,
        };
        return Some(Piece::new(promoting_piece, acting_color));
    }

    ///Get the current game state
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    ///If a piece is standing on a tile, return all possible moves for
    ///that piece. Include rules for check. Include special moves (en passent, etc)
    pub fn get_possible_moves(reference: &mut Self, _position: String) -> Option<Vec<String>> {
        let index_position = Game::string_to_index(_position);
        let mut returning_possible_moves: Vec<String> = vec![];
        if !reference.possible_moves.is_empty() {
            reference.possible_moves.iter().map(|x| {
                if x.0 == index_position {
                    returning_possible_moves.push(Game::index_to_string(x.1));
                }
            });
            return Some(returning_possible_moves);
        } else {
            None
        }
    }
    fn calculate_possible_moves(
        board: &[Option<Piece>; 64],
        acting_color: Colors,
    ) -> Vec<(usize, i8)> {
        let mut possible_moves_vector: Vec<(usize, i8)> = vec![];
        //Fix an if statement for check so that we dont do unecessary calculations for check
        //Loops through the board
        for index in 0..64 {
            //Check if the square has a piece on it self.board.iter().find(|&&found_piece| found_piece != None)
            if board[index].is_some() && board[index].unwrap().color == acting_color {
                //found_piece.piece_type
                match board[index].unwrap().piece_type {
                    Pawn => {
                        let mut offset: Vec<(i8, i8)> = vec![(1, 0)];
                        //Maybe return if promotion is not as I think
                        //These rules check the nearby conditions for the pawns where their moves depend
                        if board[index].unwrap().color == White {
                            if index >= 8 || index < 16 {
                                offset.push((2, 0));
                            }
                            if board[index + 9].is_some() && index % 8 != 7 {
                                offset.push((1, 1));
                            }
                            if board[index + 7].is_some() && index % 8 != 0 {
                                offset.push((1, -1));
                            }
                        } else {
                            if index >= 48 || index < 56 {
                                offset.push((2, 0));
                            }
                            if board[index - 9].is_some() && index % 8 != 7 {
                                offset.push((1, 1));
                            }
                            if board[index - 7].is_some() && index % 8 != 0 {
                                offset.push((1, -1));
                            }
                        }
                        let vector =
                            Game::normal_move(board.clone(), acting_color, index, offset, 6);
                        for x in vector {
                            possible_moves_vector.push((x.0, x.1));
                        }
                    }
                    Knight => {
                        //Moves with its own offset but with a scalar restriction
                        let offset: Vec<(i8, i8)> = vec![
                            (2, 1),
                            (2, -1),
                            (-2, 1),
                            (-2, -1),
                            (1, 2),
                            (1, -2),
                            (-1, 2),
                            (-1, -2),
                        ];
                        let vector =
                            Game::normal_move(board.clone(), acting_color, index, offset, 6);
                        for x in vector {
                            possible_moves_vector.push((x.0, x.1));
                        }
                    }
                    Bishop => {
                        //The directions that the bishop can move
                        let offset: Vec<(i8, i8)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
                        let vector =
                            Game::normal_move(board.clone(), acting_color, index, offset, 0);
                        for x in vector {
                            possible_moves_vector.push((x.0, x.1));
                        }
                    }
                    Rook => {
                        //Moving in straight lines
                        let offset: Vec<(i8, i8)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
                        let vector =
                            Game::normal_move(board.clone(), acting_color, index, offset, 0);
                        for x in vector {
                            possible_moves_vector.push((x.0, x.1));
                        }
                    }
                    Queen => {
                        //Moving in basically all directions
                        let offset: Vec<(i8, i8)> = vec![
                            (0, 1),
                            (0, -1),
                            (-1, 0),
                            (1, 0),
                            (1, 1),
                            (1, -1),
                            (-1, 1),
                            (-1, -1),
                        ];
                        let vector =
                            Game::normal_move(board.clone(), acting_color, index, offset, 0);
                        for x in vector {
                            possible_moves_vector.push((x.0, x.1));
                        }
                    }
                    King => {
                        //Moving like a queen, but with a limit to the scalar
                        let offset: Vec<(i8, i8)> = vec![
                            (0, 1),
                            (0, -1),
                            (-1, 0),
                            (-1, 0),
                            (1, 1),
                            (1, -1),
                            (-1, 1),
                            (-1, -1),
                        ];

                        let vector =
                            Game::normal_move(board.clone(), acting_color, index, offset, 6);
                        for x in vector {
                            possible_moves_vector.push((x.0, x.1));
                        }
                    }
                }
            }
        }
        //returns the vector of all possible moves
        possible_moves_vector
    }
    ///The standard template for a normal move
    fn normal_move(
        board: [Option<Piece>; 64],
        acting_color: Colors,
        index: usize,
        offset: Vec<(i8, i8)>,
        special_movement: i8,
    ) -> Vec<(usize, i8)> {
        let mut possible_moves_vector: Vec<(usize, i8)> = vec![];
        //Black pieces move inverse to white pieces, mainly implemented for pawn movement
        let inverse_movement: i8;
        if board[index].unwrap().color == White {
            inverse_movement = 1
        } else {
            inverse_movement = -1
        }
        //loops through the different directions in the given offset
        for direction in offset {
            //Gives all possible moves from the given piece, it goes up to 7 since it is not possible to move further on the board
            for scalar in 1..(8 - special_movement) {
                //Since the board is 8x8 the possible moves loop around after 8 steps in the array
                let possible_move = (direction.1 + 8 * direction.0) * scalar * inverse_movement;
                //Check so that the move is inside the board
                if (possible_move + index as i8) <= 63 && (possible_move + index as i8) >= 0 {
                    if (index < 8 && (index as i8 + direction.1 < 8))
                        || (index < 16
                            && index >= 8
                            && (index as i8 + direction.1 < 16 && index as i8 + direction.1 >= 8))
                        || (index < 24
                            && index >= 16
                            && (index as i8 + direction.1 < 24 && index as i8 + direction.1 >= 16))
                        || (index < 32
                            && index >= 24
                            && (index as i8 + direction.1 < 32 && index as i8 + direction.1 >= 24))
                        || (index < 40
                            && index >= 32
                            && (index as i8 + direction.1 < 40 && index as i8 + direction.1 >= 32))
                        || (index < 48
                            && index >= 40
                            && (index as i8 + direction.1 < 48 && index as i8 + direction.1 >= 40))
                        || (index < 56
                            && index >= 48
                            && (index as i8 + direction.1 < 56 && index as i8 + direction.1 >= 48))
                        || (index > 56 && index < 64 && (index as i8 + direction.1 < 64))
                    {
                        if board[(index as i8 + possible_move) as usize].is_some() {
                            if board[(index as i8 + possible_move) as usize].unwrap().color
                                != acting_color
                            {
                                //Creates a theoretical board where the found move takes place and controls that it does not
                                //cause a check
                                let mut test_move: [Option<Piece>; 64] = board;
                                test_move[(index as i8 + possible_move) as usize] =
                                    test_move[index];
                                test_move[index] = None;
                                if Game::check_check(acting_color, board, test_move) == false {
                                    //Add move
                                    possible_moves_vector.push((index, possible_move));
                                }
                            }
                            break;
                        } else {
                            let mut test_move: [Option<Piece>; 64] = board;
                            test_move[(index as i8 + possible_move) as usize] = test_move[index];
                            test_move[index] = None;
                            if Game::check_check(acting_color, board, test_move) == false {
                                //Add move
                                possible_moves_vector.push((index, possible_move));
                            }
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
        return possible_moves_vector;
    }
    ///Finds the location of the king of the playing color
    fn find_king_index(board: [Option<Piece>; 64], active_color: Colors) -> usize {
        let mut return_index: usize = 0;
        for index in 0..64 {
            if board[index].is_some()
                && board[index].unwrap().piece_type == King
                && board[index].unwrap().color == active_color
            {
                return_index = index;
                break;
            }
        }
        return return_index;
    }
    ///Checks if the king is in check or not, returns true if in check
    fn check_check(
        acting_color: Colors,
        board: [Option<Piece>; 64],
        test_board: [Option<Piece>; 64],
    ) -> bool {
        let inverse_movement: i8;
        if acting_color == White {
            inverse_movement = 1;
        } else {
            inverse_movement = -1;
        }
        let king_index = Game::find_king_index(test_board, acting_color);
        //reference.impossible_moves.push((index, possible_move));
        let offset_1: Vec<(i8, i8)> = vec![
            (0, 1),
            (0, -1),
            (-1, 0),
            (1, 0), //Looking for rooks or queens on the rook offset
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1), //Bishop offset looking for queens and bishops
        ];
        let offset_2: Vec<(i8, i8)> = vec![
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2), //The knight offset checking for possibilities of being checked by a knight
            (1, 1),
            (1, -1),
        ];
        for direction in offset_1 {
            for scalar in 1..8 {
                let possible_move = (direction.1 + 8 * direction.0) * scalar * inverse_movement;
                //Check so that the line is inside the board
                if (possible_move as isize + king_index as isize) <= 63
                    && (possible_move as isize + king_index as isize) >= 0
                {
                    //piece in path
                    if board[(king_index as i8 + possible_move) as usize].is_some() {
                        if board[(king_index as i8 + possible_move) as usize]
                            .unwrap()
                            .color
                            != acting_color
                        {
                            //Checks if we are looking for rooks or bishops essentially
                            if direction.1 == 0 || direction.0 == 0 {
                                if board[(king_index as i8 + possible_move) as usize]
                                    .unwrap()
                                    .piece_type
                                    == Rook
                                    || board[king_index + possible_move as usize]
                                        .unwrap()
                                        .piece_type
                                        == Queen
                                {
                                    return true;
                                }
                            }
                            //Checks if we are being attacked by bisops or queens
                            if board[(king_index as i8 + possible_move) as usize]
                                .unwrap()
                                .piece_type
                                == Bishop
                                || board[(king_index as i8 + possible_move) as usize]
                                    .unwrap()
                                    .piece_type
                                    == Queen
                            {
                                return true;
                            }
                        }
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        for direction in offset_2 {
            let possible_move = direction.1 + 8 * direction.0 * inverse_movement;
            //Check so that the line is inside the board
            if (possible_move + king_index as i8) <= 63 && (possible_move + king_index as i8) >= 0 {
                //piece in path
                if board[(king_index as i8 + possible_move) as usize].is_some() {
                    if board[(king_index as i8 + possible_move) as usize]
                        .unwrap()
                        .color
                        != acting_color
                    {
                        //Checks if we are attacked by a knight
                        if direction.1 == 2
                            || direction.1 == -2
                            || direction.0 == 2
                            || direction.0 == -2
                        {
                            if board[(king_index as i8 + possible_move) as usize]
                                .unwrap()
                                .piece_type
                                == Knight
                            {
                                return true;
                            }
                        }
                        //Checks if we are being attacked by a pawn
                        if board[(king_index as i8 + possible_move) as usize]
                            .unwrap()
                            .piece_type
                            == Pawn
                        {
                            return true;
                        }
                    }
                }
            }
        }
        //If no checks are discovered we return false
        false
    }
    fn index_to_string(index: i8) -> String {
        let mut square: String = "".to_string();
        square.push(match index % 8 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _default => 'a',
        });
        square.push(((index - index % 8) / 8) as u8 as char);
        square
    }
    fn string_to_index(_string_input: String) -> usize {
        //expects the input to be in the shape of ex: "e4", takes the letter and converts it to a column
        let column: usize = match &_string_input[0..1] {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _default => 0,
        };
        //This assumes that the input is correct...
        return (column + _string_input[1..2].parse::<usize>().unwrap() * 8 - 8) as usize;
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
    use super::Piece;
    use crate::piece::Colors::*;
    use crate::piece::PieceType::*;
    use crate::GameState::*;
    // check test framework
    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();
        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    #[test]
    fn board_is_working_to_reference_after_init() {
        let game = Game::new();
        assert_eq!(game.board[0].unwrap(), Piece::new(Rook, White));
    }

    #[test]
    fn movement_test() {
        let mut game = Game::new();
        let mut game_board: [Option<Piece>; 64] = game.board;
        game_board = Game::make_move(&mut game, game_board, "e2".to_string(), "e4".to_string());
        for x in 0..64 {
            if x % 8 == 0 {
                println!();
            }
            if game_board[x].is_some() {
                print!("{:?} \t", game_board[x].unwrap().piece_type)
            } else {
                print!("* \t");
            }
        }
        assert_eq!(game.get_game_state(), InProgress);
        //Checks that the color switched
        assert_eq!(game.acting_color, Black);
    }

    #[test]
    fn check_test_with_moves() {
        let mut game = Game::new();
        let mut game_board: [Option<Piece>; 64] = game.board;
        let possible_moves = &game.possible_moves;
        let mut n: usize = 0;
        for z in possible_moves {
            println!("{:?}", z);
            n += 1;
        }
        println!("{}", n);
        game_board = Game::make_move(&mut game, game_board, "e2".to_string(), "e4".to_string());
        assert_eq!(game.get_game_state(), InProgress);
        game_board = Game::make_move(&mut game, game_board, "d7".to_string(), "d5".to_string());
        assert_eq!(game.get_game_state(), InProgress);
        game_board = Game::make_move(&mut game, game_board, "f1".to_string(), "b5".to_string());
        assert_eq!(game.get_game_state(), InProgress);
        for x in 0..64 {
            if x % 8 == 0 {
                println!();
            }
            if game_board[x].is_some() {
                print!("{:?} \t", game_board[x].unwrap().piece_type)
            } else {
                print!("* \t");
            }
        }
        assert_eq!(game.get_game_state(), Check);
    }
}
