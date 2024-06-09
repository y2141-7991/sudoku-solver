use sudoku_solver::Sudoku;

fn sudoku_solver(board: &[[u8; 9]; 9]) -> Option<[[u8; 9]; 9]> {
    let mut solver = Sudoku::new(*board);
    // Some(solver.board)
    if solver.solve() {
        Some(solver.board)
    } else {
        None
    }
}

fn main() {
    let board = [
        [3, 0, 6, 5, 0, 8, 4, 0, 0],
        [5, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 8, 7, 0, 0, 0, 0, 3, 1],
        [0, 0, 3, 0, 1, 0, 0, 8, 0],
        [9, 0, 0, 8, 6, 3, 0, 0, 5],
        [0, 5, 0, 0, 9, 0, 6, 0, 0],
        [1, 3, 0, 0, 0, 0, 2, 5, 0],
        [0, 0, 0, 0, 0, 0, 0, 7, 4],
        [0, 0, 5, 2, 0, 6, 3, 0, 0],
    ];

    let solved_board = sudoku_solver(&board);
    println!("{:?}", solved_board);
}
