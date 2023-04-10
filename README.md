# MINESWEEPER
#### Video Demo:  <URL HERE>
#### Description: Create a CLI Minesweeper game using idiomatic Rust, Test Driven Development, and classic game design patterns (IE, the 'game loop' design pattern)

TODO

## Introduction to the Project
Minesweeper is a classic game from my childhood: it was one of the few interesting games that came bundled with windows back in the days of my youth. There was something about the mystique of an unassuming game with simple controls and no instruction manual that was fascinating to me. I still remember the process of figuring out the rules by process of elimination like a puzzle that had to be solved before the meat of the game could even be played, and I remember making crude hand-written notes about the locations of suspected mines before I discovered flags and recognized their purpose. This process of discovery is actually much of what piqued my initial interest in computers and the games they allowed. As a result, I have a healthy respect and sentimental fondness of this simple to learn yet hard to master puzzle game about finding all of the mines without setting them off.

Now, as a matter of habit, I recreate Minesweeper whenever I want to really learn a new programming language. At first glance, the game is simple enough to implement. It requires a board, some mines, and empty squares that identify the number of adjacent mines. Once one looks beneath the surface, though, they find that Minesweeper digs to the core of what makes games tick. It lays bare how a language handles structuring data, how it defines logic to operate on that data, how it processes inputs, and how to package all of the core concepts of good software design into clean and efficient source code that "just works". I did it, or at least began the process of doing it, in Scratch: how better to show growth and wrap the course up in a neat package than to finish what I started in a brand new programming language that represents the culmination of everything we've learned in this course?

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
  - Splitting the design into discrete versions from conception to minimum viable product to a richly featured "end" product

## First Steps

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

With the framework above we will start building a working "game". First, we want to take the structure and turn it into a template for the logic that will eventually become a game. This is going to be accomplished with a simple program that prompts a user and then draws echoes their input to the screen with an escape hatch string for when they are done "playing":
