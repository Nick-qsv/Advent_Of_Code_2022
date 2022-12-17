use std::collections::HashMap;
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
    let mut vecs = HashMap::new();
    vecs.insert(1, vec!["G", "J", "Z"]);
    vecs.insert(2, vec!["Q", "L", "R", "P", "W", "F", "V", "C"]);
    vecs.insert(3, vec!["F", "P", "M", "C", "L", "G", "R"]);
    vecs.insert(4, vec!["L", "F", "B", "W", "P", "H", "M"]);
    vecs.insert(5, vec!["G", "C", "F", "S", "V", "Q"]);
    vecs.insert(6, vec!["W", "H", "J", "Z", "M", "Q", "T", "L"]);
    vecs.insert(7, vec!["H", "F", "S", "B", "V"]);
    vecs.insert(8, vec!["F", "J", "Z", "S"]);
    vecs.insert(9, vec!["M", "C", "D", "P", "F", "H", "B", "T"]);
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
            //check each string literal in vec if it can be converted to i32 push it to the "commands" vec
            match i32::from_str(num) {
                Ok(num) => num_commands.push(num),
                Err(_) => {}
            }
        }
        println!("num_commands: {:?}", num_commands);

        //get the mut vec and shift
        for j in 0..num_commands[0] {
            //length of the vec
            let length = vecs.get_mut(&num_commands[1]).unwrap().len();
            //set last element in vec to this variable
            let shifted_element = vecs.get_mut(&num_commands[1]).unwrap()[length - 1];
            //remove the variable from the vec
            vecs.get_mut(&num_commands[1]).unwrap().remove(length - 1);
            //move the variable into appropriate vec
            vecs.get_mut(&num_commands[2])
                .unwrap()
                .push(shifted_element);
            // println!("shifted element: {}", shifted_element);
        }
    }
    //print the answer
    for v in 1..10 {
        let length = vecs.get(&v).unwrap().len();
        println!(
            "YO THIS IS ANSWER{:?}",
            vecs.get_mut(&v).unwrap()[length - 1]
        );
    }
}

//Part 2
pub fn day5p2() {
    let contents = fs::read_to_string("d5input.txt").expect("Failed to read file");

    let mut vecs = HashMap::new();
    vecs.insert(1, vec!["G", "J", "Z"]);
    vecs.insert(2, vec!["Q", "L", "R", "P", "W", "F", "V", "C"]);
    vecs.insert(3, vec!["F", "P", "M", "C", "L", "G", "R"]);
    vecs.insert(4, vec!["L", "F", "B", "W", "P", "H", "M"]);
    vecs.insert(5, vec!["G", "C", "F", "S", "V", "Q"]);
    vecs.insert(6, vec!["W", "H", "J", "Z", "M", "Q", "T", "L"]);
    vecs.insert(7, vec!["H", "F", "S", "B", "V"]);
    vecs.insert(8, vec!["F", "J", "Z", "S"]);
    vecs.insert(9, vec!["M", "C", "D", "P", "F", "H", "B", "T"]);
    for line in contents.lines() {
        //make each line into a vec of &str
        let string_commands: Vec<&str> = line.split(" ").collect();
        let mut num_commands: Vec<i32> = vec![];
        //loop through the string commands
        for num in string_commands {
            //check each string literal in vec if it can be converted to i32 push it to the "commands" vec
            match i32::from_str(num) {
                Ok(num) => num_commands.push(num),
                Err(_) => {}
            }
        }
        println!("num_commands: {:?}", num_commands);

        //identify items
        let num_items = num_commands[0];
        let start_vec = num_commands[1];
        let end_vec = num_commands[2];
        //if it's a single variable we do the shift
        if num_items == 1 {
            //vec length
            let length = vecs.get(&start_vec).unwrap().len();
            //set last element in vec to this variable
            let shifted_element = vecs.get_mut(&start_vec).unwrap()[length - 1];
            //remove the variable from the vec
            vecs.get_mut(&start_vec).unwrap().remove(length - 1);
            //move the variable into appropriate vec
            vecs.get_mut(&end_vec).unwrap().push(shifted_element);
            println!("shifted element: {}", shifted_element);

        //if it's more than one we pop
        } else {
            for j in (1..=num_items).rev() {
                //vec length
                let length = vecs.get(&start_vec).unwrap().len();
                //start at vec - num_items
                let popped_element = vecs.get_mut(&start_vec).unwrap()[length - j as usize];
                //remove the variable from the vec
                vecs.get_mut(&start_vec)
                    .unwrap()
                    .remove(length - j as usize);
                //move the variable into appropriate vec
                vecs.get_mut(&end_vec).unwrap().push(popped_element);
                println!("popped element: {}", popped_element);
            }
        }
    }
    //print the answer
    for v in 1..10 {
        let length = vecs.get(&v).unwrap().len();
        println!(
            "YO THIS IS ANSWER{:?}",
            vecs.get_mut(&v).unwrap()[length - 1]
        );
    }
    println!("YO THIS IS ANSWER{:?}", vecs);
}
