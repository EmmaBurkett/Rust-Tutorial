use std::io;
use std::io::Write;

struct Game<'a> {
    game_board: Vec<char>,
    exes: &'a char,
    ohs: &'a char,
    turn_number: &'a mut i32
 }
 
 impl<'a> Game<'a> {
    fn initialize_game(
        game_board: Vec<String>,
        exes: &'a String,
        ohs: &'a String, 
        turn_number: &'a mut i32
    ) -> Game<'a> {
       Game {
          vec![" "; 3*3];,
          exes,
          ohs,
          turn_number
       }
    }
 
    fn play(&mut self) {
        // keep track of game end
        let mut game_over = false;

        // Get user's next move
        let mut user_input = 0;

        // Display positions 1-9
        self.display_rules();

        // Contine game until someone's won
        while !(game_over) {

            // Display board with xo
            self.display_board();

            // Get playing location
            user_input = self.get_input();

            // Update vector with correct sign
            self.update_board(user_input);

            // Check for winner
            game_over = self.has_won();
        }

        // Display who won
        self.display_winner();
    }

    fn display_rules(&self) {
        /*
          0 | 1 | 2 
         -----------
          3 | 4 | 5 
         -----------
          6 | 7 | 8 
        */

        // Iterate 0 through 8 for(int x = 0; x < 9; x++)
        for x in 0..9 {
            // Create row dividers
            if x == 2 || x == 5{
                print!(" {} ", x);
                println!("\n-----------");
            }
            // Don't create row divider on last digit
            else if x == 8{
                println!(" {} ", x);
            }
            // Create column divider
            else {
                print!(" {} |", x);
            }
        }
        // Add space and print the final line
        println!("");
    }

    fn display_board(&self) {
        /*
          O | X | O 
         -----------
          X | O | X 
         -----------
          O | X | O 
        */
        // Count Iterations
        let mut index = 0;

        // Iterate through game_board
        for x in &self.game_board {
            // Create row dividers
            if index == 2 || index == 5{
                print!(" {} ", x);
                println!("\n-----------");
            }
            // Don't create row divider on last digit
            else if index == 8{
                println!(" {} ", x);
            }
            // Create column divider
            else {
               
                print!(" {} |", x);
            }
            index += 1;
        }
        // Add newline and print the final line
        println!("");
    }

    fn get_input(&self) -> i32 {
        // Declare mutatable String to take user input
        let mut user_input = String::new();

        // Make it pretty. >(
        print!("> ");
        io::stdout().flush().unwrap();

        // Take user input do some error handling
        io::stdin().read_line(&mut user_input).expect("Failed to read Line");

        // Get rid of excess spaces or new lines and turn into String value
        user_input = user_input.trim().to_string();

        // Turn into a number
        let number_input: i32 = user_input.parse().unwrap();

        return number_input;
    }

    fn update_board(&mut self, user_input: i32) {
        // Find out whose turn it is and deliver an x or an o
        if *self.turn_number % 2 == 0{
            
            // Vectors take their slicing in terms of usize
            self.game_board[user_input as usize] = (*self.ohs).clone();
        }
        else {

            /* To use self.exes's value we have to dereference it.
             * String does not have a clone function so to avoid an error
             * use .clone()
             * Parenthases are needed or else .clone with clone &self.exes and 
             * not the value of self.exes
            */
            self.game_board[user_input as usize] = (*self.exes).clone();
        }

        // Dereference self.turn_number to add 1 to it
        // Dereference the storage place as well; you're not replacing the address
        *self.turn_number = *self.turn_number + 1;
    }

    fn has_won(&self) ->bool {
        /*
          0 | 1 | 2 
         -----------
          3 | 4 | 5 
         -----------
          6 | 7 | 8 
        */
                // Three across top
        if     (self.game_board[0 as usize] == self.game_board[1 as usize] 
            &&  self.game_board[2 as usize] == self.game_board[1 as usize]
            &&  self.game_board[0] != " ")
                // Three across middle 
            ||  (self.game_board[3 as usize] == self.game_board[4 as usize] 
            &&   self.game_board[5 as usize] == self.game_board[4 as usize]
            &&   self.game_board[3] != " ")
                // Three across bottom
            ||  (self.game_board[6 as usize] == self.game_board[7 as usize] 
            &&   self.game_board[8 as usize] == self.game_board[7 as usize]
            &&   self.game_board[6] != " ")
                // Three down left
            ||  (self.game_board[0 as usize] == self.game_board[3 as usize] 
            &&   self.game_board[6 as usize] == self.game_board[3 as usize]
            &&   self.game_board[0] != " ")
                // Three down middle
            ||  (self.game_board[1 as usize] == self.game_board[4 as usize] 
            &&   self.game_board[7 as usize] == self.game_board[4 as usize]
            &&   self.game_board[1] != " ")
                // Three down right
            ||  (self.game_board[2 as usize] == self.game_board[5 as usize] 
            &&   self.game_board[8 as usize] == self.game_board[5 as usize]
            &&   self.game_board[6] != " ")
                // Three diagonal left to right
            ||  (self.game_board[0 as usize] == self.game_board[4 as usize] 
            &&   self.game_board[8 as usize] == self.game_board[4 as usize]
            &&   self.game_board[0] != " ")
                // Three diagonal right to left
            ||  (self.game_board[2 as usize] == self.game_board[4 as usize] 
            &&   self.game_board[6 as usize] == self.game_board[4 as usize]
            &&   self.game_board[2] != " ")
         {
            return true;
        }
        return false;
    }
    fn display_winner(&self) {
        println!("Player: {} won in {} moves", *self.turn_number % 2 - 1, self.turn_number)
    }
 }
 
 fn main() {
    //Define ALL your variables here
    let game_board = vec![" "; 3*3];
    let player_one = 
    let exes = "X";
    let ohs = "O";
    let mut turn_number: i32; 
    
    let mut game = Game::initialize_game(
        game_board,
        &exes,
        &ohs,
        &mut turn_number
    );
 
    game.play();
 }
 // Fix has_won
 // Make is so no one can cheat 
 
 /* LifeTime Parameters 
 struct Game<'a> {
    game_board: Vec<i32>,
    exes: &'a i32,
    ohs: &'a i32,
    turn_number: &'a mut i32
 }
 
 impl<'a> Game<'a> {
    fn initialize_game(
        game_board: Vec<i32>,
        exes: &'a i32,
        ohs: &'a i32, 
        turn_number: &'a mut i32
    ) -> Game<'a> {
       Game {
          game_board,
          exes,
          ohs,
          turn_number
       }
    }
 
    fn run_program(&mut self) {
        // keep track of game end
        let mut game_over = false;

        // Get user's next move
        let mut user_input = 0;

        // Display positions 1-9
        self.display_rules();

        // Contine game until someone's won
        while !(game_over) {

            // Display board with xo
            self.display_board();

            // Get playing location
            user_input = self.get_input();

            // Update vector with correct sign
            self.update_vector(user_input);

            // Check for cat game or winner
            game_over = self.check_winner();
        }

        // Display who won
        self.display_winner();
    }

    fn display_rules(&self) {
        
    }

    fn display_board(&self) {

    }
    fn get_input(&self) -> i32 {
        return 1;
    }
    fn update_vector(&self, user_input: i32) {

    }
    fn check_winner(&self) ->bool {
        return false;
    }
    fn display_winner(&self) {

    }
 }
 
 fn main() {
    //Define ALL your variables here
    let game_board = vec![0; 9];
    let exes = 1;
    let ohs = 0;
    let mut turn_number = 0;
    
    let mut game = Game::initialize_game(
        game_board,
        &exes,
        &ohs,
        &mut turn_number
    );
 
    game.run_program();
 }
 */