use std::cmp::Ordering;

#[derive(Debug)]
pub struct SudokuBoard {
    pub board: [[i32; 9]; 9],
}

impl SudokuBoard {
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
}
