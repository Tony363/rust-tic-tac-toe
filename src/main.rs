//! Tic-Tac-Toe Game
//! 
//! A simple, idiomatic Rust implementation of the classic Tic-Tac-Toe game.
//! This implementation follows Rust best practices with zero unsafe code,
//! proper error handling, and clean separation of concerns.

mod board;
mod game;
mod player;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
