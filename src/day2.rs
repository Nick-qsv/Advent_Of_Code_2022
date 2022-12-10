use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2() {
    let contents = File::open("d2input.txt").expect("Failed to read file");

    let reader = BufReader::new(contents);
    println!("contents: {:?}", reader);

    let mut current: i32 = 0;
    let mut tuples = Vec::new();
    for line in reader.lines() {
        let line = line.expect("failed to read");
        let mut chars = line.chars();
        let c1 = chars.next().expect("Line too short");
        let c2 = chars.next().expect("line too short");
        tuples.push((c1, c2));
    }
    println!("This is Tuples: {:?}", tuples);
    for (c1, c2) in tuples {
        match (c1, c2) {
            ('A', 'X') => current += (3 + 3),
            ('A', 'Y') => current += (6 + 2),
            ('A', 'Z') => current += 1,
            ('B', 'X') => current += (3),
            ('B', 'Y') => current += (3 + 2),
            ('B', 'Z') => current += (6 + 1),
            ('C', 'X') => current += (6 + 3),
            ('C', 'Y') => current += (2),
            ('C', 'Z') => current += (3 + 1),
            _ => println!("no match found"),
        }
    }
    println!("Answer: {}", current)
}
