use sudoku_board::SudokuBoard;

mod sudoku_board;

fn main() {
    let my_sudoku = SudokuBoard {
        board: [
            [4, 0, 5, 2, 6, 9, 7, 8, 1],
            [6, 8, 2, 5, 7, 1, 4, 9, 3],
            [1, 9, 7, 8, 3, 4, 5, 6, 2],
            [8, 2, 6, 1, 9, 5, 3, 4, 7],
            [3, 7, 4, 6, 8, 2, 9, 1, 5],
            [9, 5, 1, 7, 4, 3, 6, 2, 8],
            [5, 1, 9, 3, 2, 6, 8, 7, 4],
            [2, 4, 8, 9, 5, 7, 1, 3, 6],
            [7, 6, 3, 4, 1, 8, 2, 5, 9],
        ],
    };

    let my_other_sudoku = SudokuBoard {
        board: [
            [6, 0, 0, 1, 3, 2, 4, 0, 9],
            // [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [7, 3, 4, 0, 0, 0, 0, 6, 0],
            [2, 1, 0, 0, 6, 0, 0, 0, 8],
            [9, 0, 6, 8, 0, 0, 0, 4, 5],
            [8, 5, 1, 3, 0, 0, 7, 0, 0],
            [0, 0, 0, 2, 0, 0, 0, 0, 1],
            [0, 0, 0, 4, 0, 0, 0, 0, 3],
            [3, 4, 0, 9, 0, 5, 0, 8, 0],
            [1, 9, 0, 6, 8, 0, 0, 5, 0],
        ],
    };

    // my_other_sudoku.display();
    // println!("{:?}", my_other_sudoku.grid_box(3, 3))
    // println!("{}", my_other_sudoku.is_safe());
    my_other_sudoku.display();
    my_other_sudoku.solution().unwrap().display();
}
