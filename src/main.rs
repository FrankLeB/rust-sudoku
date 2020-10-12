use std::io;

fn main() {
    let mut board: Vec<Vec<usize>> = Vec::new();
    
    // Parse input
    for _ in 0..9 {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read data");

        let row: Vec<usize> = 
            buffer.split(' ')
                  .map(|s| s.trim())
                  .filter(|s| !s.is_empty())
                  .map(|s| s.parse().unwrap())
                  .collect();
        board.push(row);
    }

    // Print initial board
    println!("Before");
    print_board(&board);
    
    // Solve
    let result = solve(&mut board, 0, 0);
    if result {
        println!("After");
        print_board(&board);
    } else {
        println!("Unsolvable");
    }
}

fn print_board(v: &Vec<Vec<usize>>) {
    for (i, row) in v.iter().enumerate() {
        for (j, digit) in row.iter().enumerate() {
            if *digit == 0 {
                print!("_");
            } else {
                print!("{}", digit);
            }
            print!(" ");

            if (j + 1) % 3 == 0 {
                print!("   ")
            }
        }
        print!("\n");

        if (i + 1) % 3 == 0 {
            print!("\n");
        }
    }
}

fn solve(v: &mut Vec<Vec<usize>>, r: usize, c: usize) -> bool {
    // End condition
    if r == 9 && c == 0 {
        return is_solved(&v);
    }

    // next position
    let next_c = if c < 8 { c + 1 } else { 0 };
    let next_r = if next_c == 0 { r + 1 } else { r };

    // If it's a given number, we don't have to do anything
    let initial_value = v[r][c];
    if initial_value != 0 {
        return solve(v, next_r, next_c);
    }

    // Check which numbers are allowed at that position
    let present_flag = check_present_digits(&v, r, c);
    for (i, present) in present_flag.iter().enumerate() {
        if !present {
            // Recursion
            v[r][c] = i + 1;
            let result = solve(v, next_r, next_c);
            if result {
                return true;
            }
        }
    }

    v[r][c] = initial_value;
    false
}

fn check_present_digits(v: &Vec<Vec<usize>>, r: usize, c: usize) -> [bool; 9] {
    let mut present_flag: [bool; 9] = [false; 9];

    // check rows and columns
    for i in 0..9 {
        if v[r][i] != 0 {
            present_flag[v[r][i] - 1] = true;
        }

        if v[i][c] != 0 {
            present_flag[v[i][c] - 1] = true;
        }
    }

    // Check the box that the current position is in
    let start_box_x = if r < 3 { 0 } else if r < 6 { 3 } else { 6 };
    let start_box_y = if c < 3 { 0 } else if c < 6 { 3 } else { 6 };
    for i in start_box_x..start_box_x + 3 {
        for j in start_box_y..start_box_y + 3 {
            if v[i][j] != 0 {
                present_flag[v[i][j] - 1] = true;
            }
        }
    }

    present_flag
}

fn is_solved(v: &Vec<Vec<usize>>) -> bool {
    // Check each rows and columns
    for i in 0..v.len() {
        let mut row_sum = 0;
        let mut col_sum = 0;
        for j in 0..v.len() {
            row_sum += v[j][i];
            col_sum += v[i][j];
        }

        if row_sum != 45 || col_sum != 45 {
            return false;
        }
    }


    // Check each small squares
    let mut sums = [0; 9];
    for i in 0..3 {
        for j in 0..3 {
            sums[0] += v[i][j];
            sums[1] += v[i][j + 3];
            sums[2] += v[i][j + 6];

            sums[3] += v[i + 3][j];
            sums[4] += v[i + 3][j + 3];
            sums[5] += v[i + 3][j + 6];

            sums[6] += v[i + 6][j];
            sums[7] += v[i + 6][j + 3];
            sums[8] += v[i + 6][j + 6];
        }
    }
    for s in sums.iter() {
        if *s != 45 {
            return false;
        }
    }

    true
}
