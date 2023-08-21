mod sudoku;

use crate::sudoku::*;

fn main() {
    println!("Hello, world!");

    /*let test_board: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 6, 0, 9, 0, 3, 0],
        vec![8, 0, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 7, 0, 0, 0, 6, 0],
        vec![0, 6, 0, 5, 0, 0, 0, 7, 0],
        vec![0, 0, 0, 0, 1, 0, 4, 0, 0],
        vec![0, 0, 0, 0, 2, 0, 0, 0, 0],
        vec![2, 0, 0, 0, 0, 0, 5, 0, 4],
        vec![0, 0, 0, 3, 0, 0, 8, 0, 0],
        vec![0, 9, 0, 0, 0, 0, 0, 0, 0],
    ];*/

    let test_board: Vec<Vec<u8>> = vec![
        vec![0, 9, 0, 0, 0, 0, 7, 5, 0],
        vec![0, 0, 3, 0, 0, 2, 0, 0, 0],
        vec![4, 6, 2, 9, 0, 0, 0, 0, 8],
        vec![6, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 5, 0, 0, 4, 6, 8, 0],
        vec![7, 3, 0, 0, 8, 0, 2, 0, 4],
        vec![0, 5, 0, 0, 0, 0, 8, 0, 7],
        vec![0, 0, 0, 2, 3, 0, 0, 0, 6],
        vec![0, 2, 0, 6, 0, 5, 3, 4, 0],
    ];

    let mut game = Sudoku::new(test_board);

    game.solve();

    for row in game.get_board() {
        println!("{:?}", row);
    }
}