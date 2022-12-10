use std::fs;

//Part 1

pub fn day1() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut max: i32 = 0;
    let mut current: i32 = 0;
    for line in contents.lines() {
        if line.is_empty() {
            if current > max {
                max = current
            }
            current = 0;
        } else {
            let num: i32 = line.parse().expect("not an integer");
            current += num;
        }
    }
    println!("Answer: {}", max)
}

//Part 2

pub fn day1p2() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut current: i32 = 0;
    let mut total_cals: Vec<i32> = vec![];
    for line in contents.lines() {
        if !line.is_empty() {
            let x: i32 = line.parse().expect("not an integer");
            current += x;
        } else {
            total_cals.push(current);
            current = 0;
        }
    }
    total_cals.sort_by_key(|x| -x);
    let top_3: i32 = total_cals[0] + total_cals[1] + total_cals[2];
    println!("Answer: {}", top_3)
}
