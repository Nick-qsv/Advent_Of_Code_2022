use std::fs;

use std::collections::HashSet;
use std::hash::Hash;

//Part 1
pub fn day3() {
    let mut current: i32 = 0;
    let contents = fs::read_to_string("d3input.txt").expect("Failed to read file");
    for line in contents.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        // let common = first.chars().zip(second.chars()).find(|(c1, c2)| c1 == c2);
        let set1: HashSet<char> = first.chars().collect();
        let set2: HashSet<char> = second.chars().collect();
        let common = set1.iter().find(|c| set2.contains(c));
        println!("common: {:?}", common);
        match common {
            Some(c) => {
                if c.is_lowercase() {
                    let val = *c as u8 - b'a' + 1;
                    current += val as i32;
                    println!("lowercase Value: {}", val);
                } else {
                    let value = *c as u8 - b'A' + 27;
                    println!("Uppercase Value: {}", value);
                    current += value as i32;
                }
            }
            None => {}
        }
    }
    println!("Current: {}", current)
}
