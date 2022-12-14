use std::fs;
use std::str::FromStr;

//Part 1
pub fn day5() {
    let contents = fs::read_to_string("d5input.txt").expect("Failed to read file");

    //I don't know how to do this one through reading this text file for the main input
    //There's probably a complex way.  If each string was transposed it would be way easier
    //but because i'm reading left to right it's so much harder
    //     [C]             [L]         [T]
    //     [V] [R] [M]     [T]         [B]
    //     [F] [G] [H] [Q] [Q]         [H]
    //     [W] [L] [P] [V] [M] [V]     [F]
    //     [P] [C] [W] [S] [Z] [B] [S] [P]
    // [G] [R] [M] [B] [F] [J] [S] [Z] [D]
    // [J] [L] [P] [F] [C] [H] [F] [J] [C]
    // [Z] [Q] [F] [L] [G] [W] [H] [F] [M]
    //  1   2   3   4   5   6   7   8   9
    let mut one = vec!["G", "J", "Z"];
    let mut two = vec!["Q", "L", "R", "P", "W", "F", "V", "C"];
    let mut three = vec!["F", "P", "M", "C", "L", "G", "R"];
    let mut four = vec!["L", "F", "B", "W", "P", "H", "M"];
    let mut five = vec!["G", "C", "F", "S", "V", "Q"];
    let mut six = vec!["W", "H", "J", "Z", "M", "Q", "T", "L"];
    let mut seven = vec!["H", "F", "S", "B", "V"];
    let mut eight = vec!["F", "J", "Z", "S"];
    let mut nine = vec!["M", "C", "D", "P", "F", "H", "B", "T"];

    for line in contents.lines() {
        //so we have this format:
        //move 1 from 5 to 6
        //# of items, array to take from, array to move into
        //extract each number from this line
        //make each line into a vec of &str
        let string_commands: Vec<&str> = line.split(" ").collect();
        let mut num_commands: Vec<i32> = vec![];
        //loop through the string commands
        for num in string_commands {
            match i32::from_str(num) {
                Ok(num) => num_commands.push(num),
                Err(_) => {}
            }
        }
        println!("numcommandstest:{:?}", num_commands);
    }

    println!(
        "Answer: {}{}{}{}{}{}{}{}{}",
        one[0], two[0], three[0], four[0], five[0], six[0], seven[0], eight[0], nine[0]
    )
}
