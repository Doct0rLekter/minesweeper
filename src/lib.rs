// Set clippy to pedantic
#![warn(clippy::pedantic)]

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
            let input = self.input.clone();
            input
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
        draw();

        loop {
            let game_over = state.get_game_over();

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

    fn reset(state: &mut GameState) {
        state.set_input("".to_string());
        state.set_game_over(false);
    }

    fn process_input() {
        // TODO
    }

    fn update() {
        // TODO
    }

    fn draw() {
        // TODO
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
        state.set_input("See? I can set input".to_string());

        assert_eq!("See? I can set input".to_string(), state.input);
    }

    #[test]
    fn sets_game_over() {
        let mut state = game_loop::GameState::new();
        state.set_game_over(true);

        assert_eq!(true, state.game_over)
    }
}
