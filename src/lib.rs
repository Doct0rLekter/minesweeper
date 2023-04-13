// Set clippy to pedantic
#![warn(clippy::pedantic)]

// Start blocking out a framework
// The below framework is loosely based on learnings from many sources on the 'game loop' design pattern
// The primary source referenced during the building of this framework is:
// https://www.gameprogrammingpatterns.com/game-loop.html
// This, of course, has to be translated into the world of rust

// Provide structure to game data
pub struct GameState {
    game_over: bool,
    board_width: u32,
    board_height: u32,
    tiles: Vec<Tile>,
    selected_tile: Option<usize>,
}

// Provide type checked names to capture the state of our tiles
#[derive(Debug, PartialEq)]
pub enum Tile {
    Hidden { has_mine: bool, flagged: bool },
    Revealed { has_mine: bool, hint: u32 },
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    #[must_use]
    pub fn new() -> GameState {
        GameState {
            game_over: false,
            board_height: 0,
            board_width: 0,
            tiles: Vec::new(),
            selected_tile: None,
        }
    }

    #[must_use]
    pub fn get_game_over(&self) -> bool {
        self.game_over
    }

    #[must_use]
    pub fn get_height(&self) -> u32 {
        self.board_height
    }

    #[must_use]
    pub fn get_width(&self) -> u32 {
        self.board_width
    }

    #[must_use]
    pub fn get_tile(&self, index: usize) -> &Tile {
        &self.tiles[index]
    }

    #[must_use]
    pub fn get_selected(&self) -> usize {
        self.selected_tile
            .expect("Should always have 'Some' value during normal play.")
    }

    pub fn set_game_over(&mut self, game_over: bool) {
        self.game_over = game_over;
    }

    pub fn set_height(&mut self, height: u32) {
        self.board_height = height;
    }

    pub fn set_width(&mut self, width: u32) {
        self.board_width = width;
    }

    pub fn set_tile(&mut self, index: usize, tile_state: Tile) {
        self.tiles[index] = tile_state;
    }

    pub fn set_selected(&mut self, index: u32) {
        let selected_tile = index as usize;
        self.selected_tile = Some(selected_tile);
    }
}

pub mod game_loop {
    use super::{input_handler, GameState, Tile};

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
        state.set_game_over(false);
    }

    fn process_input(state: &mut GameState) {
        loop {
            println!("Select a hidden tile\n");
            let mut row = input_handler::read_as_int("Enter row: ", 1, state.get_height());
            let mut column = input_handler::read_as_int("Enter column: ", 1, state.get_width());

            // Turn input into index
            row -= 1;
            column -= 1;

            state.set_selected((row * state.get_width()) + column);

            let index = state.get_selected();

            match state.get_tile(index) {
                Tile::Hidden {
                    has_mine: x,
                    flagged: false,
                } => {
                    let flag_tile = input_handler::read_as_bool("Flag this tile? Y/n: ");
                    if flag_tile {
                        state.set_tile(
                            index,
                            Tile::Hidden {
                                has_mine: *x,
                                flagged: true,
                            },
                        );
                    } else {
                        state.set_tile(
                            index,
                            Tile::Revealed {
                                has_mine: *x,
                                hint: if *x { 10 } else { 0 },
                            },
                        );
                    }
                    break;
                }
                Tile::Hidden {
                    has_mine: y,
                    flagged: true,
                } => {
                    let unflag_tile = input_handler::read_as_bool("Unflag this tile? Y/n: ");
                    if unflag_tile {
                        state.set_tile(
                            index,
                            Tile::Hidden {
                                has_mine: *y,
                                flagged: false,
                            },
                        );
                        break;
                    }
                    continue;
                }
                _ => {
                    println!("Selected tile must be hidden.");
                    continue;
                }
            }
        }
    }

    pub fn update(state: &mut GameState) {
        let index = state.get_selected();

        if let Tile::Revealed {
            has_mine: true,
            hint: _,
        } = state.get_tile(index)
        {
            state.set_game_over(true)
        }
    }

    fn draw(state: &mut GameState) {
        if state.get_game_over() {
            println!("Game over!")
        }
    }
}

// Create a new module to handle input to the program
pub mod input_handler {

    use std::io::{self, Write};

    #[must_use]
    #[allow(clippy::missing_panics_doc)] // This function is unlikely to panic under normal circumstances
    pub fn read_input(prompt: &str) -> String {
        let mut input = String::new();

        loop {
            print!("{prompt}");

            // # Panics
            //
            // This function will panic if flushing the stdout buffer fails.
            // However, this is unlikely to happen under normal circumstances.
            io::stdout().flush().unwrap();

            // # Panics
            //
            // This function will panic if reading from stdin fails.
            // This could happen if there's an issue with the input stream,
            // or if the process does not have access to the standard input.
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
        // Might want to make a configuration parameter for case sensitivity that defaults to false
        input.to_lowercase()
    }

    #[must_use]
    pub fn read_as_bool(prompt: &str) -> bool {
        let input = loop {
            let input = read_input(prompt);

            match input.trim().to_lowercase().as_str() {
                "yes" => break true,
                "no" => break false,
                _ => println!("Invalid input. Please enter either 'yes' or 'no'."),
            }
        };

        input
    }

    #[must_use]
    pub fn read_as_int(prompt: &str, min: u32, max: u32) -> u32 {
        let input = loop {
            let input = read_input(prompt);

            if let Ok(n) = input.trim().parse::<u32>() {
                if n >= min && n <= max {
                    break n;
                }

                println!("Number must be between {min} and {max} inclusive.");
                continue;
            }
            println!("Invalid input. Please enter an integer.");
        };
        input
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_game_over() {
        let state = GameState::new();

        assert_eq!(state.game_over, state.get_game_over());
    }

    #[test]
    fn gets_height() {
        let state = GameState::new();

        assert_eq!(state.board_height, state.get_height());
    }

    #[test]
    fn gets_width() {
        let state = GameState::new();

        assert_eq!(state.board_width, state.get_width());
    }

    #[test]
    fn gets_tile() {
        let mut state = GameState::new();

        state.tiles.push(Tile::Hidden {
            has_mine: false,
            flagged: false,
        });

        assert_eq!(&state.tiles[0], state.get_tile(0));
    }

    #[test]
    fn gets_selected() {
        let mut state = GameState::new();

        state.selected_tile = Some(1);

        assert_eq!(1, state.get_selected());
    }

    #[test]
    fn sets_game_over() {
        let mut state = GameState::new();
        state.set_game_over(true);

        assert_eq!(true, state.game_over)
    }

    #[test]
    fn sets_width() {
        let mut state = GameState::new();
        state.set_width(5);

        assert_eq!(5, state.board_width)
    }

    #[test]
    fn sets_height() {
        let mut state = GameState::new();
        state.set_height(5);

        assert_eq!(5, state.board_height)
    }

    #[test]
    fn sets_tile() {
        let mut state = GameState::new();
        state.tiles.push(Tile::Hidden {
            has_mine: (false),
            flagged: false,
        });
        state.set_tile(
            0,
            Tile::Revealed {
                has_mine: (false),
                hint: (0),
            },
        );

        assert_eq!(
            state.tiles[0],
            Tile::Revealed {
                has_mine: (false),
                hint: (0)
            }
        )
    }

    #[test]
    fn sets_selected() {
        let mut state = GameState::new();
        state.set_selected(0);

        assert_eq!(state.selected_tile.unwrap(), 0)
    }

    #[test]
    fn updates_state() {
        let mut state = GameState::new();

        game_loop::update(&mut state);

        assert_eq!(true, state.get_game_over());
    }
}
