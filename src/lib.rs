#[derive(Debug)]
pub struct Sudoku {
    pub board: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Sudoku {
        Sudoku { board }
    }

    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn is_value_valid(&self, coordinates: (usize, usize), value: u8) -> bool {
        let (row, col) = coordinates;
        for current_row in 0..9 {
            if self.board[current_row][col] == value {
                return false;
            }
        }
        for current_col in 0..9 {
            if self.board[row][current_col] == value {
                return false;
            }
        }

        let start_row = row / 3 * 3;
        let start_col = col / 3 * 3;

        for current_row in start_row..start_col + 3 {
            for current_col in start_col..start_row + 3 {
                if self.board[current_row][current_col] == value {
                    return false;
                }
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        let empty_cell = self.find_empty_cell();

        if let Some((row, col)) = empty_cell {
            for value in 1..=9 {
                if self.is_value_valid((row, col), value) {
                    self.board[row][col] = value;
                    if self.solve() {
                        return true;
                    }
                }
                self.board[row][col] = 0;
            }
        } else {
            return true;
        }
        false
    }
}
