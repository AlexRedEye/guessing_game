# Guessing Game

A classic number guessing game implemented in Rust as a learning project from [The Rust Programming Language](https://doc.rust-lang.org/book/) book (Chapter 2).

## About This Project

This project is part of my journey learning Rust programming. It follows the tutorial from "The Rust Programming Language" book (also known as "The Rust Book"), specifically the guessing game example from Chapter 2. This project helped me learn fundamental Rust concepts including:

- Variables and mutability
- Using external crates (dependencies)
- Handling user input
- Pattern matching with `match`
- Error handling
- Loops and control flow
- Comparing values

## What It Does

The program generates a random secret number between 1 and 100, and then prompts you to guess it. After each guess, it tells you whether your guess was too high or too low. The game continues until you guess correctly, at which point it displays the number of attempts it took you to win.

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
2. When prompted, enter a number between 1 and 100
3. The game will tell you if your guess is too high or too low
4. Keep guessing until you find the secret number
5. The game will display how many attempts it took you to win

### Example Gameplay

```
Guess the number!
Please input your guess.
50
You guessed: 50
Too small!
Please input your guess.
75
You guessed: 75
Too big!
Please input your guess.
63
You guessed: 63
You win, with 3 attempts!
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
