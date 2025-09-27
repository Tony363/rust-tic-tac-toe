//! Board module for tic-tac-toe game
//! 
//! This module manages the game board state, including cell placement,
//! win detection, and board display.

use crate::player::Player;
use std::fmt;

/// Represents a cell on the game board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Occupied(player) => write!(f, "{}", player),
        }
    }
}

/// The game board with a 3x3 grid
pub struct Board {
    cells: [Cell; 9],
}

impl Board {
    /// Creates a new empty board
    pub fn new() -> Self {
        Board {
            cells: [Cell::Empty; 9],
        }
    }

    /// Places a player's mark at the specified position
    /// 
    /// # Arguments
    /// 
    /// * `position` - The position (1-9) where the player wants to place their mark
    /// * `player` - The player making the move
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` if the move was successful
    /// * `Err(String)` if the position is invalid or already occupied
    pub fn place_mark(&mut self, position: usize, player: Player) -> Result<(), String> {
        if position < 1 || position > 9 {
            return Err(format!("Position must be between 1 and 9, got {}", position));
        }

        let index = position - 1;
        match self.cells[index] {
            Cell::Empty => {
                self.cells[index] = Cell::Occupied(player);
                Ok(())
            }
            Cell::Occupied(_) => Err(format!("Position {} is already occupied", position)),
        }
    }

    /// Checks if the board is full (no empty cells)
    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell != Cell::Empty)
    }

    /// Checks if there are any empty cells
    pub fn has_empty_cells(&self) -> bool {
        self.cells.iter().any(|&cell| cell == Cell::Empty)
    }

    /// Checks if a player has won the game
    /// 
    /// # Returns
    /// 
    /// * `Some(Player)` if a player has won
    /// * `None` if no player has won yet
    pub fn check_winner(&self) -> Option<Player> {
        // Define all winning combinations (rows, columns, diagonals)
        const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
            [0, 1, 2], // Top row
            [3, 4, 5], // Middle row
            [6, 7, 8], // Bottom row
            [0, 3, 6], // Left column
            [1, 4, 7], // Middle column
            [2, 5, 8], // Right column
            [0, 4, 8], // Top-left to bottom-right diagonal
            [2, 4, 6], // Top-right to bottom-left diagonal
        ];

        for combination in WINNING_COMBINATIONS {
            if let Cell::Occupied(player) = self.cells[combination[0]] {
                if self.cells[combination[1]] == Cell::Occupied(player)
                    && self.cells[combination[2]] == Cell::Occupied(player)
                {
                    return Some(player);
                }
            }
        }

        None
    }

    /// Returns a reference to the cells for inspection
    pub fn cells(&self) -> &[Cell; 9] {
        &self.cells
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n     |     |     ")?;
        writeln!(f, "  {}  |  {}  |  {}  ", self.cells[0], self.cells[1], self.cells[2])?;
        writeln!(f, "_____|_____|_____")?;
        writeln!(f, "     |     |     ")?;
        writeln!(f, "  {}  |  {}  |  {}  ", self.cells[3], self.cells[4], self.cells[5])?;
        writeln!(f, "_____|_____|_____")?;
        writeln!(f, "     |     |     ")?;
        writeln!(f, "  {}  |  {}  |  {}  ", self.cells[6], self.cells[7], self.cells[8])?;
        writeln!(f, "     |     |     ")?;
        writeln!(f)?;
        writeln!(f, "Positions (1-9):")?;
        writeln!(f, " 1 | 2 | 3 ")?;
        writeln!(f, "---|---|---")?;
        writeln!(f, " 4 | 5 | 6 ")?;
        writeln!(f, "---|---|---")?;
        writeln!(f, " 7 | 8 | 9 ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        assert!(board.has_empty_cells());
        assert!(!board.is_full());
    }

    #[test]
    fn test_place_mark_valid_position() {
        let mut board = Board::new();
        assert!(board.place_mark(1, Player::X).is_ok());
        assert_eq!(board.cells[0], Cell::Occupied(Player::X));
    }

    #[test]
    fn test_place_mark_invalid_position() {
        let mut board = Board::new();
        assert!(board.place_mark(0, Player::X).is_err());
        assert!(board.place_mark(10, Player::X).is_err());
    }

    #[test]
    fn test_place_mark_occupied_position() {
        let mut board = Board::new();
        board.place_mark(1, Player::X).unwrap();
        assert!(board.place_mark(1, Player::O).is_err());
    }

    #[test]
    fn test_check_winner_horizontal() {
        let mut board = Board::new();
        board.place_mark(1, Player::X).unwrap();
        board.place_mark(2, Player::X).unwrap();
        board.place_mark(3, Player::X).unwrap();
        assert_eq!(board.check_winner(), Some(Player::X));
    }

    #[test]
    fn test_check_winner_vertical() {
        let mut board = Board::new();
        board.place_mark(1, Player::O).unwrap();
        board.place_mark(4, Player::O).unwrap();
        board.place_mark(7, Player::O).unwrap();
        assert_eq!(board.check_winner(), Some(Player::O));
    }

    #[test]
    fn test_check_winner_diagonal() {
        let mut board = Board::new();
        board.place_mark(1, Player::X).unwrap();
        board.place_mark(5, Player::X).unwrap();
        board.place_mark(9, Player::X).unwrap();
        assert_eq!(board.check_winner(), Some(Player::X));
    }

    #[test]
    fn test_board_is_full() {
        let mut board = Board::new();
        for i in 1..=9 {
            let player = if i % 2 == 0 { Player::O } else { Player::X };
            board.place_mark(i, player).unwrap();
        }
        assert!(board.is_full());
        assert!(!board.has_empty_cells());
    }
}