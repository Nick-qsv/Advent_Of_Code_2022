use std::collections::HashMap;
use std::fs;

//Part 1
pub fn day8() {
    let contents = fs::read_to_string("d8input.txt").expect("Failed to read file");

    let mut tree_count: i32 = 0;
    let mut grid: Vec<Vec<i32>> = vec![];
    for line in contents.lines() {
        //feel like a hashmap is the right thing for this
        //get it into a vec of i32
        //ordered vec of i32 for each line
        //then on each index, if not an edge
        //check left and right first
        //then check up and down
        //if left or right doesn't exist then it's an edge and return yes
        //keep a count
        //seems simple enough.  not sure how efficient it will run
        //hashmap pretty efficient i think though

        //hashmap not good.  need a grid or vec of vecs
        let num_vec: Vec<i32> = line
            .chars()
            .map(|x| x.to_string().parse().expect("error parsing int"))
            .collect();
        grid.push(num_vec);
    }
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
                    //check if all the numbers to the right are less
                    //get the slice of i32 for the right
                    let right_slice = &grid[i][j..row_len];
                    //get the slice of i32 for the left
                    let left_slice = &grid[i][0..j + 1];
                    println!("left slice: {:?}", left_slice);
                    if check_right(&right_slice) || check_left(&left_slice) {
                        tree_count += 1;
                    }
                }
            }
        }
    }

    // println!("Hashmap:{:?}", grid);
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
    let length = numbers.len() - 2;
    for i in (0..length).rev() {
        if numbers[i] >= numbers[length + 1] {
            return false;
        }
    }
    true
}
//check if all the numbers above are less

//check if all the numbers below are less
