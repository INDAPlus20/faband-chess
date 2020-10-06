# faband-chess

I have not implemented en passent, castling, and promotion is very iffy with how it works and there are several problems that I will fix within a day if someone decides to use my chess engine. (Please message me via discord so that I can help you make it work)

# Use at your own discretion

### Enumerables
| **Enumerable** | **Values** | **Vague description** |
|----------------|------------|-----------------------|
| 'GameState'    | 'InProgress' 'Check' 'GameOver' | The states the game can have, where 'GameOver' is a version of check mate. |
| 'PieceType'    | 'Pawn' 'Knight' 'Bishop' 'Rook' 'Queen' 'King' | The different pieces, I think. |
| 'Colors'       | 'White' 'Black' | The two colors. |

### Structure 'Game'
*Similar to rust-task-3 instructions*

| **Function** | **Description** |
|--------------|-----------------|
| 'pub fn new() -> Game' | Creates a new game with pieces in place. |
| 'pub fn make_move(reference: &mut Self, ref_board: [Option<Piece>; 64], _from: String, _to: String) -> [Option<Piece>; 64]' | Takes a board and string references from a square, to a square. | 
| 'pub fn get_game_state(&self) -> GameState' | Returns the game state. |
| 'pub fn get_possible_moves(reference: &mut Self, _position: String) -> Option<Vec<String>>' | Returns a vector of possible moves and I am not sure if it works properly :P |

Pleases note that inputs are taken as two different strings with the square from and to, ex: "e4" "e5".
