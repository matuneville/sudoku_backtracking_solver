# Sudoku: getting solution with backtracking algorithm

## Introduction

This project aims to solve Sudoku puzzles using `Rust` programming language. Sudoku is a popular logic-based number puzzle where the goal is to fill a 9x9 grid with digits so that each column, each row, and each of the nine 3x3 subgrids that compose the grid (also called "boxes", "blocks", "regions", or "sub-squares") contains all of the digits from 1 to 9. This program implements a backtracking algorithm solution to efficiently solve Sudoku puzzles.

## The Problem of Brute Force

One way to solve a Sudoku puzzle is to use a brute-force approach, where you try every possible number in every empty cell until you find a valid solution. While this method is technically correct, it's highly inefficient. Sudoku puzzles have a vast number of possible combinations, and trying every possibility consumes a significant amount of time and computational resources, making it impractical for larger puzzles or real-time scenarios.  

## Backtracking solution

Backtracking offers a more efficient approach to solving Sudoku (compared to brute force). It systematically explores potential solutions, eliminating invalid choices as it progresses. The method involves placing numbers in empty cells and backtracking if a solution becomes invalid, allowing it to refine choices until a valid solution is reached or all options are tried.

Backtracking is particularly effective for Sudoku due to its utilization of puzzle structure and constraints. By making decisions and discarding no viable paths, the algorithm significantly reduces the search space, leading to faster solution times. Thus, execution time will be considerably efficient.

## Example


