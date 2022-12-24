use std::collections::HashMap;
use std::fs;

//Part 1
pub fn day8() {
    let contents = fs::read_to_string("d8input.txt").expect("Failed to read file");

    let mut tree_count: i32 = 0;
    let mut grid: Vec<Vec<i32>> = vec![];
    //make the grid
    for line in contents.lines() {
        let num_vec: Vec<i32> = line
            .chars()
            .map(|x| x.to_string().parse().expect("error parsing int"))
            .collect();
        grid.push(num_vec);
    }
    //define some variables to make it easier to read
    let row_len = grid[0].len();
    let col_len = grid.len();
    for i in 0..col_len {
        //if its first or last row then you dont need to loop
        if i == 0 || i == col_len - 1 {
            tree_count += row_len as i32;
        } else {
            //now we need to start the loop across the rows
            for j in 0..grid[i].len() {
                //check if its the end of the row or beginning
                if j == 0 || j == grid[i].len() - 1 {
                    tree_count += 1;
                } else {
                    //now we are inside the tree, need to check if its visible from all sides

                    //get the slice of i32 for the left and right
                    let right_slice = &grid[i][j..row_len];
                    let left_slice = &grid[i][0..j + 1];

                    if check_right(&right_slice) || check_left(&left_slice) {
                        tree_count += 1;
                    } else if check_above(&grid, j, i) {
                        tree_count += 1;
                    } else if check_below(&grid, j, i) {
                        tree_count += 1;
                    }
                }
            }
        }
    }
    println!("Answer: {}", tree_count);
}

//check if all the numbers to the right are less
fn check_right(numbers: &[i32]) -> bool {
    //get a slice of i32, start from 1
    for i in 1..numbers.len() {
        if numbers[i] >= numbers[0] {
            return false;
        }
    }
    true
}
//check if all the numbers to the left are less
fn check_left(numbers: &[i32]) -> bool {
    //length is equal to length of the array -1, this gives the index of the number we're checking
    let length = numbers.len() - 1;
    //starts at length - 1 because not inclusive
    for i in (0..length).rev() {
        if numbers[i] >= numbers[length] {
            return false;
        }
    }
    true
}
//check if all the numbers above are less
fn check_above(grid: &Vec<Vec<i32>>, curr_col: usize, curr_row: usize) -> bool {
    for x in (0..curr_row).rev() {
        if grid[x][curr_col] >= grid[curr_row][curr_col] {
            return false;
        }
    }
    true
}
//check if all the numbers below are less
fn check_below(grid: &Vec<Vec<i32>>, curr_col: usize, curr_row: usize) -> bool {
    for x in curr_row + 1..grid.len() {
        if grid[x][curr_col] >= grid[curr_row][curr_col] {
            return false;
        }
    }
    true
}

//Part 2
pub fn day8p2() {
    let contents = fs::read_to_string("d8input.txt").expect("Failed to read file");

    let mut largest_s_score: i32 = 0;
    let mut grid: Vec<Vec<i32>> = vec![];
    //make the grid
    for line in contents.lines() {
        let num_vec: Vec<i32> = line
            .chars()
            .map(|x| x.to_string().parse().expect("error parsing int"))
            .collect();
        grid.push(num_vec);
    }
    //define some variables to make it easier to read
    let row_len = grid[0].len();
    let col_len = grid.len();
    for i in 0..col_len {
        //if its first or last row then you dont need to loop
        if i != 0 && i != col_len - 1 {
            //now we need to start the loop across the rows
            for j in 0..grid[i].len() {
                //check if its the end of the row or beginning
                if j != 0 && j != grid[i].len() - 1 {
                    //now we are inside the tree, need get each score
                    //get the slice of i32 for the left and right
                    let right_slice = &grid[i][j..row_len];
                    // println!("rightslice:{:?}", right_slice);
                    let left_slice = &grid[i][0..j + 1];
                    // println!("leftslice:{:?}", left_slice);

                    let right = check_right_s(right_slice);
                    let left = check_left_s(left_slice);
                    let up = check_above_s(&grid, j, i);
                    let down = check_below_s(&grid, j, i);
                    let mut scenic = right * left * up * down;
                    // println!("UP:{}", up);
                    // println!("right:{}", right);
                    // println!("left:{}", left);
                    // println!("down:{}", down);

                    if scenic > largest_s_score {
                        // println!("scenic:{}", scenic);
                        largest_s_score = scenic;
                    }
                    scenic = 0;
                }
            }
        }
    }
    println!("Answer: {}", largest_s_score);
}

//get the scenic score for the right
fn check_right_s(numbers: &[i32]) -> i32 {
    //get a slice of i32, start from 1
    let mut counter = 0;
    for i in 1..numbers.len() {
        // println!("numers[i]:{}", numbers[i]);
        if numbers[i] < numbers[0] {
            counter += 1;
        } else if numbers[i] == numbers[0] {
            counter += 1;
            break;
        } else {
            break;
        }
    }
    counter
}
//get the scenic score for the left
fn check_left_s(numbers: &[i32]) -> i32 {
    //length is equal to length of the array -1, this gives the index of the number we're checking
    let mut counter = 0;
    let length = numbers.len() - 1;
    //starts at length - 1 because not inclusive
    for i in (0..length).rev() {
        if numbers[i] < numbers[length] {
            counter += 1;
        } else if numbers[i] == numbers[length] {
            counter += 1;
            break;
        } else {
            break;
        }
    }
    counter
}
//get the scenic score for above
fn check_above_s(grid: &Vec<Vec<i32>>, curr_col: usize, curr_row: usize) -> i32 {
    let mut counter = 0;
    for x in (0..curr_row).rev() {
        if grid[x][curr_col] < grid[curr_row][curr_col] {
            counter += 1;
        } else if grid[x][curr_col] == grid[curr_row][curr_col] {
            counter += 1;
            break;
        } else {
            break;
        }
    }
    counter
}
//get the scenic score for below
fn check_below_s(grid: &Vec<Vec<i32>>, curr_col: usize, curr_row: usize) -> i32 {
    let mut counter = 0;
    for x in curr_row + 1..grid.len() {
        if grid[x][curr_col] < grid[curr_row][curr_col] {
            counter += 1;
        } else if grid[x][curr_col] == grid[curr_row][curr_col] {
            counter += 1;
            break;
        } else {
            break;
        }
    }
    counter
}
