use crate::piece::Colors;
use crate::piece::Colors::*;
use crate::piece::PieceType::*;
use crate::Piece;

///Defines how the board is constructed
pub struct Board {
    ///A board piece has a piece (or not) and a unique location in the array.
    board: [Option<Piece>; 64]
    //movelog: (usize, string, string), maybe not have it stored in board
}

impl Board {
    
    ///Takes the current board and calculates all possible moves for each different piece
    pub fn calculate_moves(
        &self,
        state: GameState,
        acting_color: Colors,
    ) -> Vec<BoardSquare> {
        let mut new_board_with_moves: Vec<BoardSquare> = Vec::new();
        //Fix an if statement for check so that we dont do unecessary calculations
        for x in 0..64 {
            if let Some(piece) = self.board[x].piece {
                //Deconstructs the location tuple
                let (file, rank) = current_board[x].location;
                match piece.piece_type {
                    Pawn => {
                        //Code or something :P
                        //Ill do this last, holy hell pawns are annoying
                    }
                    Knight => {
                        //How tf does a knight move (I know but damn the code)
                        for x in 0..2 {
                            for y in 0..2 {
                                //if current_board.find(| &(x,y)| x,y == current_board[x].location)
                            }
                        }
                    }
                    //Defines the movement for a bishop
                    Bishop => {
                        //The directions that the bishop can move
                        let offset: Vec<(i8, i8)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
                        for direction in offset {
                            //Gives all possible moves for the max amount of steps
                            for scalar in 1..8 {
                                let a = (
                                    file + scalar * direction.0 as usize,
                                    rank + scalar * direction.1 as usize,
                                );
                                let obstructing_piece = current_board[]
                                    .iter()
                                    .find(|&x| x.location == a)
                                    .unwrap()
                                    .piece;
                                if obstructing_piece == Some(piece) {
                                    //Code
                                    if obstructing_piece.unwrap().color != acting_color {
                                        //Add move
                                    }
                                    break;
                                }
                                //Add move, or not?
                            }
                        }
                        //Diagonal movement, no need to think of color
                    }
                    Rook => {
                        //Crushing movement
                    }
                    Queen => {
                        //God tier movement
                    }
                    King => {
                        //Shitty movement
                    }
                }
            }
        }
        return new_board_with_moves;
    }
    
}
