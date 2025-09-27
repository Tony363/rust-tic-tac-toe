//! Player module for tic-tac-toe game
//! 
//! This module defines the Player enum and associated functionality
//! for managing player turns and symbols in the game.

use std::fmt;

/// Represents a player in the tic-tac-toe game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    /// Returns the opposite player
    /// 
    /// # Examples
    /// 
    /// ```
    /// let player = Player::X;
    /// assert_eq!(player.switch(), Player::O);
    /// ```
    pub fn switch(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    /// Returns the symbol character for display
    pub fn symbol(self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_switch() {
        assert_eq!(Player::X.switch(), Player::O);
        assert_eq!(Player::O.switch(), Player::X);
    }

    #[test]
    fn test_player_symbol() {
        assert_eq!(Player::X.symbol(), 'X');
        assert_eq!(Player::O.symbol(), 'O');
    }

    #[test]
    fn test_player_display() {
        assert_eq!(format!("{}", Player::X), "X");
        assert_eq!(format!("{}", Player::O), "O");
    }
}