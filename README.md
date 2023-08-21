# Sudoku: getting solution with backtracking algorithm

## Introduction

This project aims to solve Sudoku puzzles using `Rust` programming language. Sudoku is a popular logic-based number puzzle where the goal is to fill a 9x9 grid with digits so that each column, each row, and each of the nine 3x3 subgrids that compose the grid (also called "boxes", "blocks", "regions", or "sub-squares") contains all of the digits from 1 to 9. This program implements a backtracking algorithm solution to efficiently solve Sudoku puzzles.

## The Problem of Brute Force 

One way to solve a Sudoku puzzle is to use a brute-force approach, where you try every possible number in every empty cell until you find a valid solution. While this method is technically correct, it's highly inefficient. Sudoku puzzles have a vast number of possible combinations, and trying every possibility consumes a significant amount of time and computational resources, making it impractical for larger puzzles or real-time scenarios.  

## Backtracking solution

Backtracking offers a more efficient approach to solving Sudoku (compared to brute force). It systematically explores potential solutions, eliminating invalid choices as it progresses. The method involves placing numbers in empty cells and backtracking if a solution becomes invalid, allowing it to refine choices until a valid solution is reached or all options are tried.

Backtracking is particularly effective for Sudoku due to its utilization of puzzle structure and constraints. By making decisions and discarding no viable paths, the algorithm significantly reduces the search space, leading to faster solution times. Thus, execution time will be considerably efficient.


## Algorithm

The main solving algorithm is the following:

```rust
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
```

What is does is:
- checks for base case: if there are no empty cells, then the board is solved, and *True* is returned
- for every possible number to place in an empty cell, it creates a new recursive branch; if it wasn't a solution, *False* is returned and backtracks, otherwise it will finally reach the base case

## Example

Here we have a board with 50 blank spaces to fill:

```rust
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
```

Output:

```
[8, 9, 1, 4, 6, 3, 7, 5, 2]
[5, 7, 3, 8, 1, 2, 4, 6, 9]
[4, 6, 2, 9, 5, 7, 1, 3, 8]
[6, 8, 4, 3, 2, 1, 9, 7, 5]
[2, 1, 5, 7, 9, 4, 6, 8, 3]
[7, 3, 9, 5, 8, 6, 2, 1, 4]
[3, 5, 6, 1, 4, 9, 8, 2, 7]
[1, 4, 7, 2, 3, 8, 5, 9, 6]
[9, 2, 8, 6, 7, 5, 3, 4, 1]

Execution time in seconds: 0
Execution time in milliseconds: 0
Execution time in microseconds: 636
```

## How to use

The project includes the source and the executable files for `Rust`.  

In the main code, you can create a new instance of Sudoku, which requires a $9 \times 9$ square grid (`Vec<Vec<u8>>` type). A test function is already written to be run. 

## Complexity

