use std::fs;

//Part 1
pub fn day2() {
    let mut current: i32 = 0;
    let mut tuples = Vec::new();

    let contents = fs::read_to_string("d2input.txt").expect("Failed to read file");
    println!("Contents: {:?}", contents);
    for line in contents.lines() {
        let (c1, c2) = line.split_at(1);
        tuples.push((c1, c2));
    }
    for (c1, c2) in tuples {
        match (c1, c2) {
            ("A", " X") => current += (3 + 1),
            ("A", " Y") => current += (6 + 2),
            ("A", " Z") => current += 3,
            ("B", " X") => current += (1),
            ("B", " Y") => current += (3 + 2),
            ("B", " Z") => current += (6 + 3),
            ("C", " X") => current += (6 + 1),
            ("C", " Y") => current += (2),
            ("C", " Z") => current += (3 + 3),
            _ => {}
        }
    }
    println!("Answer: {}", current)
}

//Part 2
pub fn day2p2() {
    let mut current: i32 = 0;
    let mut tuples = Vec::new();

    let contents = fs::read_to_string("d2input.txt").expect("Failed to read file");
    for line in contents.lines() {
        let (c1, c2) = line.split_at(1);
        tuples.push((c1, c2));
    }
    for (c1, c2) in tuples {
        match (c1, c2) {
            ("A", " X") => current += (3),
            ("A", " Y") => current += (1 + 3),
            ("A", " Z") => current += (2 + 6),
            ("B", " X") => current += (1),
            ("B", " Y") => current += (3 + 2),
            ("B", " Z") => current += (6 + 3),
            ("C", " X") => current += (2),
            ("C", " Y") => current += (3 + 3),
            ("C", " Z") => current += (1 + 6),
            _ => {}
        }
    }
    println!("Answer: {}", current)
}
