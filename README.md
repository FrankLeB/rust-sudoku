# rust-sudoku

This is an implementation of backtracking in rust for solving a sudoku. The input parsing is pretty straightforward,
the program assumes you will feed it the board as 9 lines with 9 numbers each. A 0 means a value to fill.

To run the program, simply use the following command:
`cargo run < input.in`

To run with your own custom sudoku, fill in the sudoku in an input file following the format in the input.in or
input_hard.in examples and use the following command:
`cargo run < your_sudoku_file.in`
