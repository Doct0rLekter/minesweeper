// Set clippy to pedantic
#![warn(clippy::pedantic)]

use minesweeper::{self, game_loop};

fn main() {
    game_loop::play();

    println!("Hello, world!");
}
