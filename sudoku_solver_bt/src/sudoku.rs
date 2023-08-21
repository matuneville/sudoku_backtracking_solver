use std::io::empty;
use std::time::{Instant, Duration};

pub struct Sudoku {
    board: Vec<Vec<u8>>,
    empty_cells: Vec<(u8,u8)>
}

impl Sudoku{

    pub fn new(board: Vec<Vec<u8>>) -> Self {
        let mut empty_cells: Vec<(u8,u8)> = Vec::new();
        for i in 0..9{  // O(1)
            for j in 0..9{  // O(1)
                if board[i][j] == 0{   // check if cell is empty
                    empty_cells.push((i as u8,j as u8));
                }
            }
        }
        return Self { board, empty_cells};
    }

    pub fn get_board(&self) -> & Vec<Vec<u8>> {
        return &self.board;
    }

    pub fn solve(&mut self) -> bool{
        if self.empty_cells.is_empty(){
            return true;
        }

        let pos = self.empty_cells.pop().unwrap();

        for number in 1..=9{ // O(1)
            let pos = (pos.0 as usize, pos.1 as usize); // change type to fit indexing
            if self.is_valid_pos(pos.0, pos.1, number){
                self.board[pos.0][pos.1] = number;
                if self.solve(){
                    return true;
                };
                self.board[pos.0][pos.1] = 0;
            }
        } // if didn't find a number to place, there is no solution

        self.empty_cells.push(pos); // return the cell back
        return false;
    }

    /*pub fn all_solutions(&self) -> Vec<Vec<Vec<u8>>>{
    }*/

    fn is_valid_pos(&self, row: usize, col: usize, num: u8) -> bool {
        for i in 0..9{
            if self.board[row][i] == num || self.board[i][col] == num {
                return false;
            }
        }

        let square_rows: (u8,u8) = crate::Sudoku::get_square_range(row);
        let square_cols: (u8,u8) = crate::Sudoku::get_square_range(col);

        for i in square_rows.0..square_rows.1 {
            for j in square_cols.0..square_cols.1 {
                if self.board[i as usize][j as usize] == num {return false; }
            }
        }

        return true;
    }

    fn get_square_range(coord: usize) -> (u8, u8) {
        if 0 <= coord && coord < 3 {
            (0, 3)
        } else if 3 <= coord && coord < 6 {
            (3, 6)
        } else {
            (6, 9)
        }
    }
}

impl Clone for Sudoku {
    fn clone(&self) -> Self {
        Sudoku {
            board: self.board.clone(),
            empty_cells: self.empty_cells.clone(),
        }
    }
}

pub fn test_sudoku(game: &mut Sudoku){

    let start_time = Instant::now();

    game.solve();

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    let elapsed_time_secs:f32 = elapsed_time.as_secs() as f32;
    let elapsed_time_ms = elapsed_time.as_millis() as f32;
    let elapsed_time_mic = elapsed_time.as_micros() as f32;


    for row in game.get_board() {
        println!("{:?}", row);
    }
    println!("Execution time in seconds: {}", elapsed_time_secs);
    println!("Execution time in milliseconds: {}", elapsed_time_ms);
    println!("Execution time in microseconds: {}", elapsed_time_mic);
}