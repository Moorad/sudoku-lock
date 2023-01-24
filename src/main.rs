use std::io::{stdin, stdout, Stdin, Stdout, Write};

use game::Game;
use termion::{
    clear, cursor as termion_cursor,
    event::{Event, Key},
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

mod cursor;
mod game;
mod sudoku_board;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    init(&mut stdout, stdin);
}

fn init(stdout: &mut RawTerminal<Stdout>, stdin: Stdin) {
    write!(stdout, "{}", clear::All).unwrap();

    let mut game = Game::custom_test();

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
    write!(stdout, "{}", termion_cursor::Goto(1, 1)).unwrap();
    game.sudoku_board.display(stdout, &game.cursor);
}
