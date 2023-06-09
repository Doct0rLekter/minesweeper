// Set clippy to pedantic
#![warn(clippy::pedantic)]

// Start blocking out a framework
// The below framework is loosely based on learnings from many sources on the 'game loop' design pattern
// The primary source referenced during the building of this framework is:
// https://www.gameprogrammingpatterns.com/game-loop.html
// This, of course, has to be translated into the world of rust
//
// In an effort to improve my grasp on the concepts of game creation in Rust,
// I began reading the book, "Hands on Rust," which intends to teach Rust through the making of games.
// This was done using the 'bracket-lib' game engine crate.
// This product will remain with its current "from scratch" engine; however, there is likely
// to be a certain amount of influence from the more formal implementation of the same concepts.
// It should be noted that the games created in "Hands on Rust" are a Flappy Bird clone,
// and a roguelike dungeon crawler which I've yet to start on.

use input_handler::InputMode;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameMode {
    Config,
    Play,
    Quit,
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

// Provide structure to game data
pub struct GameState {
    game_mode: GameMode,
    game_over: bool,
    game_won: bool,
    board_width: u32,
    board_height: u32,
    starting_mines: u32,
    mine_count: u32,
    turn_count: u32,
    tiles: Vec<Tile>,
    selected_tile: Option<usize>,
    input_mode: InputMode,
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
            game_mode: GameMode::Play,
            game_over: false,
            game_won: false,
            board_height: 0,
            board_width: 0,
            starting_mines: 0,
            mine_count: 0,
            turn_count: 0,
            tiles: Vec::new(),
            selected_tile: None,
            input_mode: InputMode::Undo,
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

    #[must_use]
    pub fn get_input_mode(&self) -> InputMode {
        self.input_mode
    }

    #[must_use]
    pub fn get_won(&self) -> bool {
        self.game_won
    }

    #[must_use]
    pub fn get_mines(&self) -> u32 {
        self.starting_mines
    }

    #[must_use]
    pub fn get_game_mode(&self) -> GameMode {
        self.game_mode
    }

    #[must_use]
    pub fn get_mine_count(&self) -> u32 {
        self.mine_count
    }

    #[must_use]
    pub fn get_turn_count(&self) -> u32 {
        self.turn_count
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

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.input_mode = input_mode;
    }

    pub fn set_won(&mut self, game_won: bool) {
        self.game_won = game_won;
    }

    pub fn set_mines(&mut self, num_mines: u32) {
        self.starting_mines = num_mines;
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }

    pub fn set_mine_count(&mut self, mine_count: u32) {
        self.mine_count = mine_count;
    }

    pub fn board_setup(&mut self, width: u32, height: u32, num_mines: u32) {
        self.board_width = width;
        self.board_height = height;
        self.starting_mines = num_mines;
        self.mine_count = num_mines;
        self.turn_count = 0;
        self.game_won = false;
        self.game_over = false;
        self.clear_tiles();
    }

    pub fn add_tile(&mut self, tile_state: Tile) {
        self.tiles.push(tile_state);
    }

    pub fn represent_tile(&mut self, index: u32) -> String {
        let tile = index as usize;

        match self.tiles.get(tile) {
            Some(Tile::Hidden {
                has_mine: _,
                flagged: true,
            }) => String::from(" F "),
            Some(Tile::Revealed {
                has_mine: true,
                hint: _,
            }) => String::from(" X "),
            Some(Tile::Revealed {
                has_mine: false,
                hint: x,
            }) => format!(" {x} "),
            None => String::from(" ? "),
            _ => String::from(" - "),
        }
    }

    pub fn clear_tiles(&mut self) {
        self.tiles = Vec::new();
    }

    pub fn increment_turn_count(&mut self) {
        self.turn_count += 1;
    }

    pub fn increment_mine_count(&mut self) {
        self.mine_count += 1;
    }

    pub fn decrement_mine_count(&mut self) {
        self.mine_count -= 1;
    }
}

pub mod game_loop {

    use super::{input_handler, input_handler::InputMode, Difficulty, GameMode, GameState, Tile};
    use crossterm::{execute, terminal};
    use rand::Rng;
    use std::io::stdout;

    pub fn play() {
        loop {
            let mut state = GameState::new();
            let (mode, difficulty) = menu();

            if mode == GameMode::Quit {
                clear_screen();
                break;
            }

            // Reset the game state after a game over
            setup(&mut state, &difficulty);

            // Draw the initial game state
            draw(&mut state);

            loop {
                let game_over = state.get_game_over();
                let won = state.get_won();

                if game_over || won {
                    input_handler::enter_to_continue();
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
    }

    fn print_title() {
        let title_menu = r#"
  __  __ _____ _   _ ______  _______          ________ ______ _____  ______ _____  
 |  \/  |_   _| \ | |  ____|/ ____\ \        / /  ____|  ____|  __ \|  ____|  __ \ 
 | \  / | | | |  \| | |__  | (___  \ \  /\  / /| |__  | |__  | |__) | |__  | |__) |
 | |\/| | | | | . ` |  __|  \___ \  \ \/  \/ / |  __| |  __| |  ___/|  __| |  _  / 
 | |  | |_| |_| |\  | |____ ____) |  \  /\  /  | |____| |____| |    | |____| | \ \ 
 |_|  |_|_____|_| \_|______|_____/    \/  \/   |______|______|_|    |______|_|  \_\


   __  ___  __  _           
  / / | _ \ \ \| |__ _ _  _ 
 | |  |  _/  | | / _` | || |
 | |  |_|    | |_\__,_|\_, |
  \_\       /_/        |__/ 

   __   ___  __            __ _                   
  / /  / __| \ \ ___ _ _  / _(_)__ _ _  _ _ _ ___ 
 | |  | (__   | / _ \ ' \|  _| / _` | || | '_/ -_)
 | |   \___|  | \___/_||_|_| |_\__, |\_,_|_| \___|
  \_\        /_/               |___/              

   __   ___   __       _ _   
  / /  / _ \  \ \ _  _(_) |_ 
 | |  | (_) |  | | || | |  _|
 | |   \__\_\  | |\_,_|_|\__|
  \_\         /_/            
      "#;

        println!("{title_menu}");
    }

    fn config() -> Difficulty {
        input_handler::read_difficulty(
            "Enter preferred difficulty level [(e)asy | (m)edium | (h)ard] : ",
        )
    }

    fn menu() -> (GameMode, Difficulty) {
        clear_screen();
        print_title();
        let game_mode =
            input_handler::read_game_mode("Enter choice [(p)lay | (c)onfigure | (q)uit] : ");

        let difficulty;
        match game_mode {
            GameMode::Play | GameMode::Quit => (game_mode, Difficulty::Easy),
            GameMode::Config => {
                difficulty = config();
                (GameMode::Play, difficulty)
            }
        }
    }

    fn place_mines(state: &mut GameState) {
        let total_tiles = (state.get_width() * state.get_height()) as usize;
        let num_mines = state.get_mines() as usize;

        // Generate an array of all tile indices
        let mut indices: Vec<usize> = (0..total_tiles).collect();

        // Fisher-Yates shuffle algorithm
        // idea to use this algorithm came from the following stack overflow question:
        // https://stackoverflow.com/questions/28891084/minesweeper-mine-generation-algorithm
        let mut rng = rand::thread_rng();
        for i in (1..total_tiles).rev() {
            let j = rng.gen_range(0..=i);
            indices.swap(i, j);
        }

        // Place mines in the first `num_mines` positions of the shuffled indices
        for &mine_index in indices.iter().take(num_mines) {
            let tile = state.get_tile(mine_index);

            if let Tile::Hidden {
                has_mine: false,
                flagged,
            } = tile
            {
                state.set_tile(
                    mine_index,
                    Tile::Hidden {
                        has_mine: true,
                        flagged: *flagged,
                    },
                );
            }
        }
    }

    fn setup(state: &mut GameState, difficulty: &Difficulty) {
        match difficulty {
            Difficulty::Easy => state.board_setup(5, 5, 4),
            Difficulty::Medium => state.board_setup(8, 8, 14),
            Difficulty::Hard => state.board_setup(12, 12, 35),
        }

        let number_of_tiles = state.get_height() * state.get_width();

        for _tile in 0..number_of_tiles {
            state.add_tile(Tile::Hidden {
                has_mine: false,
                flagged: false,
            });
        }

        place_mines(state);
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn find_neighbors(state: &GameState, index: usize) -> Vec<usize> {
        let width = state.get_width() as isize;
        let height = state.get_height() as isize;

        let index_x = index as isize % width;
        let index_y = index as isize / width;

        let mut neighbors = Vec::new();

        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let neighbor_x = index_x + col_offset;
                let neighbor_y = index_y + row_offset;

                if neighbor_x >= 0 && neighbor_x < width && neighbor_y >= 0 && neighbor_y < height {
                    let neighbor_index = (neighbor_y * width + neighbor_x) as usize;
                    neighbors.push(neighbor_index);
                }
            }
        }

        neighbors
    }

    fn calculate_hint(state: &GameState, index: usize) -> u32 {
        let neighbors = find_neighbors(state, index);

        let mut count = 0;

        for &neighbor_index in &neighbors {
            if let Tile::Hidden {
                has_mine: true,
                flagged: _,
            } = state.get_tile(neighbor_index)
            {
                count += 1;
            }
        }

        count
    }

    fn process_input(state: &mut GameState) {
        loop {
            println!("Select a hidden tile\n");

            let (column, row) = input_handler::read_column_row(
                "Enter column and row: ",
                1,
                state.get_width(),
                state.get_height(),
            );

            let input_mode =
                input_handler::read_input_mode("(C)lear, (F)lag, or (U)ndo selection? ");

            if input_mode == InputMode::Undo {
                continue;
            }

            state.set_input_mode(input_mode);
            state.set_selected((row * state.get_width()) + column);

            let index = state.get_selected();

            let stored_hint = calculate_hint(state, index);

            let mine_count = state.get_mine_count();
            let max_mines = state.get_mines();

            match state.get_tile(index) {
                Tile::Hidden {
                    has_mine: x,
                    flagged: false,
                } => {
                    if input_mode == InputMode::Flag {
                        state.set_tile(
                            index,
                            Tile::Hidden {
                                has_mine: *x,
                                flagged: true,
                            },
                        );

                        if mine_count > 0 {
                            state.decrement_mine_count();
                        }
                    } else {
                        state.set_tile(
                            index,
                            Tile::Revealed {
                                has_mine: *x,
                                hint: if *x { 10 } else { stored_hint },
                            },
                        );
                    }
                    break;
                }
                Tile::Hidden {
                    has_mine: y,
                    flagged: true,
                } => {
                    if input_mode == InputMode::Flag {
                        state.set_tile(
                            index,
                            Tile::Hidden {
                                has_mine: *y,
                                flagged: false,
                            },
                        );
                        if mine_count < max_mines {
                            state.increment_mine_count();
                        }
                        break;
                    }
                    let clear_anyways =
                        input_handler::read_as_bool("Tile is flagged, clear anyways? (Y/n): ");
                    if clear_anyways {
                        state.set_tile(
                            index,
                            Tile::Revealed {
                                has_mine: *y,
                                hint: if *y { 10 } else { stored_hint },
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

    fn reveal_neighbors(state: &mut GameState, index: usize) {
        let neighbors = find_neighbors(state, index);

        for neighbor_index in neighbors {
            let tile = state.get_tile(neighbor_index);

            if let Tile::Hidden {
                has_mine: false,
                flagged: false,
            } = tile
            {
                let hint = calculate_hint(state, neighbor_index);

                state.set_tile(
                    neighbor_index,
                    Tile::Revealed {
                        has_mine: false,
                        hint,
                    },
                );

                if hint == 0 {
                    reveal_neighbors(state, neighbor_index);
                }
            }
        }
    }

    fn check_for_win(state: &mut GameState) {
        let mut winner = true;

        for tile in &state.tiles {
            match tile {
                Tile::Revealed {
                    has_mine: false,
                    hint: _,
                }
                | Tile::Hidden {
                    has_mine: true,
                    flagged: _,
                } => continue,
                _ => {
                    winner = false;
                    break;
                }
            }
        }
        if winner {
            state.set_won(winner);
        }
    }

    fn update(state: &mut GameState) {
        state.increment_turn_count();

        let index = state.get_selected();
        let stored_hint = calculate_hint(state, index);

        if let Tile::Revealed {
            has_mine: true,
            hint: _,
        } = state.get_tile(index)
        {
            state.set_game_over(true);

            state.tiles.iter_mut().for_each(|tile| {
                if let Tile::Hidden {
                    has_mine: true,
                    flagged: _,
                } = tile
                {
                    *tile = Tile::Revealed {
                        has_mine: true,
                        hint: 10,
                    };
                }
            });
        } else if state.get_input_mode() == InputMode::Clear && stored_hint == 0 {
            reveal_neighbors(state, index);
        }

        check_for_win(state);
    }

    fn clear_screen() {
        let mut stdout = stdout();

        execute!(stdout, terminal::Clear(terminal::ClearType::All))
            .expect("Failed to clear screen");
    }

    #[allow(clippy::cast_possible_truncation)] // Our column number will never go above u8.
                                               // May refactor to be u8 by default?
    fn column_to_letter(col: u32) -> char {
        ((col as u8) + b'A') as char
    }

    fn draw(state: &mut GameState) {
        clear_screen();

        let mine_count = state.get_mine_count();
        let turn_count = state.get_turn_count() + 1;

        println!("Turns: {turn_count}\nMines: {mine_count}\n");
        // Print the column numbers
        print!("     ");
        for col in 0..state.get_width() {
            print!("{:3}", column_to_letter(col));
        }
        println!();

        for row in 0..state.get_height() {
            print!("{:4}", row + 1); // Print the row number

            for col in 0..state.get_width() {
                let index = row * state.get_width() + col;
                let tile_representation = state.represent_tile(index);
                print!("{tile_representation:3}");
            }

            println!();
        }

        if state.get_game_over() {
            // Consider adding end of game stats
            println!("Game over!");
        }

        if state.get_won() {
            println!("Congratulations, you found all of the mines!");
        }
    }
}

// Create a new module to handle input to the program
pub mod input_handler {

    use super::{Difficulty, GameMode};
    use std::io::{self, Write};

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum InputMode {
        Clear,
        Flag,
        Undo,
    }

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
                "yes" | "y" | "Y" => break true,
                "no" | "n" | "N" => break false,
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

    #[must_use]
    pub fn read_column_row(prompt: &str, min: u32, width: u32, height: u32) -> (u32, u32) {
        let (column, row) = loop {
            let input = read_input(prompt);
            let mut chars = input.chars();

            let column_char = chars.next();
            let column_number = if let Some(c) = column_char {
                (c.to_ascii_lowercase() as u32) - ('a' as u32)
            } else {
                println!("Invalid input. Please enter a valid column and row.");
                continue;
            };

            let row_number = chars.as_str().parse::<u32>();
            match row_number {
                Ok(n) => {
                    if column_number >= min - 1 && column_number < width && n >= min && n <= height
                    {
                        break (column_number, n - 1);
                    }
                    println!("Column and row must be within valid bounds.");
                }
                Err(_) => println!("Invalid input. Please enter a valid column and row."),
            }
        };

        (column, row)
    }

    #[must_use]
    pub fn read_input_mode(prompt: &str) -> InputMode {
        let input_mode = loop {
            let input = read_input(prompt);
            let lower_input = input.trim().to_lowercase();

            match lower_input.as_str() {
                "clear" | "c" => break InputMode::Clear,
                "flag" | "f" => break InputMode::Flag,
                "undo" | "u" => break InputMode::Undo,
                _ => println!("Invalid input. Please enter a valid input mode."),
            }
        };

        input_mode
    }

    #[must_use]
    pub fn read_game_mode(prompt: &str) -> GameMode {
        let game_mode = loop {
            let input = read_input(prompt);
            let lower_input = input.trim().to_lowercase();

            match lower_input.as_str() {
                "play" | "p" => break GameMode::Play,
                "configure" | "c" => break GameMode::Config,
                "quit" | "q" => break GameMode::Quit,
                _ => println!("Invalid input. Please select a menu option."),
            }
        };

        game_mode
    }

    #[must_use]
    pub fn read_difficulty(prompt: &str) -> Difficulty {
        let difficulty = loop {
            let input = read_input(prompt);
            let lower_input = input.trim().to_lowercase();

            match lower_input.as_str() {
                "easy" | "e" => break Difficulty::Easy,
                "medium" | "m" => break Difficulty::Medium,
                "hard" | "h" => break Difficulty::Hard,
                _ => println!("Invalid input. Please select a difficulty."),
            }
        };

        difficulty
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn enter_to_continue() {
        let mut input = String::new();
        print!("Press enter to continue... ");

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
    }
}

#[cfg(test)]
mod test {
    // Open entire root module to our test module
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
    fn gets_input_mode() {
        let mut state = GameState::new();

        state.input_mode = InputMode::Clear;

        assert_eq!(InputMode::Clear, state.get_input_mode());
    }

    #[test]
    fn gets_won() {
        let mut state = GameState::new();

        state.game_won = true;

        assert_eq!(true, state.get_won());
    }

    #[test]
    fn gets_mines() {
        let mut state = GameState::new();

        state.starting_mines = 6;

        assert_eq!(6, state.get_mines());
    }

    #[test]
    fn gets_game_mode() {
        let mut state = GameState::new();

        state.game_mode = GameMode::Play;

        assert_eq!(GameMode::Play, state.get_game_mode());
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
    fn sets_input_mode() {
        let mut state = GameState::new();
        state.set_input_mode(InputMode::Flag);

        assert_eq!(state.input_mode, InputMode::Flag)
    }

    #[test]
    fn sets_won() {
        let mut state = GameState::new();
        state.set_won(true);

        assert_eq!(state.game_won, true)
    }

    #[test]
    fn sets_mines() {
        let mut state = GameState::new();
        state.set_mines(6);

        assert_eq!(state.starting_mines, 6)
    }

    #[test]
    fn sets_game_mode() {
        let mut state = GameState::new();
        state.set_game_mode(GameMode::Play);

        assert_eq!(state.game_mode, GameMode::Play)
    }
}
