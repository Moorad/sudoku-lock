use std::io::{stdin, stdout, Stdin, Stdout, Write};

use sudoku_board::{Cursor, SudokuBoard};
use termion::{
    clear, cursor,
    event::{Event, Key},
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};
mod sudoku_board;

struct Game {
    sudoku_board: SudokuBoard,
    cursor: Cursor,
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    init(&mut stdout, stdin);
}

fn init(stdout: &mut RawTerminal<Stdout>, stdin: Stdin) {
    write!(stdout, "{}", clear::All).unwrap();

    let mut game = Game {
        // sudoku_board: SudokuBoard::random(),
        sudoku_board: SudokuBoard::from([
            [6, 0, 0, 1, 3, 2, 4, 0, 9],
            [7, 3, 4, 0, 0, 0, 0, 6, 0],
            [2, 1, 0, 0, 6, 0, 0, 0, 8],
            [9, 0, 6, 8, 0, 0, 0, 4, 5],
            [8, 5, 1, 3, 0, 0, 7, 0, 0],
            [0, 0, 0, 2, 0, 0, 0, 0, 1],
            [0, 0, 0, 4, 0, 0, 0, 0, 3],
            [3, 4, 0, 9, 0, 5, 0, 8, 0],
            [1, 9, 0, 6, 8, 0, 0, 5, 0],
        ]),
        cursor: Cursor::new(),
    };

    redraw(stdout, &game);

    for c in stdin.events() {
        let event = c.unwrap();

        match event {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Right) => game.cursor.right(),
            Event::Key(Key::Left) => game.cursor.left(),
            Event::Key(Key::Up) => game.cursor.up(),
            Event::Key(Key::Down) => game.cursor.down(),

            _ => {}
        }

        stdout.flush().unwrap();
        redraw(stdout, &game);
    }
}

fn redraw(stdout: &mut RawTerminal<Stdout>, game: &Game) {
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
    game.sudoku_board.display(stdout, &game.cursor);
}
