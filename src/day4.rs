use std::collections::HashSet;
use std::fs;

//Part 1
pub fn day4() {
    let contents = fs::read_to_string("d4input.txt").expect("Failed to read file");

    let mut counter: i32 = 0;
    for line in contents.lines() {
        //make each line into a vec of ["2-4","4-6"]
        let sub: Vec<&str> = line.split(",").collect();

        //convert into a vec of 4 i32 values [2,4,4,6]
        let mut nums = Vec::new();
        for substring in sub {
            //this makes it into ["2","4","4","6"]
            let parts: Vec<&str> = substring.split('-').collect();
            let start: i32 = parts[0].parse().unwrap();
            let end: i32 = parts[1].parse().unwrap();
            nums.push(start);
            nums.push(end);
        }
        //push these numbers into hashsets to easily compare
        let mut set1: HashSet<i32> = HashSet::new();
        let mut set2: HashSet<i32> = HashSet::new();
        for i in nums[0]..=nums[1] {
            set1.insert(i);
        }
        for j in nums[2]..=nums[3] {
            set2.insert(j);
        }
        //If either is a subset add one to the counter
        if set1.is_subset(&set2) || set2.is_subset(&set1) {
            counter += 1;
        }
    }
    println!("Answer: {}", counter);
}
