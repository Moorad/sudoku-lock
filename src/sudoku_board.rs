use std::io::{Stdout, Write};

use rand::{seq::SliceRandom, thread_rng};
use termion::{color, raw::RawTerminal};

use crate::cursor::Cursor;

#[derive(Debug, Clone, Copy)]
pub struct SudokuBoard {
    board: [[Cell; 9]; 9],
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    value: i32,
    mutable: bool,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            value: 0,
            mutable: false,
        }
    }

    fn from(value: i32, mutable: bool) -> Cell {
        Cell { value, mutable }
    }

    fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    fn set_mutable(&mut self, mutable: bool) {
        self.mutable = mutable;
    }

    fn value(&self) -> i32 {
        self.value
    }

    fn mutable(&self) -> bool {
        self.mutable
    }
}

impl SudokuBoard {
    pub fn new() -> SudokuBoard {
        SudokuBoard {
            board: [[Cell::new(); 9]; 9],
        }
    }

    pub fn from(board_structure: [[i32; 9]; 9]) -> SudokuBoard {
        let mut new_board = SudokuBoard::new();

        for i in 0..9 {
            for j in 0..9 {
                new_board.board[i][j] = Cell::from(board_structure[i][j], false)
            }
        }

        if !new_board.is_safe() {
            panic!("The board structure given does not follow the sudoku rules");
        }

        new_board
    }

    pub fn random() -> SudokuBoard {
        let mut board = SudokuBoard::new();

        fn recursive_create(current_board: &mut SudokuBoard) -> bool {
            let mut rng = thread_rng();
            let mut possibilities: Vec<i32> = (1..10).collect();
            possibilities.shuffle(&mut rng);

            // Find an empty cell
            for i in 0..9 {
                for j in 0..9 {
                    let current_cell = &current_board.board[i][j];

                    // Checks if cell is empty
                    if current_cell.value() == 0 {
                        let mut iter = possibilities.iter();

                        // Go through possible solutions for empty cell
                        loop {
                            // try placing, and check if its safe
                            // if safe recurse to next empty cell
                            // otherwise set placed cell back to empty and try next possibility
                            match iter.next() {
                                Some(val) => current_board.set_cell(j, i, val.clone()),
                                None => return false,
                            };

                            if current_board.is_safe() {
                                if recursive_create(current_board) {
                                    return true;
                                }
                            }

                            current_board.set_cell(j, i, 0);
                            continue;
                        }
                    }
                }
            }
            true
        }

        recursive_create(&mut board);
        board
    }

    // Get row by index
    pub fn row(&self, index: usize) -> Option<&[Cell; 9]> {
        self.board.get(index)
    }

    // Get col by index
    pub fn col(&self, index: usize) -> Option<[Cell; 9]> {
        let mut col: [Cell; 9] = [Cell::new(); 9];

        for (i, elem) in self.board.iter().enumerate() {
            col[i] = elem.get(index)?.clone();
        }

        Some(col)
    }

    // Get 3x3 box by x and y index
    pub fn grid_box(&self, x: usize, y: usize) -> Option<[Cell; 9]> {
        let abs_x = x * 3;
        let abs_y = y * 3;
        let mut _box: [Cell; 9] = [Cell::new(); 9];

        let mut index = 0;
        for i in 0..3 {
            for j in 0..3 {
                _box[index] = self.board.get(abs_y + i)?.get(abs_x + j)?.clone();
                index += 1;
            }
        }

        Some(_box)
    }

    pub fn set_cell(&mut self, x: usize, y: usize, val: i32) {
        self.board[y][x] = Cell::from(val, false);
    }

    pub fn place(&mut self, x: usize, y: usize, val: i32) {
        self.board[y][x] = Cell::from(val, true);
    }

    // Displays the sudoku board
    pub fn display(&self, stdout: &mut RawTerminal<Stdout>, cursor: &Cursor) {
        let mut horizontal_separator = String::new();
        horizontal_separator.push('|');
        // 9 is each number each taking 3 with whitespace + 2 pipes
        horizontal_separator.push_str(&"-".repeat(9 * 3 + 2));
        horizontal_separator.push('|');

        for (row_index, row) in self.board.iter().enumerate() {
            if row_index % 3 == 0 {
                write!(stdout, "{}\n\r", horizontal_separator).unwrap();
            }

            for (elem_index, element) in row.iter().enumerate() {
                // This handles the pipes every 3 elements
                // Also handles first row, left pipe because 3 % 0 == 0
                if elem_index % 3 == 0 {
                    write!(stdout, "|").unwrap();
                }

                let mut value = format!(" {} ", element.value());
                if element.value() == 0 {
                    value = " . ".to_string();
                }

                if elem_index == cursor.x && row_index == cursor.y {
                    write!(
                        stdout,
                        "{}{}{}",
                        color::Bg(color::Blue),
                        value,
                        color::Bg(color::Reset)
                    )
                    .unwrap();
                } else {
                    write!(stdout, "{}", value).unwrap();
                }

                if elem_index == 8 {
                    write!(stdout, "|\n\r").unwrap();
                }
            }
            if row_index == 8 {
                write!(stdout, "{}\n\r", horizontal_separator).unwrap();
            }
        }
    }

    // Check current state of board follows sudoku rules
    pub fn is_safe(&self) -> bool {
        fn check_array_is_safe(arr: &[Cell; 9]) -> bool {
            let mut unseen: Vec<i32> = (1..10).collect();

            for cell in arr {
                if cell.value() == 0 {
                    continue;
                }

                // We check if value is in unseen
                // If it is remove, otherwise value was seen
                // therefore board invalid
                if unseen.contains(&cell.value()) {
                    unseen.retain(|x| x != &cell.value());
                } else {
                    return false;
                }
            }

            true
        }
        for i in 0..9 {
            match self.row(i) {
                // if index was valid
                Some(row) => {
                    if !check_array_is_safe(row) {
                        return false;
                    }
                }
                None => return false,
            }

            // Same exact thing for cols
            match self.col(i) {
                Some(col) => {
                    if !check_array_is_safe(&col.clone()) {
                        return false;
                    }
                }
                None => return false,
            }

            let x = i % 3;
            let y = i / 3;
            match self.grid_box(x, y) {
                Some(_box) => {
                    if !check_array_is_safe(&_box.clone()) {
                        return false;
                    }
                }
                None => return false,
            }
        }

        true
    }

    // Returns a new board that is solved
    // Using backtracking algorithm (DFS)
    pub fn solution(&self) -> Option<SudokuBoard> {
        let mut solution = SudokuBoard {
            board: self.board.clone(),
        };

        fn recursive_solve(current_board: &mut SudokuBoard) -> bool {
            let possibilities: Vec<i32> = (1..10).collect();

            // Find an empty cell
            for i in 0..9 {
                for j in 0..9 {
                    let current_cell = &current_board.board[i][j];

                    // Checks if cell is empty
                    if current_cell.value() == 0 {
                        let mut iter = possibilities.iter();

                        // Go through possible solutions for empty cell
                        loop {
                            // try placing, and check if its safe
                            // if safe recurse to next empty cell
                            // otherwise set placed cell back to empty and try next possibility
                            match iter.next() {
                                Some(val) => current_board.place(j, i, val.clone()),
                                None => return false,
                            };

                            if current_board.is_safe() {
                                if recursive_solve(current_board) {
                                    return true;
                                }
                            }

                            current_board.place(j, i, 0);
                            continue;
                        }
                    }
                }
            }
            true
        }

        let found_solution = recursive_solve(&mut solution);

        if !found_solution {
            return None;
        }

        Some(solution)
    }
}
