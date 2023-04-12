# MINESWEEPER
#### Video Demo:  <URL HERE>
#### Description: Create a CLI Minesweeper game using idiomatic Rust, Test Driven Development, and classic game design patterns (IE, the 'game loop' design pattern)

TODO

# Introduction to the Project
Minesweeper is a classic game from my childhood: it was one of the few interesting games that came bundled with windows back in the days of my youth. There was something about the mystique of an unassuming game with simple controls and no instruction manual that was fascinating to me. I still remember the process of figuring out the rules by process of elimination like a puzzle that had to be solved before the meat of the game could even be played, and I remember making crude hand-written notes about the locations of suspected mines before I discovered flags and recognized their purpose. This process of discovery is actually much of what piqued my initial interest in computers and the games they allowed. As a result, I have a healthy respect and sentimental fondness of this simple to learn yet hard to master puzzle game about finding all of the mines without setting them off.

Now, as a matter of habit, I recreate Minesweeper whenever I want to really learn a new programming language. At first glance, the game is simple enough to implement. It requires a board, some mines, and empty squares that identify the number of adjacent mines. Once one looks beneath the surface, though, they find that Minesweeper digs to the core of what makes games tick. It lays bare how a language handles structuring data, how it defines logic to operate on that data, how it processes inputs, and how to package all of the core concepts of good software design into clean and efficient source code that "just works". I did it, or at least began the process of doing it, in Scratch for week 0: how better to show growth and wrap the course up in a neat bow than to finish what I started in a brand new programming language that represents the culmination of everything we've learned in this course?

# Goals

- Implement a fully featured Minesweeper game on the command line using rust:
  - users should be able to configure the board size and other details about the game using a text based "title menu" before entering the actual game loop
  - users should be greeted by a "rendered" game board that gives them access to all necessary information to play the game
  - Using text input, the user should be able to select tiles, flag/unflag them, clear them, and "chord" the tiles to quickly clear large sections in one input.
  - Number of mines should scale properly with the size of the board and should be placed randomly each time the game state is reset to avoid identical boards
  - The user should be able to track the number of mines left to be discovered, and the number of "turns" taken to clear the board
  - The user should be able reset the board after every clear/game over to play again

- Demonstrate mastery of key programming concepts:
  - How to structure code to be clear and readable
  - How to implement the basic tenants of test driven development ensure changes do not break intended functionality
  - How to effectively implement standard design patterns
  - How to model data clearly so as to allow for easy debugging and proper function
  - How to use the basic features of version control to track changes effectively
  - And much more...

- Prove basic understanding of software design:
  - Documenting the process of design
  - Breaking a problem down into smaller problems for easier processing
  - Splitting the design into discrete versions from conception to minimum viable product to a richly featured "end" product

# First Steps

Every piece of software has to start somewhere. Rather than attempt to make a full piece of software on the word go, a designer has to make a basic framework with which they will begin to structure their codebase. Here, I've started by 'blocking' together an initial structure for the program and game loop:

'''

    // Defines a module within the project's module tree into which we separate elements of 
    // the main game loop
    mod game_loop {
        
        // We will use a 'struct' to group all data related to the game state so that it can
        // be processed consistently, efficiently, and in an organized fashion
        struct GameState {
            TODO
        }

        // An implementation block allows us to group methods that directly interact with the 
        // GameState struct. This combined with the GameState struct is similar to the 
        // Functionality of a 'Class' in the object-oriented programming paradigm
        impl GameState {
            TODO
        }

        // The 'play' function contains the core structure of our game loop logic.
        // It and all of it's associated functions interact with the GameState in some way; 
        // however, they will interact indirectly through 'getter' and 'setter' methods
        // defined in the implementation block, so they will remain a separate part of the 
        // game_loop module.
        play() {
            TODO
        
            // Reset the game state after a game over
            reset();
        
            // Draw the initial game state
            draw();
        
            loop {
                // Allows us to break out of the game loop for "game over" screens and
                // returning to the "title menu" once those have been implemented
                if game_over {
                    break;
                }
        
                // Process console input
                process_input();
        
                // Update the game state
                update();
        
                // Redraw game state at the end of each loop
                draw();
            }
        }
        
        // This function ensures the user can continue a session without any risk
        // that any game states from a previous session will affect the state of a new game
        Reset() {
            TODO
        }
        
        // Here we will take console input from the user and store it for the 'update' 
        // function to operate upon.
        process_input() {
            TODO
        }
        
        // The update function will handle updating the game state based on user input.
        // It will determine what GameState fields need to be changed using GameState's
        // 'setter' methods
        update() {
            TODO
        }
        
        // Draw will simply use 'getter' methods to determine the game state.
        // It will then 'render' the updated game state to the user's screen
        draw() {
            TODO
        }
    }

'''

With the framework above we will start building a working "game". First, we want to take the structure and turn it into a prototype for the logic that will eventually become a game. This is going to be accomplished with a simple program that prompts a user and then draws echoes their input to the screen with an escape hatch string for when they are done "playing". Already the code is becoming too large to process in one chunk, so we will start with the 'game_loop' module and the GameState struct (as well as its associated implementation blocks):

'''

    // Provide structure to game data
    //
    // GameState now has two fields: one holds user input from each iteration of the main loop, the tells the game logic whether the user wants to end the game.
    pub struct GameState {
        pub input: String,
        pub game_over: bool,
    }

    // This implements the 'Default' trait on our 'GameState' struct. Default is a part of the Rust standard library which is often expected by
    // Rust developers when dealing with Types that implement a ::new() method. It sets the Type to a reasonable set of default values.
    impl Default for GameState {
        fn default() -> Self {
            Self::new()
        }
    }

    // Below we implement a series of methods to act upon our 'GameState' struct. These include a default 'constructor' (the 'new' function), as well as 'getter' and 'setter'
    // methods to either retrieve the values of our fields, or set them. This allows us to create a public 'API' while protecting our GameState from unintended changes.
    // Several of our methods also use the '#[must_use]' attribute so that the compiler can ensure the results of the functions aren't left unused.
    impl GameState {
        // This is our default constructor to instantiate a new GameState
        #[must_use]
        pub fn new() -> GameState {
            GameState {
                input: String::new(),
                game_over: false,
            }
        }

        // This is a 'getter' method which returns a clone of our structs 'input' field. Cloning is relatively inefficient though, and so we may change this in the future.
        #[must_use]
        pub fn get_input(&self) -> String {
            self.input.clone()
        }

        // This is a 'getter' method that allows us to check whether a 'game_over' state has been reached.
        #[must_use]
        pub fn get_game_over(&self) -> bool {
            self.game_over
        }

        // This is a 'setter' method that allows us to update the 'input' field with a new String
        pub fn set_input(&mut self, input: String) {
            self.input = input;
        }

        // This is a 'setter' method that allows us to set the 'game_over' field to true or false (though the current prototype 'game' has no real reason to set 'false')
        pub fn set_game_over(&mut self, game_over: bool) {
            self.game_over = game_over;
        }
    }

'''

In a future iteration of our program we may want to move the 'GameState' struct out of the 'game_loop' module for two key reasons.

1. In order to test our 'getter' and 'setter' methods properly, we actually have to make our fields public. If we move 'GameState' to the root module of 'lib.rs', our 'test' module can access private fields since they come from a parent module.
2. GameState isn't necessarily a part of our 'game_loop' since the game loop only reads and operates on state. It then makes sense to decouple GameState and the game loop to ensure our game loop only operates on GameState using the accepted public API (IE, 'getters' and 'setters') instead of operating directly on private fields. 

Next, we will take a look at the game loop itself:

'''

    // The 'play' function is where we set up the loop to be called from our 'main' function. This is, in many ways, equivalent to the 'run' function in most Rust
    // programs; however, I decided to call it 'play' as this is supposed to be a game.
    pub fn play() {

        // Here we construct a new 'GameState' using the default constructor '::new()'
        // We define it as mutable because if we can't interact with state then we don't exactly have a game
        let mut state = GameState::new();

        // We need to pass the game state mutably to all of our functions which operate on state in order for them to properly update/render our game.

        // Resets our game state to it's default values. This may be discovered unnecessary depending on how we implement the game proper.
        reset(&mut state);

        // Draw the initial game state
        draw(&mut state);

        loop {
            // Here, we use a 'getter' method to set a game_over flag. It may be better to move this 'let' binding outside of the loop since we don't need to re-declare
            // (or, in this case, shadow) our game_over flag every time a loop iterates
            let game_over = state.get_game_over();

            // We check to see if we have reached a game over state. Future iterations of the program will likely use more logic in a 'game_over' state than this iteration
            // which simply breaks out of the loop, thus ending the program.
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

    // Here we find the actual logic for the functions associated with our game loop. This also shows a major ergonomic advantage of Rust:
    // functions do not require forward declaration. If a function is used before it is declared the program will simply check for declarations below the first use.
    // This is enabled by the compiler's extensive analysis; however, this also results in above average compile times. The benefits seem to outweigh slow compile times, though
    // since it avoids a lot of copy paste work and necessity of updating multiple forward declarations in addition to any uses of a function throughout the code.

    // Here we can see how the public 'setter' methods allow us to control how the game loop can and cannot interact with the game's state. Reset uses that API to reset
    // our game state to the default values; however, it currently sets each field individually. As the 'GameState' struct (and, by extension, the game state) become
    // more complex. It probably makes more sense to use the default 'constructor' method to reset the fields to their default implementation.
    fn reset(state: &mut GameState) {
        state.set_input(String::new());
        state.set_game_over(false);
    }

    // Here we use a function from a newly created 'input_handler' module to read input from a user. We will get to the 'input_handler' module later on; however,
    // let it suffice to say, for now, that the 'read_input' function implements some basic error handling to ensure the user actually inputs something
    // as well as converting input to lowercase.
    //
    // Beyond that, 'process_input' uses the input with the 'set_input' method in order to update the 'input' field of our 'GameState'.
    fn process_input(state: &mut GameState) {
        let input = input_handler::read_input("Enter a message to be echoed: ");
        state.set_input(input);
    }

    // The update method currently only uses the 'get_input' method from 'GameState' to determine whether the user has typed '.exit' to indicate they want
    // to end the game. If this is the case, we use our 'setter' method for the 'game_over' field to indicate a game over state has been reached.
    pub fn update(state: &mut GameState) {
        if state.get_input() == ".exit" {
            state.set_game_over(true);
        };
    }

    // Finally, the 'draw' function checks whether we are in a game over state and, if we aren't, it renders our game state to the screen.
    // We should likely change the if statement to directly check the 'game_over' field for better clarity, and we may also want to have it
    // render a game over screen of some kind in the case where we have reached a game_over state (this is currently done by the 'main' function).
    // In the future, we will also want to clear our previous game state from the screen; however, we have yet to decide how to do so in a platform independent fashion.
    fn draw(state: &mut GameState) {
        if state.get_input() != ".exit" {
            println!("Input text and I will repeat it back to you!\n    (Type '.exit' to quit the 'game')\n");
            println!("{}\n", state.get_input());
        }
    }

'''

There are a few things we will need to change as we go forward with the prototype logic, many items of which are already called out in the commentary above, but as it stands we have a functioning programming replete with a meager "gameplay" loop and a simple exit condition! Our "game" has earned every double quotes I've given it at this point considering how it lacks basically all of the elements of good gameplay design, but at the prototype phase all we really need is something to prove that the engine runs before we break it with the early iterations of the program we actually want to build. Before we start on that, though, we have a couple more modules we need to get to, starting with the new 'input_handler' module we created for our 'process_input' function:

'''

    // Here is our 'input_handler' module. We can see it's pretty bare at the moment in spite of the relative complexity of the 'read_input' function compared to other functions.
    // Still, a game is mostly about processing and responding to user input, so as our game becomes more complex than the prototype logic we've written: we can be certain
    // that much more input handling will be necessary over the life of our program.
    pub mod input_handler {

        use std::io::{self, Write};

        // On to the actual function we have. We start out with something that simply takes a prompt as a parameter and then spits out a string based on some user input
        // Most of the complexity comes from the basic error handling we've implemented. There are two types of errors we can expect with an input reader:
        // pressing the wrong keys at the wrong time (very difficult when reading strings), and issues with the standard input/output streams. For the former,
        // we simply re-prompt the user, but the latter requires more robust error handling which we will eventually improve as our use case evolves.
        //
        // You may also notice the '# Panics' comments which we use to inform anyone reading our source code that there are potential panic points within a given piece of code.
        // For those unfamiliar, a 'Panic' in Rust is an orderly crash that cleans up after itself.
        #[must_use]
        #[allow(clippy::missing_panics_doc)] // This function is unlikely to panic under normal circumstances
        pub fn read_input(prompt: &str ) -> String { // Might want to make a configuration parameter for case sensitivity that defaults to false
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

                // The next two 'if' statements process the input to ensure common escape characters aren't included when we return the input. Useful if we are checking
                // our input against some other input.
                if let Some('\n') = input.chars().next_back() {
                    input.pop();
                }

                if let Some('\r') = input.chars().next_back() {
                    input.pop();
                }

                // This checks that, after our input is processed, the input stream isn't empty. We will assume that the user doesn't intend to enter nothing and re-prompt them.
                if input.is_empty() {
                    continue;
                }
                
                // If everything else succeeds, we break out of the loop and move on to the return expression
                break;
            }
            // Finally, we return the users input, minus any erroneous capitalization. This should likely be made configurable with a default initialized parameter.
            // One more thing that may have been noticed: in Rust, writing 'return' is superfluous in cases where you aren't returning early from a function.
            // Instead, Rust simply assumes that the last statement in the function without a semicolon is our return statement.
            input.to_lowercase()
        }
    }

'''

The 'input_handler' module will be the focus of a lot of our future changes, as how you handle input is a critical part of building a full game. One of the first things we should consider doing, though, is making the error handling part of "handling input" a bit more robust. We don't want to slow down prototyping too much for this since nothing above represents even a near final state, but we also don't want to ignore such an important part of creating a fully formed piece of software until it's already complete. That just invites laziness right at the end. Speaking of ensuring we create a well-formed piece of software, next we will look at our very important 'test' module:

'''
    
    // Rust comes packaged with both unit testing and integration testing by default. With a simple command in its package manager 'cargo' we can run each of the below
    // unit tests and receive a summary report. This is fantastic for Test Driven Development as it allows tests to be made the motive power of development right
    // out of the 'box' so to speak.
    #[cfg(test)]
    mod test {
        // Unlike our other 'use' import statements, this one brings most everything from the module tree into scope. Very convenient when you want to test everything
        use super::*;

        // Each test below was created before the function they were intended to test. This is a part of the Test Driven Development cycle:
        // understand a feature to be implemented, write a failing unit test for the intended functionality, and then write the code to make that unit test pass.
        // From there, you refactor the code and re-run the test to make sure it is still working as intended! This is very similar to the 'Plan, Do, Study, Act'
        // or PDSA cycle from the Toyota Production system.
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

'''

Test Driven Development and writing unit tests are the two concepts most new to me at this point, so I admittedly can only parrot what I've learned from trusted sources. I also may not be writing the best unit tests; however, the power is still there insofar as the basics are concerned. We now have a series of tests that can be modified and added to as the program evolves into a real game. With that in mind, we will now segue into a discussion of version control. This documentation can only describe the evolution of our program in discreet meaningful version jumps (although, until we start the work of turning our game engine into a minesweeper game, we will avoid actually changing version numbers). Any more fine grained and we would run into a couple of problems such as giving too much information to be meaningful and slowing development of the program to a crawl. Our single major iteration of taking the framework we blocked out and turning it into a working "prototype" game took something on the order of 12 individual commits to our git repository. Each of those commits represented the implementation of a single piece of functionality, and yet could/should probably have been broken down even further.
To get a more full picture of the minute details of this process one may look at the public repository and see exactly what changes are being made in what order which additionally allows us to go back and find the problem if one of our changes breaks the software. One note, I try to keep to a policy of never pushing a series of commits to the remote unless:

   - The code compiles
   - The code represents a functioning iteration of the program we are trying to build
   - and, the code passes all unit tests

This helps ensure that each public commit represents something that other developers, testers, and potentially users can reliably contribute to, test, or use. For a project where there is only one person developing it this may not seem particularly important; however, what if we were to add more developers? What if we wanted to ask someone to test it? Even if we never do either, we will likely want/need to work in a team at some point, and you perform in the same way you practice. If you cut corners when practicing what is to say you won't do the same in a production environment? This makes it of crucial importance to try and follow best practices insofar as you have learned and can possibly follow no matter the circumstances of your project.

### Section Summary
In this section, we created a framework from which to start developing our 'minesweeper' game. We then put together some simple logic, and tests to probe that logic, to prove the functionality of our framework and stand as a prototype from which to build the full program. We also called out where we may focus some future refactoring of our code base, and discussed some of features of the Rust programming language that were of benefit to our end goals. Finally, we spoke about how we are using Test Driven Development and version control with git to enhance our ability to ensure the program is, and remains, correct. From here, it is now time to start working on turning our engine prototype into something that looks more like Minesweeper.


# Building Minesweeper from the Ground Up
