use sudoku_board::SudokuBoard;

mod sudoku_board;

fn main() {
    let my_sudoku = SudokuBoard::from([
        [6, 0, 0, 1, 3, 2, 4, 0, 9],
        [7, 3, 4, 0, 0, 0, 0, 6, 0],
        [2, 1, 0, 0, 6, 0, 0, 0, 8],
        [9, 0, 6, 8, 0, 0, 0, 4, 5],
        [8, 5, 1, 3, 0, 0, 7, 0, 0],
        [0, 0, 0, 2, 0, 0, 0, 0, 1],
        [0, 0, 0, 4, 0, 0, 0, 0, 3],
        [3, 4, 0, 9, 0, 5, 0, 8, 0],
        [1, 9, 0, 6, 8, 0, 0, 5, 0],
    ]);
    // my_other_sudoku.display();
    // println!("{:?}", my_other_sudoku.grid_box(3, 3))
    // println!("{}", my_other_sudoku.is_safe());
    // my_other_sudoku.display();
    // my_other_sudoku.solution().unwrap().display();
    let empty_sudoku = SudokuBoard::new();
    let random_sudoku = SudokuBoard::random();
    // random_sudoku.display();
    // println!("{}", random_sudoku.is_safe())
    random_sudoku.solution().unwrap().display();
}
