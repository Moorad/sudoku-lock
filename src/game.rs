use crate::cursor::Cursor;
use crate::sudoku_board::SudokuBoard;

pub struct Game {
    pub sudoku_board: SudokuBoard,
    pub cursor: Cursor,
}

impl Game {
    pub fn new() -> Game {
        Game {
            sudoku_board: SudokuBoard::random(),
            cursor: Cursor::new(),
        }
    }

    // Will be removed
    pub fn custom_test() -> Game {
        Game {
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
        }
    }
}
