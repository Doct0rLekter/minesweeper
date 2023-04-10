// Set clippy to pedantic
#![warn(clippy::pedantic)]

// Start blocking out a framework
// The below framework is loosely based on learnings from many sources on the 'game loop' design pattern
// The primary source referenced during the building of this framework is: https://www.gameprogrammingpatterns.com/game-loop.html
// This, of course, has to be translated into the world of rust
pub mod game_loop {
    use super::input_handler;

    // Provide structure to game data
    pub struct GameState {
        pub input: String,
        pub game_over: bool,
    }

    impl GameState {
        pub fn new() -> GameState {
            GameState {
                input: String::from(""),
                game_over: false,
            }
        }

        pub fn get_input(&self) -> String {
            self.input.clone()
        }

        pub fn get_game_over(&self) -> bool {
            self.game_over
        }

        pub fn set_input(&mut self, input: String) {
            self.input = input;
        }

        pub fn set_game_over(&mut self, game_over: bool) {
            self.game_over = game_over;
        }
    }

    pub fn play() {
        let mut state = GameState::new();
        // Reset the game state after a game over
        reset(&mut state);

        // Draw the initial game state
        draw(&mut state);

        loop {
            let game_over = state.get_game_over();

            if game_over {
                break;
            }

            // Process console input
            process_input(&mut state);

            // Update the game state
            update(&mut state);

            // Redraw game state after each update
            draw(&mut state);
        }
    }

    fn reset(state: &mut GameState) {
        state.set_input(String::from(""));
        state.set_game_over(false);
    }

    fn process_input(state: &mut GameState) {
        let input = input_handler::read_input("Enter a message to be echoed: ");
        state.set_input(input);
    }

    pub fn update(state: &mut GameState) {
        if state.get_input() == String::from(".exit") {
            state.set_game_over(true);
        };
    }

    fn draw(state: &mut GameState) {
        if state.get_input() != String::from(".exit") {
            println!("Input text and I will repeat it back to you!\n    (Type '.exit' to quit the 'game')");
            println!("{}", state.get_input());
        }
    }
}

// Create a new module to handle input to the program
pub mod input_handler {

    use std::io::{self, Write};

    pub fn read_input(prompt: &str) -> String {
        let mut input = String::new();

        loop {
            print!("{prompt}");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }

            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }

            if input.is_empty() {
                continue;
            }

            break;
        }
        input.to_lowercase()
    }
}
#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn sets_input() {
        let mut state = game_loop::GameState::new();
        state.set_input(String::from("See? I can set input"));

        assert_eq!(String::from("See? I can set input"), state.input);
    }

    #[test]
    fn sets_game_over() {
        let mut state = game_loop::GameState::new();
        state.set_game_over(true);

        assert_eq!(true, state.game_over)
    }

    #[test]
    fn updates_state() {
        let mut state = game_loop::GameState::new();

        state.set_input(String::from(".exit"));

        game_loop::update(&mut state);

        assert_eq!(true, state.get_game_over());
    }
}
