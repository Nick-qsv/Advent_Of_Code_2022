use std::fs;

pub fn day2() {
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

//so we have to go through and keep track of the top 3
//so its the same thing but the if is more complex
//so you can add up all of the sums and put them into a tuple array
//[(total,index)]
//then sort the tuple array by total top to bottom
//Feel like the tuple array will come in handy in the future, good way to organize the data
