// Start blocking out a framework

// The below framework is loosely based on learnings from many sources on the 'game loop' design pattern
// The primary source referenced during the building of this framework is: https://www.gameprogrammingpatterns.com/game-loop.html
// This, of course, has to be translated into the world of rust
mod game_loop {
    
    // Provide structure to game data
    pub struct GameState {
        pub input: String,
        pub game_over: bool,
    }

    impl GameState {
        pub fn new() -> GameState {
            
            let state = GameState {
                input: "".to_string(),
                game_over: false,
            };
            state
        }

        pub fn get_input(&self) -> String {
            self.input
        }

        pub fn get_game_over(&self) -> bool {
            self.game_over            
        }
    }


    pub fn play() {
        let mut state = GameState::new();    
        // Reset the game state after a game over
        reset();
        
        // Draw the initial game state
        draw();
        
        loop {
            if game_over {
                break;
            }
            
            // Process console input
            process_input();
            
            // Update the game state
                update();
                
                // Redraw game state after each update
                draw();
            }
        }
        
        fn reset() {
            TODO
        }
        
        fn process_input() {
            TODO
        }
        
        fn update() {
            TODO
        }
        
        fn draw() {
            TODO
        }
    }




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(4, (2 + 2))
    } 

    #[test]   
    fn gets_input() {
        let state = game_loop::GameState::new();

        assert_eq!(state.input, state.get_input());  
        
    }

    #[test]
    fn gets_game_over() {
        let state = game_loop::GameState::new();

        assert_eq!(state.game_over, state.get_game_over());
    }
}
