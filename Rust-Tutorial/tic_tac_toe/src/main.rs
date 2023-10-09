struct Game {
    game_board: Vec<i32>,
    exes: &i32,
    ohs: &i32,
    player_turn: &mut i32
 }
 
 impl Game {
    fn initialize_game(
        game_board: Vec<i32>,
        exes: &i32,
        ohs: &i32, 
        player_turn: &mut i32
    ) -> Game {
       Game {
          game_board,
          exes,
          ohs,
          player_turn
       }
    }
 
    fn run_program(&mut self) {
        // keep track of game end
        let game_over = false;

        // Get user's next move
        let user_input = 0;

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
    fn update_vector(&self,user_input: i32) {

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
    let mut player_turn = 0;
    
    let mut game = Game::initialize_game(
        game_board,
        &exes,
        &ohs,
        &mut player_turn
    );
 
    game.run_program();
 }
 