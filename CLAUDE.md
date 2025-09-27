# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Commands

### Development
- **Run the game**: `cargo run`
- **Run all tests**: `cargo test`
- **Run a single test**: `cargo test test_name` (e.g., `cargo test test_check_winner_horizontal`)
- **Run tests for specific module**: `cargo test --lib module_name` (e.g., `cargo test --lib board`)

### Code Quality
- **Check with clippy (pedantic mode)**: `cargo clippy -- -W clippy::pedantic`
- **Format code**: `cargo fmt`
- **Check formatting**: `cargo fmt -- --check`

### Build
- **Debug build**: `cargo build`
- **Release build**: `cargo build --release`
- **Clean build artifacts**: `cargo clean`

## Architecture Overview

This is a simple, idiomatic Rust implementation of Tic-Tac-Toe following clean separation of concerns:

### Module Structure and Responsibilities

- **`main.rs`**: Entry point, minimal bootstrapping only
- **`game.rs`**: Game controller and state management
  - Manages game loop and flow
  - Handles player input and validation
  - Coordinates between board and player modules
  - Implements `GameState` enum (InProgress, Won, Draw)
  
- **`board.rs`**: Board representation and game logic
  - Uses efficient 1D array representation (`[Cell; 9]`)
  - Handles move validation and placement
  - Implements win detection for all 8 winning combinations
  - Provides board display formatting
  
- **`player.rs`**: Player types and turn management
  - Simple enum (X, O) with switching logic
  - Handles player representation and display

### Key Design Patterns

- **Error Handling**: Uses `Result<T, String>` for fallible operations (e.g., placing marks)
- **State Management**: Enum-based state machine for game flow
- **Zero Unsafe Code**: All operations are memory-safe
- **Testing**: Each module has comprehensive unit tests co-located with the implementation

### Board Position Mapping
Positions 1-9 map to array indices 0-8:
```
1 | 2 | 3     →    [0] | [1] | [2]
4 | 5 | 6     →    [3] | [4] | [5]  
7 | 8 | 9     →    [6] | [7] | [8]
```

### Win Detection Algorithm
Checks 8 predefined winning combinations:
- 3 rows: [0,1,2], [3,4,5], [6,7,8]
- 3 columns: [0,3,6], [1,4,7], [2,5,8]
- 2 diagonals: [0,4,8], [2,4,6]