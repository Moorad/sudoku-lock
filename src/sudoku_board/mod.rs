use std::cmp::Ordering;

#[derive(Debug)]
pub struct SudokuBoard {
    pub board: [[i32; 9]; 9],
}

impl SudokuBoard {
    // Get row by index
    pub fn row(&self, index: usize) -> Option<&[i32; 9]> {
        self.board.get(index)
    }

    // Get col by index
    pub fn col(&self, index: usize) -> Option<[&i32; 9]> {
        let mut col: [&i32; 9] = [&0; 9];

        for (i, elem) in self.board.iter().enumerate() {
            col[i] = elem.get(index)?;
        }

        Some(col)
    }

    // Get 3x3 box by x and y index
    pub fn grid_box(&self, x: usize, y: usize) -> Option<[&i32; 9]> {
        let abs_x = x * 3;
        let abs_y = y * 3;
        let mut _box: [&i32; 9] = [&0; 9];

        let mut index = 0;
        for i in 0..3 {
            for j in 0..3 {
                _box[index] = self.board.get(abs_y + i)?.get(abs_x + j)?;
                index += 1;
            }
        }

        Some(_box)
    }

    // Displays the sudoku board
    pub fn display(&self) {
        let mut horizontal_separator = String::new();
        horizontal_separator.push('|');
        // 9 is each number each taking 3 with whitespace + 2 pipes
        horizontal_separator.push_str(&"-".repeat(9 * 3 + 2));
        horizontal_separator.push('|');

        for (row_index, row) in self.board.iter().enumerate() {
            if row_index % 3 == 0 {
                println!("{}", horizontal_separator)
            }

            for (elem_index, element) in row.iter().enumerate() {
                // This handles the pipes every 3 elements
                // Also handles first row, left pipe because 3 % 0 == 0
                if elem_index % 3 == 0 {
                    print!("|")
                }

                match element.cmp(&0) {
                    Ordering::Equal => print!(" . "),
                    _ => print!(" {} ", element),
                };

                if elem_index == 8 {
                    print!("|")
                }
            }
            print!("\n");
            if row_index == 8 {
                println!("{}", horizontal_separator)
            }
        }
    }

    // Check current state of board follows sudoku rules
    pub fn is_safe(&self) {
        // for i in self.board
    }

    // Returns a new board that is solved
    pub fn solution(&self) -> SudokuBoard {
        let mut solution = SudokuBoard { board: self.board };

        solution.board[0][0] = 0;

        solution
    }
}
