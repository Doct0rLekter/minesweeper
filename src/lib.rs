// Start blocking out a framework

// The below framework is loosely based on learnings from many sources on the 'game loop' design pattern
// The primary source referenced during the building of this framework is: https://www.gameprogrammingpatterns.com/game-loop.html
// This, of course, has to be translated into the world of rust
/*

mod game_loop {
    
    // Provide structure to game data

    struct GameState {
        TODO
    }

    impl GameState {
        TODO
    }

    play() {
        TODO
    
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
    
    Reset() {
        TODO
    }
    
    process_input() {
        TODO
    }
    
    update() {
        TODO
    }
    
    draw() {
        TODO
    }
}

*/



#[cfg(test)]
mod test {
    #[test]
    fn two_plus_two() {
        assert_eq!(4, (2 + 2))
    }
}
