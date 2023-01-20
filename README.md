# Sudoku Lock

This is a program is a twisted way of creating an "anti procrastination" tool. The program works by locking the specified file for a period of time specified by the user. The key to unlock the file is a solution to a sudoku board generated before locking. If the user attempts to open the file during the locked period they will be presented with a sudoku board (or a set of sudoku boards). The user can either waste time solving the sudoku boards to obtain the key or just do work until the lock period ends :).

## Usage

Not available yet

## Building

To build this code manually you must have Rust and its associated tools (`rustup`, `rustc`, `cargo`, etc) which can be installed via https://www.rust-lang.org/tools/install

Compile and run:

```
cargo run
```

or alternatively

Compile and build:

```
cargo build
```

Manually run the binary:

```
./target/debug/hello_cargo
```

## Todo

Sudoku:

-   [ ] Display a test board
-   [ ] Add a cursor to the board
-   [ ] Add a simple move cursor system
-   [ ] Add a simple placing system
-   [ ] Add a board solution checker
-   [ ] Implement board solving algorithm
-   [ ] Add proper feedback system (including errors)
-   [ ] Randomly generate boards
-   [ ] Generate a key from a sudoku board solution
-   [ ] Create a set of sudoku boards that could be switched between
-   [ ] Add a config file
-   [ ] Encrypt/lock files
-   [ ] Add timing system to unlock after a period of time
-   [ ] Add a move mode
-   [ ] Add a place mode
