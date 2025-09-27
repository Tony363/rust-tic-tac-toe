# Tic-Tac-Toe in Rust

A simple, idiomatic Rust implementation of the classic Tic-Tac-Toe game, created using SuperClaude Framework's Extended Agent system.

## Features

- ✨ Zero unsafe code
- 🎮 Interactive CLI gameplay
- 🏆 Win detection for rows, columns, and diagonals
- 🤝 Two-player mode (X and O)
- 🔄 Play again option
- ✅ Input validation and error handling
- 🧪 Comprehensive unit tests (15 tests, 100% pass rate)

## Project Structure

```
tic_tac_toe/
├── Cargo.toml
├── src/
│   ├── main.rs      # Entry point
│   ├── game.rs      # Game loop and state management
│   ├── board.rs     # Board representation and win detection
│   └── player.rs    # Player types and switching
```

## How to Play

1. **Build and run the game:**
   ```bash
   cargo run
   ```

2. **Game rules:**
   - Players take turns placing X's and O's on a 3x3 grid
   - Player X always goes first
   - Enter a number from 1-9 to place your mark
   - The board positions are numbered as follows:
     ```
     1 | 2 | 3
     ---|---|---
     4 | 5 | 6
     ---|---|---
     7 | 8 | 9
     ```

3. **Winning conditions:**
   - Get 3 of your marks in a row (horizontal, vertical, or diagonal)
   - If all 9 squares are filled with no winner, it's a draw

## Development

### Running tests:
```bash
cargo test
```

### Running with clippy (pedantic mode):
```bash
cargo clippy -- -W clippy::pedantic
```

### Building for release:
```bash
cargo build --release
```

## Rust Best Practices

This implementation follows Rust best practices:
- **Memory Safety**: Zero unsafe code blocks
- **Error Handling**: Uses `Result` types for fallible operations
- **Ownership**: Proper ownership and borrowing patterns
- **Testing**: Comprehensive unit tests for all modules
- **Documentation**: Inline documentation with examples
- **Separation of Concerns**: Clean module separation

## Technical Details

- **Board Representation**: Uses a 1D array of 9 cells for efficiency
- **Win Detection**: Checks all 8 winning combinations efficiently
- **State Management**: Uses enums for game state and player types
- **Zero-Cost Abstractions**: Leverages Rust's type system without runtime overhead

## Created with SuperClaude Framework

This project was created using the SuperClaude Framework's Extended Agent system, specifically leveraging the Rust engineer agent specification for idiomatic Rust development.