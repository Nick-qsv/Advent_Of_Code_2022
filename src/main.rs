use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");

    //So i need to go through each line
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
