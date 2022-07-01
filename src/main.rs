use std::io;

/**
 * Will hold key functions for Tic Tac Toe game
 */
#[derive(Clone, Copy)]
struct Game {
    board: [[i8;3]; 3],
}
impl Game {
    /**
     * Constructor for Game struct
     */
    fn new() -> Self {
        // initialize 2D array to zero 
        let b: [[i8;3]; 3] = [[0;3];3];
        let game = Game{ board: b };
        game
    }

    /**
     * Pretty board output
     */
    fn print_board(&self) {
        for x in 0..3 {
            if x != 0 {
                print!("|");
            }
            println!("\n-------------");
            for y in 0..3 {
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
        println!("\n-------------");
    }

    /**
     * Checks to see if a given move is valid and if so makes it
     */
    fn player_move(self, player: i8, position: i8, board: &mut [[i8;3]; 3]) -> bool{
        match position {
            1 => if board[0][0] != 0 {return false;} else {board[0][0] = player; return true;},
            2 => if board[0][1] != 0 {return false;} else {board[0][1] = player; return true;},
            3 => if board[0][2] != 0 {return false;} else {board[0][2] = player; return true;},
            4 => if board[1][0] != 0 {return false;} else {board[1][0] = player; return true;},
            5 => if board[1][1] != 0 {return false;} else {board[1][1] = player; return true;},
            6 => if board[1][2] != 0 {return false;} else {board[1][2] = player; return true;},
            7 => if board[2][0] != 0 {return false;} else {board[2][0] = player; return true;},
            8 => if board[2][1] != 0 {return false;} else {board[2][1] = player; return true;},
            9 => if board[2][2] != 0 {return false;} else {board[2][2] = player; return true;},
            _ => false,
        }
    }

    fn remove_move(self,position: i8,board: &mut [[i8;3]; 3]) {
        match position {
            1 => board[0][0] = 0,
            2 => board[0][1] = 0,
            3 => board[0][2] = 0,
            4 => board[1][0] = 0,
            5 => board[1][1] = 0,
            6 => board[1][2] = 0,
            7 => board[2][0] = 0,
            8 => board[2][1] = 0,
            9 => board[2][2] = 0,
            _ => panic!("Error in remove move function"),
        }
    }

    /**
     * Function checks if win condition has been met for
     * given player
     */
    fn is_game_over(&self, player: i8, board: [[i8;3]; 3]) -> bool {
        let n = 3;
        let mut in_a_row: i8 = 0;

        // Diagnol check
        for x in 0..n {
            if board[x][x] == player {
                in_a_row+=1;
            }
        }

        if in_a_row == 3 { return true; }
        in_a_row = 0;

        // Second diagnol check
        let mut reverse = n;
        for x in 0..n {
            reverse-=1; 
            if board[x][reverse] == player {
                in_a_row+=1;
            }    
        }

        if in_a_row == 3 { return true; }
        in_a_row = 0;

        // Rows
        for x in 0..n {
            for y in 0..n {
                if board[x][y] == player {
                    in_a_row+=1;
                }
            }
            if in_a_row == 3 { return true; }
            in_a_row = 0;
        }

        // Columns
        for x in 0..n {
            for y in 0..n {
                if board[y][x] == player {
                    in_a_row+=1;
                }
            }
            if in_a_row == 3 { return true; }
            in_a_row = 0;
        }

        false
    }

    fn is_draw(&self, board: [[i8;3]; 3]) -> bool {
        let mut filled_slots = 0;
        for x in 0..3 {
            for y in 0..3 {
                if board[x][y] != 0 {
                    filled_slots+=1;
                }
            }
        }

        if filled_slots == 9 {
            return true;
        }
        false
    }

    /**
     * The evaluate function will determine how good a move is based on
     * how many possible ways the given player can win in the current
     * board state
     */
    fn evaluate(&self, board: [[i8;3]; 3], player: i8) -> i32 {
        if self.is_game_over(player, board) {
            return i32::max_value();
        }
        if self.is_draw(board) {
            return 0;
        }
        let mut ways_to_win = 0;
        let mut ctr = 0;
        let n = 3;

                // Diagnol check
                for x in 0..n {
                    if board[x][x] == player || board[x][x] == 0 {
                        ctr+=1;
                    }
                }
        
                if ctr == 3 { ways_to_win+=1; }
                ctr = 0;
        
                // Second diagnol check
                let mut reverse = n;
                for x in 0..n {
                    reverse-=1; 
                    if board[x][reverse] == player || board[x][reverse] == 0 {
                        ctr+=1;
                    }    
                }
        
                if ctr == 3 { ways_to_win+=1; }
                ctr = 0;
        
                // Rows
                for x in 0..n {
                    for y in 0..n {
                        if board[x][y] == player || board[x][y] == 0 {
                            ctr+=1;
                        }
                    }
                    if ctr == 3 { ways_to_win+=1; }
                    ctr = 0;
                }
        
                // Columns
                for x in 0..n {
                    for y in 0..n {
                        if board[y][x] == player || board[y][x] == 0 {
                            ctr+=1;
                        }
                    }
                    if ctr == 3 { ways_to_win+=1; }
                    ctr = 0;
                }

                ways_to_win
    }

    /**
     * Minimax algorithm so the ai can make intelligent moves
     */
    fn minimax(&self, ai: i8, human: i8,  depth: i8, mut board: [[i8;3]; 3], maximize: bool) -> i32{
        if depth == 0 || self.is_draw(board) || self.is_game_over(ai, board) || self.is_game_over(human,board) {
            let ai_eval: i32 = self.evaluate(board, ai);
            let human_eval: i32 = self.evaluate(board, human);
            return ai_eval - human_eval;
        }

        if maximize {
            let mut max_eval: i32 = i32::min_value();
            for position in 1..10 {
                
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
            for position in 1..10 {
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
     * Function makes a random move for AI based on 
     * open slots randomly
     */
    fn ai_move(&mut self, ai: i8, human: i8) {
        let mut best = i32::min_value();
        let mut ai_position = -1;
        let depth = 4;

        let mut board = self.board.clone();

        for position in 1..10 {
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

fn main() {
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
        game.print_board();

        if current_player == human {
            println!("Make your move 1-9:");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            while !game.player_move(current_player, input.trim().parse().unwrap(),&mut game.board) {
                println!("Invalid move!\nMake your move 1-9:");
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

    println!("Press any key to quit");
    io::stdin().read_line(&mut input).expect("Enter");
}
