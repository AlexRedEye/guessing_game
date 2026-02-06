# Guessing Game

A classic number guessing game implemented in Rust with multiple difficulty levels.

> **AI Disclaimer:** All code was written by me, but this README was written by Copilot.

## About This Project

This project started as a learning exercise from Chapter 2 of [The Rust Programming Language](https://doc.rust-lang.org/book/) book and has been expanded with additional features. It demonstrates fundamental Rust concepts including:

- Variables and mutability
- Using external crates (dependencies)
- Handling user input
- Pattern matching with `match`
- Error handling
- Loops and control flow
- Comparing values
- Function organization

## What It Does

The program presents a menu where you can choose from three difficulty levels:
- **Easy:** Guess a number between 1 and 50
- **Medium:** Guess a number between 1 and 100
- **Hard:** Guess a number between 1 and 500

After selecting a difficulty, the game generates a random secret number within the chosen range. You'll then be prompted to guess the number. After each guess, the game tells you whether your guess was too high or too low. The game continues until you guess correctly, at which point it displays the number of attempts it took you to win.

## How to Build and Run

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.56 or later)

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

## How to Play

1. Run the program using `cargo run`
2. Select a difficulty level by entering 1, 2, or 3
3. When prompted, enter your guess
4. The game will tell you if your guess is too high or too low
5. Keep guessing until you find the secret number
6. The game will display how many attempts it took you to win
7. You can play again by selecting another difficulty level, or exit by choosing option 4

### Example Gameplay

```
Choose your difficulty.
1. Easy (1-50)
2. Medium (1-100)
3. Hard (1-500)
4. Exit Game
> 2
Guess the number!
Please input your guess.
> 50
You guessed: 50
Too small!
Please input your guess.
> 75
You guessed: 75
Too big!
Please input your guess.
> 63
You guessed: 63
You win, with 3 attempts!
Choose your difficulty.
1. Easy (1-50)
2. Medium (1-100)
3. Hard (1-500)
4. Exit Game
> 4
```

## Dependencies

- `rand` (0.8.5) - For generating random numbers

## Learning Resources

If you're also learning Rust, check out:
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get you used to reading and writing Rust code

## License

This is a learning project following the examples from The Rust Programming Language book.
