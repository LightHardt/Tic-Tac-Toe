use std::io;
use rand::seq::SliceRandom;

/**
 * Will hold key functions for Tic Tac Toe game
 */
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
    fn player_move(&mut self, player: i8, position: i8) -> bool{
        match position {
            1 => if self.board[0][0] != 0 {return false;} else {self.board[0][0] = player; return true;},
            2 => if self.board[0][1] != 0 {return false;} else {self.board[0][1] = player; return true;},
            3 => if self.board[0][2] != 0 {return false;} else {self.board[0][2] = player; return true;},
            4 => if self.board[1][0] != 0 {return false;} else {self.board[1][0] = player; return true;},
            5 => if self.board[1][1] != 0 {return false;} else {self.board[1][1] = player; return true;},
            6 => if self.board[1][2] != 0 {return false;} else {self.board[1][2] = player; return true;},
            7 => if self.board[2][0] != 0 {return false;} else {self.board[2][0] = player; return true;},
            8 => if self.board[2][1] != 0 {return false;} else {self.board[2][1] = player; return true;},
            9 => if self.board[2][2] != 0 {return false;} else {self.board[2][2] = player; return true;},
            _ => false,
        }
    }

    /**
     * Function checks if win condition has been met for
     * given player
     */
    fn is_game_over(&self, player: i8) -> bool {
        let n = 3;
        let mut in_a_row: i8 = 0;

        // Diagnol check
        for x in 0..n {
            if self.board[x][x] == player {
                in_a_row+=1;
            }
        }

        if in_a_row == 3 { return true; }
        in_a_row = 0;

        // Second diagnol check
        let mut reverse = n;
        for x in 0..n {
            reverse-=1; 
            if self.board[x][reverse] == player {
                in_a_row+=1;
            }    
        }

        if in_a_row == 3 { return true; }
        in_a_row = 0;

        // Rows
        for x in 0..n {
            for y in 0..n {
                if self.board[x][y] == player {
                    in_a_row+=1;
                }
            }
            if in_a_row == 3 { return true; }
            in_a_row = 0;
        }

        // Columns
        for x in 0..n {
            for y in 0..n {
                if self.board[y][x] == player {
                    in_a_row+=1;
                }
            }
            if in_a_row == 3 { return true; }
            in_a_row = 0;
        }

        false
    }

    /**
     * Function makes a random move for AI based on 
     * open slots randomly
     */
    fn ai_move(&mut self, player: i8) {
        let mut ai_choices: Vec<i8> = vec![];

        for x in 0..3 {
            for y in 0..3 {
                if self.board[x][y] == 0{
                    match (x,y) {
                        (0,0) => ai_choices.push(1),
                        (0,1) => ai_choices.push(2),
                        (0,2) => ai_choices.push(3),
                        (1,0) => ai_choices.push(4),
                        (1,1) => ai_choices.push(5),
                        (1,2) => ai_choices.push(6),
                        (2,0) => ai_choices.push(7),
                        (2,1) => ai_choices.push(8),
                        (2,2) => ai_choices.push(9),
                        (_,_) => panic!("Error"),
                    }
                }
            }
        }
        // Choose random move from vector and take it for ai player
        self.player_move(player, *ai_choices.choose(&mut rand::thread_rng()).expect("AI choice"));
        
    }
}

fn main() {
    let mut game = Game::new();
    let mut game_over = false;
    let mut input = String::new();
    let mut current_player: i8 = 1;

    while !game_over {
        game.print_board();

        if current_player == 1 {
            println!("Make your move 1-9:");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            while !game.player_move(current_player, input.trim().parse().unwrap()) {
                println!("Invalid move!\nMake your move 1-9:");
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read line");
            }
        }
        else {
            game.ai_move(current_player)
        }

        game_over = game.is_game_over(current_player);
        if game_over {
            if current_player == 1 {
                game.print_board();
                println!("Player X has won!");
            }
            else {
                game.print_board();
                println!("Player O has won!");
            }
        }
        if current_player == 1 { current_player = 2; }
        else { current_player = 1; }
        input.clear();
    }
}
