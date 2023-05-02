# MINESWEEPER

#### Video Demo:  <URL HERE>

#### Description: Create a CLI Minesweeper game using idiomatic Rust, Test Driven Development, and classic game design patterns (IE, the 'game loop' design pattern)

## Introduction to the Project

Minesweeper is a classic game from my childhood: it was one of the few interesting games that came bundled with windows back in the days of my youth. There was something about the mystique of an unassuming game with simple controls and no instruction manual that was fascinating to me. I still remember the process of figuring out the rules by process of elimination like a puzzle that had to be solved before the meat of the game could even be played, and I remember making crude hand-written notes about the locations of suspected mines before I discovered flags and recognized their purpose. This process of discovery is actually much of what piqued my initial interest in computers and the games they allowed. As a result, I have a healthy respect and sentimental fondness of this simple to learn yet hard to master puzzle game about finding all of the mines without setting them off.

Now, as a matter of habit, I recreate Minesweeper whenever I want to really learn a new programming language. At first glance, the game is simple enough to implement. It requires a board, some mines, and empty squares that identify the number of adjacent mines. Once one looks beneath the surface, though, they find that Minesweeper digs to the core of what makes games tick. It lays bare how a language handles structuring data, how it defines logic to operate on that data, how it processes inputs, and how to package all of the core concepts of good software design into clean and efficient source code that "just works". I did it, or at least began the process of doing it, in Scratch for week 0: how better to show growth and wrap the course up in a neat bow than to finish what I started in a brand new programming language that represents the culmination of everything we've learned in this course?

## Goals

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
  - Splitting the design into discrete versions from conception to a fully featured minimum viable product

## First Steps

Every piece of software has to start somewhere. Rather than attempt to make a full piece of software on the word go, a designer has to make a basic framework with which they will begin to structure their codebase. Here, I've started by 'blocking' together an initial structure for the program and game loop:

### Blocking out the Game Loop

~~~rust
/*
 Defines a module within the project's 
 module tree into which we separate elements 
 of the main game loop
*/
mod game_loop {

    /* 
     We will use a 'struct' to group all data related to
     the game state so that it can be processed consistently,
     efficiently, and in an organized fashion
    */  
    struct GameState {
        TODO
    }

    /* 
     An implementation block allows us to group methods
     that directly interact with the GameState struct.
     This combined with the GameState struct is similar to the 
     Functionality of a 'Class' in the
     object-oriented programming paradigm
    */
    impl GameState {
        TODO
    }

    /* 
     The 'play' function contains the core structure
     of our game loop logic.
     It and all of it's associated functions interact with
     the GameState in some way; however, they will interact 
     indirectly through 'getter' and 'setter' methods
     defined in the implementation block,
     so they will remain a separate part of the game_loop module.
    */    
    play() {
        // TODO
    
        // Reset the game state after a game over
        reset();
    
        // Draw the initial game state
        draw();
    
        loop {
            /* 
             Allows us to break out of the
             game loop for "game over" screens and
             returning to the "title menu" once
             those have been implemented
            */
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
    /*
     This function ensures the user can
     continue a session without any risk
     that game states from a previous session
     will affect the state of a new game
    */
    Reset() {
        TODO
    }
    /*
     Here we will take console input from the user and 
     store it for the 'update' function to operate upon.
    */
    process_input() {
       // TODO
    }
    /*
     The update function will handle updating
     the game state based on user input.
     It will determine what GameState fields need
     to be changed using GameState's 'setter' methods
    */
    update() {
       // TODO
    }
    /*
     Draw will simply use 'getter' methods
     to determine the game state.
     It will then 'render' the 
     updated game state to the user's screen
    */
    draw() {
       // TODO
    }
}
~~~

With the framework above we will start building a working "game". First, we want to take the structure and turn it into a prototype for the logic that will eventually become a game. This is going to be accomplished with a simple program that prompts a user and then draws echoes their input to the screen with an escape hatch string for when they are done "playing". Already the code is becoming too large to process in one chunk, so we will start with the 'game_loop' module and the GameState struct (as well as its associated implementation blocks):

## Filling out the Framework with Prototype Logic

~~~rust
// Provide structure to game data

/*
 GameState now has two fields: 
 one holds user input from each iteration
 of the main loop, the other tells 
 the game logic whether the user
 wants to end the game.
*/
pub struct GameState {
    pub input: String,
    pub game_over: bool,
}

/*
 This implements the 'Default' trait on our 'GameState' struct.
 Default is a part of the Rust standard library which
 is often expected by Rust developers when dealing with Types
 that implement a ::new() method. 
 It sets the Type to a reasonable set of default values.
*/
impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

/*
 Below we implement a series of methods to act upon our
 'GameState' struct.
 These include a default 'constructor' (the 'new' function),
 as well as 'getter' and 'setter' methods to either
 retrieve the values of our fields, or set them.
 This allows us to create a public 'API' while
 protecting our GameState from unintended changes.
 Several of our methods also use the '#[must_use]'
 attribute so that the compiler can ensure
 the results of the functions aren't left unused.
*/
impl GameState {

    /*
     This is our default constructor
     to instantiate a new GameState
    */
    #[must_use]
    pub fn new() -> GameState {
        GameState {
            input: String::new(),
            game_over: false,
        }
    }

    /*
     This is a 'getter' method which returns a
     clone of our structs 'input' field.
     Cloning is relatively inefficient though,
     so we may change this in the future.
    */
    #[must_use]
    pub fn get_input(&self) -> String {
        self.input.clone()
    }

    /*
     This is a 'getter' method that allows us to
     check whether a 'game_over' state has been reached.
    */
    #[must_use]
    pub fn get_game_over(&self) -> bool {
        self.game_over
    }

    /*
     This is a 'setter' method that allows us to
     update the 'input' field with a new String
    */
    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }

    /*
     This is a 'setter' method that allows us to
     set the 'game_over' field to true or false
     (though the current prototype 'game'
     has no real reason to set 'false')
    */
    pub fn set_game_over(&mut self, game_over: bool) {
        self.game_over = game_over;
    }
}
~~~

In a future iteration of our program we may want to move the 'GameState' struct out of the 'game_loop' module for two key reasons.

1. In order to test our 'getter' and 'setter' methods properly, we actually have to make our fields public. If we move 'GameState' to the root module of 'lib.rs', our 'test' module can access private fields since they come from a parent module.
2. GameState isn't necessarily a part of our 'game_loop' since the game loop only reads and operates on state. It then makes sense to decouple GameState and the game loop to ensure our game loop only operates on GameState using the accepted public API (IE, 'getters' and 'setters') instead of operating directly on private fields.

Next, we will take a look at the game loop itself:

~~~rust

/*
 The 'play' function is where we set up
 the game loop to be called from our 'main' function.
 This is, in many ways, equivalent to 
 the 'run' function in most Rust programs;
 however, I decided to call it 'play'
 as that is more appropriate for a game.
*/
pub fn play() {
    /*
     Here we construct a new 'GameState'
     using the default constructor '::new()'
     We define it as mutable since a game without
     interaction isn't a game at all.
    */
    let mut state = GameState::new();

    /*
     We need to pass the game state mutably 
     to all of our functions which operate on
     state in order for them to properly
     update/render our game.

     Resets our game state to it's default values.
     This may be discovered unnecessary depending
     on how we implement the game proper.
    */
    reset(&mut state);

    // Draw the initial game state
    draw(&mut state);

    loop {
        
        /*
         Here, we use a 'getter' method
         to set a game_over flag.
         It may be better to move this 'let' binding
         outside of the loop since we don't need to shadow
         our game_over flag every time the loop iterates.
        */
        let game_over = state.get_game_over();

        /*
         We check to see if we have reached a game over state.
         Future iterations of the program will likely use 
         more logic in a 'game_over' state than this version
         which simply breaks out of the loop,
         thus ending the program.
        */
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

/*
 Here we find the actual logic for the functions
 associated with our game loop.
 This also shows a major ergonomic advantage of Rust:
 functions do not require forward declaration.
 If a function is used before it is declared,
 the program will simply check for it below the first use.
 This is enabled by the compiler's extensive analysis;
 however, this also contributes to above average compile times.
 The benefits seem to outweigh slow compile times, though,
 since it avoids a lot of copy paste work and
 necessity of updating multiple forward declarations
 in addition to any uses of a function throughout the code.

 We can see how the public 'setter' methods
 allow us to control how the game loop can and
 cannot interact with the game's state.
 Reset uses that API to reset our game state
 to the default values; however,
 it currently sets each field individually.
 As the 'GameState' struct (and, by extension, the game state)
 become more complex.
 It probably makes more sense to use the default 'constructor'
 method to reset the fields to their default implementation.
*/
fn reset(state: &mut GameState) {
    state.set_input(String::new());
    state.set_game_over(false);
}

/*
 Here we use a function from a newly created 'input_handler'
 module to read input from a user.
 We will get to the 'input_handler' module later on; however,
 let it suffice to say, for now, that
 the 'read_input' function implements some
 basic input processing to remove whitespace
 and convert the input to lowercase.

 Beyond that, 'process_input' uses the input with
 the 'set_input' method in order to update 
 the 'input' field of our 'GameState'.
*/
fn process_input(state: &mut GameState) {
    let input = input_handler::read_input("Enter a message to be echoed: ");
    state.set_input(input);
}

/*
 The update method currently only uses the 'get_input'
 method from 'GameState' to determine whether the user
 has typed '.exit' to indicate they want to end the game.
 If this is the case, we use our 'setter' method for
 the 'game_over' field to indicate a game over state has been reached.
*/
pub fn update(state: &mut GameState) {
    if state.get_input() == ".exit" {
        state.set_game_over(true);
    };
}

/*
 Finally, the 'draw' function checks whether we are
 in a game over state and, if we aren't,
 it renders our game state to the screen.
 We should likely change the if statement to directly check
 the 'game_over' field for better clarity,
 and we may also want to have it render a game over screen
 in the case where we have reached a game_over state
 (this is currently done by the 'main' function).
 In the future, we will also want to clear our previous
 game state from the screen; however,
 we have yet to decide how to do so in a
 platform independent fashion.
*/
fn draw(state: &mut GameState) {
    if state.get_input() != ".exit" {
        println!("Input text and I will repeat it back to you!\n
            (Type '.exit' to quit the 'game')\n");
        println!("{}\n", state.get_input());
    }
}
~~~

There are a few things we will need to change as we go forward with the prototype logic, many items of which are already called out in the commentary above, but as it stands we have a functioning programming replete with a meager "gameplay" loop and a simple exit condition! Our "game" has earned every double quotes I've given it at this point considering how it lacks basically all of the elements of good gameplay design, but at the prototype phase all we really need is something to prove that the engine runs before we break it with the early iterations of the program we actually want to build. Before we start on that, though, we have a couple more modules we need to get to, starting with the new 'input_handler' module we created for our 'process_input' function:

~~~rust
/*
 Here is our 'input_handler' module.
 We can see it's pretty bare at the moment in spite of 
 the relative complexity of the 'read_input'
 function compared to other functions.
 Still, a game is mostly about processing and
 responding to user input,
 so as our game becomes more complex than 
 the prototype logic we've written: we can be certain
 that much more input handling will be necessary over
 the life of our program.
*/
pub mod input_handler {

    use std::io::{self, Write};

    /*
     On to the actual function we have.
     We start out with something that simply takes
     a prompt as a parameter and then spits out 
     a string based on some user input.
     Most of the complexity comes from the
     error handling we've implemented.
     There are two types of errors we can expect with an input reader:
     pressing the wrong keys at the wrong time
     (very difficult when reading strings),
     and issues with the standard input/output streams.
     For the former, we simply re-prompt the user,
     but the latter requires more robust error handling
     which we will eventually improve as our use case evolves.
    
     You may also notice the '# Panics' comments
     which we use to inform anyone reading our source code
     that there are potential panic points within a given piece of code.
     For those unfamiliar, a 'Panic' in Rust is
     an orderly crash that cleans up after itself.
    */
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // This function is unlikely to
                                         // panic under normal circumstances

    pub fn read_input(prompt: &str ) -> String { // Might want to make a configuration
                                                 // parameter for case sensitivity
                                                 // that defaults to false
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

            /*
             The next two 'if' statements process the input
             to ensure common escape characters aren't included
             when we return the input.
             This is necessary if we are checking the input
             against a given value.
            */    
            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }

            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }

            /*
             This checks that, after our input is processed,
             the input stream isn't empty.
             We will assume that the user doesn't intend
             to enter nothing and re-prompt them.
            */
            if input.is_empty() {
                continue;
            }
            
            /*
             If everything else succeeds,
             we break out of the loop
             and move on to the return expression
            */
            break;
        }

        /*
         Finally, we return the users input
         minus any erroneous capitalization.
         This should likely be made configurable with
         a default initialized parameter.
         One more thing that may have been noticed: in Rust,
         writing 'return' is superfluous in cases where
         you aren't returning early from a function.
         Instead, Rust simply assumes that the last
         statement in the function without
         a semicolon is our return statement.
        */
        input.to_lowercase()
    }
}
~~~

The 'input_handler' module will be the focus of a lot of our future changes, as how you handle input is a critical part of building a full game. One of the first things we should consider doing, though, is making the error handling part of "handling input" a bit more robust. We don't want to slow down prototyping too much for this since nothing above represents even a near final state, but we also don't want to ignore such an important part of creating a fully formed piece of software until it's already complete. That just invites laziness right at the end. Speaking of ensuring we create a well-formed piece of software, next we will look at our very important 'test' module:

~~~rust

/*
 Rust comes packaged with both unit testing and integration testing by default.
 With a simple command in its package manager 'cargo' we can run each of the below
 unit tests and receive a summary report.
 This is fantastic for Test Driven Development
 as it allows tests to be made the motive power
 of development right out of the 'box'.
*/
#[cfg(test)]
mod test {
    /*
     Unlike our other 'use' import statements,
     this one brings most everything from
     the module tree into scope.
     Very convenient when you want to test everything.
    */
    use super::*;

    /*
     Each test below was created before the function they were intended to test.
     This is a part of the Test Driven Development cycle:
     understand a feature to be implemented,
     write a failing unit test for the intended functionality,
     and then write the code to make that unit test pass.
     From there, you refactor the code and re-run
     the test to make sure it is still working as intended!
     I find this very similar to the 'Plan, Do, Study, Act'
     or PDSA cycle from the Toyota Production system.
    */
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
~~~

Test Driven Development and writing unit tests are the two concepts most new to me at this point, so I admittedly can only parrot what I've learned from trusted sources. I also may not be writing the best unit tests; however, the power is still there insofar as the basics are concerned. We now have a series of tests that can be modified and added to as the program evolves into a real game. With that in mind, we will now segue into a discussion of version control. This documentation can only describe the evolution of our program in discreet meaningful version jumps (although, until we start the work of turning our game engine into a minesweeper game, we will avoid actually changing version numbers). Any more fine grained and we would run into a couple of problems such as giving too much information to be meaningful and slowing development of the program to a crawl. Our single major iteration of taking the framework we blocked out and turning it into a working "prototype" game took something on the order of 12 individual commits to our git repository. Each of those commits represented the implementation of a single piece of functionality, and yet could/should probably have been broken down even further.

To get a more full picture of the minute details of this process one may look at the public repository and see exactly what changes are being made in what order which additionally allows us to go back and find the problem if one of our changes breaks the software. One note, I try to keep to a policy of never pushing a series of commits to the remote unless:

- The code compiles
- The code represents a functioning iteration of the program we are trying to build
- and, the code passes all unit tests

This helps ensure that each public commit represents something that other developers, testers, and potentially users can reliably contribute to, test, or use. For a project where there is only one person developing it this may not seem particularly important; however, what if we were to add more developers? What if we wanted to ask someone to test it? Even if we never do either, we will likely want/need to work in a team at some point, and you perform in the same way you practice. If you cut corners when practicing what is to say you won't do the same in a production environment? This makes it of crucial importance to try and follow best practices insofar as you have learned and can possibly follow no matter the circumstances of your project.

### Summary

In this section, we created a framework from which to start developing our 'minesweeper' game. We then put together some simple logic, and tests to probe that logic, to prove the functionality of our framework and stand as a prototype from which to build the full program. We also called out where we may focus some future refactoring of our code base, and discussed some of features of the Rust programming language that were of benefit to our end goals. Finally, we spoke about how we are using Test Driven Development and version control with git to enhance our ability to ensure the program is, and remains, correct. From here, it is now time to start the work of turning our engine prototype into something that looks more like Minesweeper.

## Building Minesweeper from the Ground Up

We are now at the point where our codebase is too large to be noted in detail and broken down line-by-line with explanatory commentary, so we will begin to explain only the most pertinent changes. For a more in-depth history of changes, the github repository contains the entire history of commits and what each commit changed (as well as some helpful commit messages containing the broad strokes of each change). In this version of the program, we will call this the version 0.2.0, we have gone from a simplistic prototype that serves almost as a demo for our engine, to building an incredibly early functional prototype of the game proper:

We now have a "board" rendered,
~~~
     1  2  3  4  5
   1 -  -  -  -  - 
   2 -  -  -  -  - 
   3 -  -  -  -  - 
   4 -  -  -  -  - 
   5 -  -  -  -  - 
Select a hidden tile
~~~

The ability to process input,

~~~
     1  2  3  4  5
   1 -  -  -  -  - 
   2 -  -  -  -  - 
   3 -  -  -  -  - 
   4 -  -  -  -  - 
   5 -  -  -  -  - 
Select a hidden tile

Enter column: 3
Enter row: 3
Flag this tile? Y/n: 
~~~

And the ability to represent flags and hints,

~~~

     1  2  3  4  5
   1 -  -  -  -  - 
   2 -  -  -  -  - 
   3 -  -  F  0  - 
   4 -  -  -  -  - 
   5 -  -  -  -  - 
Select a hidden tile

~~~

We still have ahead of us the need to actually include mines in our game, give the hints meaning, and make many improvements to both the functionality and readability of our codebase. All of that is ahead, though, so why don't we focus on the broad changes made to the code base:

### Modeling Data in GameState

~~~rust
/*
 We now have a much larger 'GameState' struct with fields for a range of requirements.
 At present, the height and width fields will be fixed at size 5 for simplicity.
 We also have a Vector of 'tiles' which, thanks to the power of Rust's enums,
 Can hold everything we need to represent our game visually in only 2 options.
 Finally, we keep track of the player's 'selected_tile' as an index into 'tiles'
 so that we can separate processing of input from broader updates of state.
*/
pub struct GameState {
    game_over: bool,
    board_width: u32,
    board_height: u32,
    tiles: Vec<Tile>,
    selected_tile: Option<usize>,
}

/*
 Here is the enum mentioned. It has two modes for our tiles, each of which holds
 additional information using struct-like named fields.
 Because of this ability to hold additional information, we can capture all of
 the data necessary to represent our game with only two enumerated modes.
*/
#[derive(Debug, PartialEq)]
pub enum Tile {
    Hidden { has_mine: bool, flagged: bool },
    Revealed { has_mine: bool, hint: u32 },
}
~~~

I cannot stress enough how useful it is to be able to represent the state of our tiles so succinctly. This avoids a lot of the complexity we might deal with should we have been required to enumerate states like, "hidden with mine," or use a second data structure to store where mines were located. There still is a lot of data, but Rust's type system means we have no choice but to exhaustively account for all possible states which helps in avoiding subtle bugs. Now, we move on to the game logic itself. 

### Processing Input

We are going to skip the 'reset' function for now since it merely resets the GameState struct to default gameplay values. This takes us to the 'process_input' function:

~~~rust
fn process_input(state: &mut GameState) {
        loop {
            /*
             First thing we have to do is to ask for the user to select a tile.
             we use a 'read_as_int' function from our 'input_handler' module to turn
             text input into an integer value. We bound the entered numbers between 1
             and the dimensions of our board.
            */
            println!("Select a hidden tile\n");
            let mut column = input_handler::read_as_int("Enter column: ", 1, state.get_width());
            let mut row = input_handler::read_as_int("Enter row: ", 1, state.get_height());

            /*
             Then we have to zero index the user selections
            */
            row -= 1;
            column -= 1;

            /*
             Finally, we have to set the 'selected_tile' field of our 'GameState'
             struct by turning the selection into an index with a short equation
            */
            state.set_selected((row * state.get_width()) + column);

            let index = state.get_selected();

            /*
             Now that we have the selected_tile processed, we can use this to index
             into our 'tiles' Vector and match on the Tile's modality.
             The exhaustive matching was snipped for brevity, but we essentially
             Check the state of the tile selected to decide whether we need further
             input, or if we need to reprompt our user in the case where they selected
             an already revealed tile.
            */
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
                    } else  { // snip ...
            }
        }
    }
}
~~~

Once we've processed the user's input, we call a simple update function which just checks if the tile at the selected index is a revealed mine. In that case, we set the 'game_over' field of our 'GameState' to true before rendering our state to the screen with the 'draw' function. Speaking of which, let's move onto examining, 'draw'.

### Rendering the Game to our Display

The draw function is quite simple:

~~~rust
    fn draw(state: &mut GameState) {
        
        /*
         First we simply check for a 'game over' state. If this has occured
         we provide a message informing the user that they lost.
         We then provide an early return 
        */
        if state.get_game_over() {

            /*
             In a future iteration we will likely want to set
             all mines to a revealed state, and render that to screen.
             We might additionally provide some game stats such as how
             many mines they found, and how many 'turns' it took to get
             to the current state of the game.
            */
            println!("Game over!");
            return;
        }
        
        /*
         'clear_screen' is a simple utility function we added that uses the
         'crossterm' crate to execute a platform independent "clear" command
         on the console before we draw the new loop's contents.
        */
        clear_screen();

        /*
         This prints column numbers with a set character width to the column.
         We use print! to send our characters to standard output, and then
         println! to print a newline character and flush the stdout buffer
        */
        print!("   ");
        for col in 0..state.get_width() {
            print!("{:3}", col + 1);
        }
        println!();

        /*
         Finally, we use a double for loop to print each row
         Inside of these loops, we see the 'represent_tile' utility method.
         This method, when called, takes a tile state and turns it into
         a printable textual representation.
        */
        for row in 0..state.get_height() {
            print!("{:4}", row + 1); // Print the row number

            for col in 0..state.get_width() {
                let index = row * state.get_width() + col;
                let tile_representation = state.represent_tile(index);
                print!("{tile_representation:3}");
            }

            println!();
        }
    }
~~~

With this, we draw a board state with characters representing different possible modes of our Tiles. We also have a special "debug" tile representation in the case that there is somehow a value of "None" (represented by the '?' character). This state shouldn't ever happen so we know that, if one of our tiles is rendered as '?', we have a logic issue somewhere to be resolved. 

### Summary

In this section we started the first version of our software that actually looks like minesweeper. We have the skeleton of features like mines and hints, though much of the meat of that functionality remains. We have a board rendered to the screen, and the ability for the user to select a specific tile and act upon it (as by flagging, unflagging, or clearing the tile). There are no mines yet (or, rather, we haven't added them to our board), hints do not indicate what they are supposed to, and we have a lot of improvement we can do for processing input. 

## Echoes of a Complete Game

At this point, we've reached what we will call our internal 0.3.0 version. In this version, we have reached a point where you could technically call our minesweeper clone a game; however, it would be more accurate to call it a puzzle. Like a classic puzzle it is a box that can be opened once and solved, but once solved there is no dynamism to make revisiting our puzzle worthwhile. We have hardcoded board parameters, unchanging mine locations, and haven't even included a way to restart our game yet. We will likely be implementing some sort of "state machine" to give us the ability to efficiently change between menu, gameplay, and gameover screens. Still, for now let's focus on what we have complete: 

We have fully implemented hints and we can chain clear tiles surrounding a '0' hint to help avoid excesses of manual clearing,

~~~
     A  B  C  D  E  
   1 -  -  1  0  0 
   2 -  -  2  1  0 
   3 1  2  -  2  1 
   4 0  1  2  -  - 
   5 0  0  1  -  - 
Select a hidden tile

Enter column and row: 
~~~

We have the ability for the user to win the game,

~~~
     A  B  C  D  E  
   1 -  2  1  0  0 
   2 2  -  2  1  0 
   3 1  2  -  2  1 
   4 0  1  2  -  2 
   5 0  0  1  2  - 
Congratulations, you found all of the mines!
~~~

and a fully implemented gameover that reveals the locations of all mines,

~~~
     A  B  C  D  E  
   1 X  -  -  -  - 
   2 -  X  -  -  - 
   3 -  -  X  -  - 
   4 -  -  -  X  - 
   5 -  -  -  -  X 
Game over!
~~~

And finally, we have made great strides in improving the user experience for selecting, flagging, unflagging, and clearing mines.

~~~
     A  B  C  D  E  
   1 -  -  -  -  - 
   2 -  -  -  -  - 
   3 -  -  -  -  - 
   4 -  -  -  -  - 
   5 -  -  -  -  - 
Select a hidden tile

Enter column and row: a1
(C)lear, (F)lag, or (U)ndo selection? c
~~~

How did we get here? As before, the most illustrative way to get the full picture is to look at the commit history of the GitHub repository; however, we will go through some select additions. We've reworked a lot of our 'process_input' function, and made some changes to our update function but I want to spotlight the helper functions that have brought us to this point since they are where the major logic changes have been implemented. We will start with how we implemented hints:

### Calculating Nearby Mines

The "calculate_hint" function takes the index of the tile in question, checks neighboring tiles (through "find_neighbors" which we will explain shortly), and returns the number of neighboring tiles with mines:

~~~rust
    fn calculate_hint(state: &GameState, index: usize) -> u32 {
        
        /*
         A vector is initialized with the indices of
         neighboring tiles.
        */ 
        let neighbors = find_neighbors(state, index);

        /*
         We initialize a variable to hold the number
         of mines in neighbors.
        */
        let mut count = 0;

        /*
         Finally, we loop through the neighbors vector
         and use an if let statement to update the count
        */
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
~~~

The function is much simplified because we eventually extracted the logic for finding neighboring tiles. That refactor happened once we realized the logic was also needed in a second function:

### Revealing Neighboring Tiles

In the game of minesweeper, clicking a '0' hinted (empty) tile clears all nearby tiles. We needed a function to separate this logic out (so to minimize bloat in our update function), and that is where "reveal_neighbors" comes into play:

~~~rust
    fn reveal_neighbors(state: &mut GameState, index: usize) {
         
        /*
         Halfway through writing this function we recognized that we
         needed the same logic of finding neighboring tiles here
         that was required for calculating hints.
         At was at this point the decision was made to refactor
         that logic into it's own function and call it from both
         helpers. It was a helper helper, so to speak.
        */
        let neighbors = find_neighbors(state, index);

        /*
         Now we loop through our neighbors vector to clear them,
         but only so long as they are not mined or flagged.
        */ 
        for neighbor_index in neighbors {
            let tile = state.get_tile(neighbor_index);

            if let Tile::Hidden {
                has_mine: false,
                flagged: false,
            } = tile
            {
                /*
                 We need to calculate our hint to properly reveal the tile
                */
                let hint = calculate_hint(state, neighbor_index);

                state.set_tile(
                    neighbor_index,
                    Tile::Revealed {
                        has_mine: false,
                        hint,
                    },
                );

                if hint == 0 {
                    /*
                     Finally, if the tile is a zero_hinted tile,
                     we want to continue recursively checking neighbors
                     until we have cleared all appropriate tiles.
                    */
                    reveal_neighbors(state, neighbor_index);
                }
            }
        }
    }
~~~

As stated above, as we began this function we recognized shared logic that could be more cleanly
represented by its own function, 'find_neighbors':

### Refactoring Shared Logic

Before getting into, 'find_neighbors', let us talk in more depth about the process behind deciding to refactor. I don't necessarily fully believe in the concept of "DRY". There are times where I don't think the extraction of logic into a new function adds much to clarity or ergonomics, and so I generally adhere more closely to the "WET" concept: Write Everything Twice. As with anything, fanatical adherence to one or the other is a fools errand. If the repeated logic is complex enough that the functions greatly improve in clarity when it is extracted then you should do so immediately. If the logic is so prevalent and consistent that there is a sizeable ergonomic gain then you should refactor it. If the logic is simple enough that there is no benefit then you should hold off. The important part is recognizing, "why" you would be extracting it and only doing so when there is a good reason (be it clarity, ergonomics, or both).

Without further ado, let's find some neighbors:

~~~rust
    /*
     We cast a lot of our usize variables to isize to support negative indexes. 
     This allows us to 'look backwards' at neighbors in x and y directions to create a grid.
     We allow sign loss and possible wrap to go unwarned 
     because we ensure they will be within bounds.
    */
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn find_neighbors(state: &GameState, index: usize) -> Vec<usize> {
        let width = state.get_width() as isize;
        let height = state.get_height() as isize;

        let index_x = index as isize % width;
        let index_y = index as isize / width;
        
        /*
         We are storing the neighbor indices in a vector to send to other functions
        */
        let mut neighbors = Vec::new();

        /*
         Check the grid while making sure out of bounds indices are not
         added to the grid
        */
        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let neighbor_x = index_x + col_offset;
                let neighbor_y = index_y + row_offset;

                if neighbor_x >= 0 && neighbor_x < width && neighbor_y >= 0 && neighbor_y < height {
                    
                    /*
                     If the tile would be in bounds, it's index is calculated
                     and it is added to our vector
                    */
                    let neighbor_index = (neighbor_y * width + neighbor_x) as usize;
                    neighbors.push(neighbor_index);
                }
            }
        }

        neighbors
    }
~~~

Next we have a batch of two helpers from our "input_handler" module, "read_column_row", and "read_input_mode':

### Improving the User Experience

We will deal with these functions (and one important data structure added) as a batch:

~~~rust
    /*
     Let's start with the data structure. Instead of a series of
     yes or no questions: we will instead divide input into three types.
    */
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum InputMode {
        Clear, // This indicates the user wishes to clear the selected tile
        Flag, // This indicates the user wishes to flag or unflag the tile
        Undo, // This indicates the user wishes to select a different tile
    }

    /* Snip... */


    /*
     The first UX improvement is turning column and row selection
     into a single command. 
     'read_column_row' takes some bounds checking parameters
     and returns a tuple representing column and then row.
    */
    #[must_use]
    pub fn read_column_row(prompt: &str, min: u32, width: u32, height: u32) -> (u32, u32) {
        let (column, row) = loop {
            
            /*
             We use the usual input capture helper and then
             create an iterator over the input characters
            */
            let input = read_input(prompt);
            let mut chars = input.chars();

            /*
             We use our iterator to process the first char
             into an appropriate integer.
            */
            let column_char = chars.next();
            let column_number = if let Some(c) = column_char {
                (c.to_ascii_lowercase() as u32) - ('a' as u32)
            } else {
                println!("Invalid input. Please enter a valid column and row.");
                continue;
            };

            /*
             Then we turn the iterator of characters back into
             a string so we can parse it to an unsigned integer.
            */
            let row_number = chars.as_str().parse::<u32>();

            /*
             Finally, we do some error and bounds checking so
             we can reprompt the user if there is an issue with their input.
            */
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

        /*
         If everything checks out, we return our tuple
        */
        (column, row)
    }
    
    /*
     The next UX improvement uses our 'InputMode' enum
     To clearly delineate user intent for our input
     processing logic.
    */
    #[must_use]
    pub fn read_input_mode(prompt: &str) -> InputMode {
        /*
         We loop over, and process, user input
         until they enter a valid symbol. 
         We do so case insensitively.
        */
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

        /*
         After checking for invalid entries,
         we return the input mode.
        */  
        input_mode
    }
~~~

These improvements significantly increase ability to effectively select and process tiles for the user. Having letters and numbers to represent columns vs rows is a time tested methodology, and putting everything in one command means fewer opportunities for a mistake. Changing the interaction with the tiles to a simple modal process also provides more clarity to the user (and the codebase) than context based "yes/no" entries. Having a clear way to change selection also reduces frustration that might arise by mistyping a valid, but unintended, tile on the board. Instead a user can simply undo their selection and enter the correct tile. After this, there are a couple of less complex/important/informative helper functions. Namely: "check_for_win," which does exactly what it says by checking on each loop whether the player has revealed all of the mines, and "column_to_letter" which is an incredibly simple function that takes a column number and turns it into a character. 

### Summary

In this section, we finally have a playable minesweeper puzzle. We took the opportunity to explore when it is appropriate to extract logic out into its own function, and the dangers of holding didactically to any specific concept or paradigm. We have made significant improvements to the user experience by implementing chain clearing and more user friendly input handling. Now, we need to turn our puzzle into a game with randomized mine placement, board configuration, and a starting/ending menu. We will also need to add a mine tracker, and some end-of-game stats.

## Making Minesweeper Dynamic

We are officially at the point I would consider minimum viable product:

We have a fancy new menu screen:

~~~
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
      
Enter choice [(p)lay | (c)onfigure | (q)uit] : 
~~~

Three difficulty settings:

~~~

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
      
Enter choice [(p)lay | (c)onfigure | (q)uit] : c
Enter preferred difficulty level [(e)asy | (m)edium | (h)ard] : 
~~~

A board with indicators for turn count and remaining mines:

~~~
Turn: 1
Remaining Mines: 14

     A  B  C  D  E  F  G  H  
   1 -  -  -  -  -  -  -  - 
   2 -  -  -  -  -  -  -  - 
   3 -  -  -  -  -  -  -  - 
   4 -  -  -  -  -  -  -  - 
   5 -  -  -  -  -  -  -  - 
   6 -  -  -  -  -  -  -  - 
   7 -  -  -  -  -  -  -  - 
   8 -  -  -  -  -  -  -  - 
Select a hidden tile

Enter column and row: 
~~~

Randomized mine placement:

~~~
Turn: 27
Remaining Mines: 0

     A  B  C  D  E  F  G  H  
   1 1  1  1  F  F  F  2  0 
   2 F  1  1  3  F  F  2  0 
   3 1  1  0  1  2  2  1  0 
   4 0  0  0  0  0  0  0  0 
   5 1  1  2  1  1  0  0  0 
   6 1  F  3  F  1  0  0  0 
   7 2  3  F  3  3  2  3  2 
   8 1  F  3  F  2  F  F  F 
Congratulations, you found all of the mines!
Press enter to continue... 
~~~

And the ability to play again:

~~~
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
      
Enter choice [(p)lay | (c)onfigure | (q)uit] : 
~~~

as
