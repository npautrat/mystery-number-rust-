# Mystery Number (Rust)

A simple command-line game written in Rust where the player has to guess a random number between **1 and 100**.

## Prerequisites

- [Rust and Cargo](https://rustup.rs/) installed on your machine.

Verify your installation:
```bash
rustc --version
cargo --version
```

## Installation and Running

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/npautrat/mystery-number-rust-.git](https://github.com/npautrat/mystery-number-rust-.git)
   cd mystery-number-rust-
   ```

2. **Run the game:**
   ```bash
   cargo run
   ```
   *(Or `cargo run --release` to run an optimized build).*

## Game Rules

- The program generates a secret number between `1` and `100`.
- Enter a number in the terminal on each turn.
- The game tells you whether your guess is too low or too high until you find the right answer.
