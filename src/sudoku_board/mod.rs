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

    pub fn place(&mut self, x: usize, y: usize, val: i32) {
        self.board[y][x] = val;
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
    pub fn is_safe(&self) -> bool {
        fn check_array_is_safe(arr: &[i32; 9]) -> bool {
            let mut unseen: Vec<i32> = (1..10).collect();

            for cell in arr {
                if cell == &0 {
                    continue;
                }

                // We check if value is in unseen
                // If it is remove, otherwise value was seen
                // therefore board invalid
                if unseen.contains(cell) {
                    unseen.retain(|x| x != cell);
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
                    if !check_array_is_safe(&col.clone().map(|x| *x)) {
                        return false;
                    }
                }
                None => return false,
            }

            let x = i % 3;
            let y = i / 3;
            match self.grid_box(x, y) {
                Some(_box) => {
                    if !check_array_is_safe(&_box.clone().map(|x| *x)) {
                        return false;
                    }
                }
                None => return false,
            }
        }

        true
    }

    // Returns a new board that is solved
    pub fn solution(&self) -> Option<SudokuBoard> {
        let mut solution = SudokuBoard { board: self.board };

        fn recursive_solve(current_board: &mut SudokuBoard) -> bool {
            let possibilities: Vec<i32> = (1..10).collect();

            // Find an empty cell
            for i in 0..9 {
                for j in 0..9 {
                    let current_cell = &current_board.board[i][j];

                    // Checks if cell is empty
                    if current_cell == &0 {
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
