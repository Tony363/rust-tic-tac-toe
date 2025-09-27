//! Game module for tic-tac-toe
//! 
//! This module manages the overall game flow, including the game loop,
//! player turns, and game state management.

use crate::board::Board;
use crate::player::Player;
use std::io::{self, Write};

/// Represents the current state of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

/// The main game controller
pub struct Game {
    board: Board,
    current_player: Player,
    state: GameState,
}

impl Game {
    /// Creates a new game with an empty board
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_player: Player::X,
            state: GameState::InProgress,
        }
    }

    /// Runs the main game loop
    pub fn run(&mut self) {
        println!("Welcome to Tic-Tac-Toe!");
        println!("========================\n");

        loop {
            self.display_board();

            if self.state != GameState::InProgress {
                self.display_result();
                if !self.ask_play_again() {
                    println!("Thanks for playing! Goodbye!");
                    break;
                }
                self.reset();
                continue;
            }

            self.process_turn();
        }
    }

    /// Processes a single turn
    fn process_turn(&mut self) {
        println!("Player {}'s turn", self.current_player);
        
        loop {
            let position = self.get_player_input();
            
            match self.board.place_mark(position, self.current_player) {
                Ok(()) => {
                    self.update_game_state();
                    if self.state == GameState::InProgress {
                        self.current_player = self.current_player.switch();
                    }
                    break;
                }
                Err(msg) => {
                    println!("Invalid move: {}", msg);
                    println!("Please try again.");
                }
            }
        }
    }

    /// Gets player input for position
    fn get_player_input(&self) -> usize {
        loop {
            print!("Enter position (1-9): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(position) => return position,
                Err(_) => {
                    println!("Please enter a valid number between 1 and 9.");
                }
            }
        }
    }

    /// Updates the game state after each move
    fn update_game_state(&mut self) {
        if let Some(winner) = self.board.check_winner() {
            self.state = GameState::Won(winner);
        } else if self.board.is_full() {
            self.state = GameState::Draw;
        }
    }

    /// Displays the current board
    fn display_board(&self) {
        println!("{}", self.board);
    }

    /// Displays the game result
    fn display_result(&self) {
        match self.state {
            GameState::Won(player) => {
                println!("🎉 Player {} wins! 🎉", player);
            }
            GameState::Draw => {
                println!("It's a draw! Well played both!");
            }
            GameState::InProgress => {
                // This shouldn't happen when this method is called
                unreachable!("display_result called while game is in progress");
            }
        }
    }

    /// Asks if players want to play again
    fn ask_play_again(&self) -> bool {
        loop {
            print!("Play again? (y/n): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => return true,
                "n" | "no" => return false,
                _ => println!("Please enter 'y' for yes or 'n' for no."),
            }
        }
    }

    /// Resets the game for a new round
    fn reset(&mut self) {
        self.board = Board::new();
        self.current_player = Player::X;
        self.state = GameState::InProgress;
        println!("\n=== New Game ===\n");
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new();
        assert_eq!(game.current_player, Player::X);
        assert_eq!(game.state, GameState::InProgress);
    }

    #[test]
    fn test_update_game_state_win() {
        let mut game = Game::new();
        // Set up a winning position for X
        game.board.place_mark(1, Player::X).unwrap();
        game.board.place_mark(2, Player::X).unwrap();
        game.board.place_mark(3, Player::X).unwrap();
        
        game.update_game_state();
        assert_eq!(game.state, GameState::Won(Player::X));
    }

    #[test]
    fn test_update_game_state_draw() {
        let mut game = Game::new();
        // Set up a draw position
        game.board.place_mark(1, Player::X).unwrap();
        game.board.place_mark(2, Player::O).unwrap();
        game.board.place_mark(3, Player::X).unwrap();
        game.board.place_mark(4, Player::O).unwrap();
        game.board.place_mark(5, Player::O).unwrap();
        game.board.place_mark(6, Player::X).unwrap();
        game.board.place_mark(7, Player::X).unwrap();
        game.board.place_mark(8, Player::X).unwrap();
        game.board.place_mark(9, Player::O).unwrap();
        
        game.update_game_state();
        assert_eq!(game.state, GameState::Draw);
    }

    #[test]
    fn test_reset() {
        let mut game = Game::new();
        game.board.place_mark(1, Player::X).unwrap();
        game.current_player = Player::O;
        game.state = GameState::Won(Player::X);
        
        game.reset();
        
        assert_eq!(game.current_player, Player::X);
        assert_eq!(game.state, GameState::InProgress);
        assert!(game.board.has_empty_cells());
    }
}