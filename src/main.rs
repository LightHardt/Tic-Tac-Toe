use std::io;

/**
 * Will be the size of board SIZE x SIZE
 */
const SIZE: usize = 4;
/**
 * Number needed in a row to win
 */
const WIN: i8 = 4;
/**
 * The difficulty of AI
 * A high number will effect performance depending on board size
 */
const DEPTH: i8 = 6;

/**
 * Will hold key functions for Tic Tac Toe game
 */
#[derive(Clone, Copy)]
struct Game {
    board: [[i8;SIZE]; SIZE],
}
impl Game {
    /**
     * Constructor for Game struct
     */
    fn new() -> Self {
        // initialize 2D array to zero 
        let b: [[i8;SIZE]; SIZE] = [[0;SIZE];SIZE];
        let game = Game{ board: b };
        game
    }

    /**
     * Pretty board output
     */
    fn print_board(&self) {
        for x in 0..SIZE {
            if x != 0 {
                print!("|");
            }
            println!("\n-----------------");
            for y in 0..SIZE {
                print!("|");
                if self.board[x][y] == 0 {
                    print!("   ");
                }
                else if self.board[x][y] == 1 {
                    print!(" X ");
                }
                else {
                    print!(" O ");
                }
            }
        }
        print!("|");
        println!("\n-----------------");
    }

    /**
     * Checks to see if a given move is valid and if so makes it
     */
    fn player_move(self, player: i8, position: usize, board: &mut [[i8;SIZE]; SIZE]) -> bool{
        match position {
            1 => if board[0][0] != 0 {return false;} else {board[0][0] = player; return true;},
            2 => if board[0][1] != 0 {return false;} else {board[0][1] = player; return true;},
            3 => if board[0][2] != 0 {return false;} else {board[0][2] = player; return true;},
            4 => if board[0][3] != 0 {return false;} else {board[0][3] = player; return true;},
            5 => if board[1][0] != 0 {return false;} else {board[1][0] = player; return true;},
            6 => if board[1][1] != 0 {return false;} else {board[1][1] = player; return true;},
            7 => if board[1][2] != 0 {return false;} else {board[1][2] = player; return true;},
            8 => if board[1][3] != 0 {return false;} else {board[1][3] = player; return true;},
            9 => if board[2][0] != 0 {return false;} else {board[2][0] = player; return true;},
            10 => if board[2][1] != 0 {return false;} else {board[2][1] = player; return true;},
            11 => if board[2][2] != 0 {return false;} else {board[2][2] = player; return true;},
            12 => if board[2][3] != 0 {return false;} else {board[2][3] = player; return true;},
            13 => if board[3][0] != 0 {return false;} else {board[3][0] = player; return true;},
            14 => if board[3][1] != 0 {return false;} else {board[3][1] = player; return true;},
            15 => if board[3][2] != 0 {return false;} else {board[3][2] = player; return true;},
            16 => if board[3][3] != 0 {return false;} else {board[3][3] = player; return true;},
            _ => false,
        }
    }

    fn remove_move(self,position: usize,board: &mut [[i8;SIZE]; SIZE]) {
        match position {
            1 => board[0][0] = 0,
            2 => board[0][1] = 0,
            3 => board[0][2] = 0,
            4 => board[0][3] = 0,
            5 => board[1][0] = 0,
            6 => board[1][1] = 0,
            7 => board[1][2] = 0,
            8 => board[1][3] = 0,
            9 => board[2][0] = 0,
            10 => board[2][1] = 0,
            11 => board[2][2] = 0,
            12 => board[2][3] = 0,
            13 => board[3][0] = 0,
            14 => board[3][1] = 0,
            15 => board[3][2] = 0,
            16 => board[3][3] = 0,
            _ => panic!("Error in remove move function"),
        }
    }

    /**
     * Function checks if win condition has been met for
     * given player
     */
    fn is_game_over(&self, player: i8, board: [[i8;SIZE]; SIZE]) -> bool {
        let mut in_a_row: i8 = 0;

        // Diagnol check
        for x in 0..SIZE {
            if board[x][x] == player {
                in_a_row+=1;
                if in_a_row == WIN { return true; }
            } else {
                in_a_row = 0;
            }
        }

        in_a_row = 0;

        // Second diagnol check
        let mut reverse = SIZE;
        for x in 0..SIZE {
            reverse-=1; 
            if board[x][reverse] == player {
                in_a_row+=1;
                if in_a_row == WIN { return true; }
            } else {
                in_a_row = 0;
            }
        }

        in_a_row = 0;

        // Rows
        for x in 0..SIZE {
            for y in 0..SIZE {
                if board[x][y] == player {
                    in_a_row+=1;
                    if in_a_row == WIN { return true; }
                } else {
                    in_a_row = 0;
                }
            }
            in_a_row = 0;
        }

        // Columns
        for x in 0..SIZE {
            for y in 0..SIZE {
                if board[y][x] == player {
                    in_a_row+=1;
                    if in_a_row == WIN { return true; }
                } else {
                    in_a_row = 0;
                }
            }
            in_a_row = 0;
        }

        false
    }

    fn is_draw(&self, board: [[i8;SIZE]; SIZE]) -> bool {
        let mut filled_slots = 0;
        for x in 0..SIZE {
            for y in 0..SIZE {
                if board[x][y] != 0 {
                    filled_slots+=1;
                }
            }
        }

        if filled_slots == SIZE*SIZE {
            return true;
        }
        false
    }

    /**
     * The evaluate function will determine how good a move is based on
     * how many possible ways the given player can win in the current
     * board state
     */
    fn evaluate(&self, board: [[i8;SIZE]; SIZE], player: i8) -> i32 {
        if self.is_game_over(player, board) {
            return i32::max_value();
        }
        if self.is_draw(board) {
            return 0;
        }
        let mut ways_to_win = 0;
        let mut ctr = 0;

                // Diagnol check
                for x in 0..SIZE {
                    if board[x][x] == player || board[x][x] == 0 {
                        ctr+=1;

                        if ctr == WIN { 
                            ways_to_win+=1;
                            ctr = 0;  
                        }
                    } else {
                        ctr = 0;
                    }
                }
        
                // Second diagnol check
                let mut reverse = SIZE;
                for x in 0..SIZE {
                    reverse-=1; 
                    if board[x][reverse] == player || board[x][reverse] == 0 {
                        ctr+=1;

                        if ctr == WIN { 
                            ways_to_win+=1;
                            ctr = 0;  
                        }
                    } else {
                        ctr = 0;
                    }
                }
        
                // Rows
                for x in 0..SIZE {
                    for y in 0..SIZE {
                        if board[x][y] == player || board[x][y] == 0 {
                            ctr+=1;

                            if ctr == WIN { 
                                ways_to_win+=1;
                                ctr = 0;  
                            }
                        } else {
                            ctr = 0;
                        }
                    }
                }
        
                // Columns
                for x in 0..SIZE {
                    for y in 0..SIZE {
                        if board[y][x] == player || board[y][x] == 0 {
                            ctr+=1;

                            if ctr == WIN { 
                                ways_to_win+=1;
                                ctr = 0;  
                            }
                        } else {
                            ctr = 0;
                        }
                    }
                }

                ways_to_win
    }

    /**
     * Minimax algorithm so the ai can make intelligent moves
     */
    fn minimax(&self, ai: i8, human: i8,  depth: i8, mut board: [[i8;SIZE]; SIZE], maximize: bool) -> i32{
        if depth == 0 || self.is_draw(board) || self.is_game_over(ai, board) || self.is_game_over(human,board) {
            let ai_eval: i32 = self.evaluate(board, ai);
            let human_eval: i32 = self.evaluate(board, human);
            return ai_eval - human_eval;
        }

        if maximize {
            let mut max_eval: i32 = i32::min_value();
            for position in 1..SIZE*SIZE+1 {
                
                if self.player_move(ai, position,&mut board) {
                    let eval = self.minimax(ai, human, depth - 1, board, false);
                    self.remove_move(position, &mut board);
                    max_eval = i32::max(max_eval, eval);
                }
                
            }
            return max_eval;
        }
        else {
            let mut min_eval = i32::max_value();
            for position in 1..SIZE*SIZE+1 {
                if self.player_move(human, position, &mut board) {
                    let eval = self.minimax(ai, human, depth - 1, board, true);
                    self.remove_move(position, &mut board);
                    min_eval = i32::min(min_eval, eval);
                }
            }
            return min_eval;
        }
    }

    /**
     * Function makes move for AI
     */
    fn ai_move(&mut self, ai: i8, human: i8) {
        let mut best = i32::min_value();
        let mut ai_position = 0;
        let depth = DEPTH;

        let mut board = self.board.clone();

        for position in 1..SIZE*SIZE+1 {
            if self.player_move(ai, position, &mut board) {
                let eval = self.minimax(ai, human, depth, board, false);
                self.remove_move(position, &mut board);
                if eval >= best {
                    best = eval;
                    ai_position = position;
                }
            }
        }

        self.player_move(ai, ai_position, &mut self.board);
    }
}

/**
 * This function will run the game
 */
fn tic_tac_toe() {
    let mut game = Game::new();
    let mut game_over = false;
    let mut input = String::new();
    let mut current_player: i8 = 1;
    let human: i8;
    let ai: i8;

    println!("1 for human first, 2 for ai:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().parse::<i8>().expect("Human or ai choice") == 1 {
        human = 1;
        ai = 2;
    }
    else {
        ai = 1;
        human = 2;
    }

    input.clear();

    while !game_over {
        if current_player == human {
            game.print_board();
            println!("Make your move 1-{}:",SIZE*SIZE);
            io::stdin().read_line(&mut input).expect("Failed to read line");
            while !game.player_move(current_player, input.trim().parse().unwrap(),&mut game.board) {
                println!("Invalid move!\nMake your move 1-{}:",SIZE*SIZE);
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read line");
            }
        }
        else {
            game.ai_move(ai,human)
        }

        if game.is_game_over(current_player,game.board) {
            game_over = true;
            if current_player == 1 {
                game.print_board();
                println!("Player X has won!");
            }
            else {
                game.print_board();
                println!("Player O has won!");
            }
        }
        
        if game.is_draw(game.board) & !game_over {
            game_over = true;
            game.print_board();
            println!("Draw!");
        }

        if current_player == ai { current_player = human; }
        else { current_player = ai; }
        input.clear();
    }
}

fn main() {
    let mut play_again = true;
    let mut input = String::new();

    while play_again {
        tic_tac_toe();
        println!("Press 1 to play again 2 to quit:");
        io::stdin().read_line(&mut input).expect("Enter");

        if input.trim().parse::<i8>().expect("Answer to if want play again") != 1 {
            play_again = false;
        }
        else {
            play_again = true;
        }
        input.clear();
    }
}
