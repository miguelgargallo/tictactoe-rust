# Tic-Tac-Toe Rust

![tictactoe-rust](https://github.com/miguelgargallo/tictactoe-rust/assets/5947268/097e8209-f94c-4b8f-8a73-dc0a5dfd81a3)

This repository contains a simple implementation of the classic game "Tic-Tac-Toe" written in Rust. The game is played on a 3x3 board between two players, who take turns marking their symbol ('X' or 'O') on the board. The player who succeeds in placing three of their marks in a horizontal, vertical, or diagonal row is the winner.

## Installation

Before running this game, make sure you have Rust installed in your system. If not, you can download it from [here](https://www.rust-lang.org/tools/install).

To download the game, clone the repository to your local machine using the following command:

```bash
git clone https://github.com/miguelgargallo/tictactoe-rust
```

## Usage

Navigate to the game's directory:

```bash
cd tictactoe-rust
```

Run the game:

```bash
cargo run
```

The game board is a 3x3 grid, represented as follows:

```bash
  1 2 3
A . . .
B . . .
C . . .
```

Each cell in the grid is identified by a combination of a letter (A, B, C) and a number (1, 2, 3). For example, the top left cell is A1, the middle cell is `A1`, and the middle cell is `B2`, and so on.

When it's your turn, enter your move in the format `A1`, `B3`, `C2`, etc. The program will prompt you for your input.

## Contributing

Feel free to contribute to this project. Any contributions you make are greatly appreciated. Please read the [CONTRIBUTING.md](./CONTRIBUTING.md) for details on the code of conduct and the process for submitting pull requests.

## License

Permission is hereby granted, free of charge, to any person obtaining a copy of the Pylar AI software and associated documentation files (the "Software"), to visualize and use the Software solely for educational and informative purposes, subject to the following restrictions and conditions on the [License](./License).
