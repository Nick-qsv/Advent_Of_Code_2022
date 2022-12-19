use std::collections::HashSet;
use std::fs;

//Part 1
pub fn day6() {
    let contents = fs::read_to_string("d6input.txt").expect("Failed to read file");

    //can use a hashset
    //and a for loop i guess?
    //when length of hashset == 4 then take the index??
    //no because you need to be removing values from the hashset..
    //can make a new hashset every loop
    //i'm thinking this: make a vec of all the chars
    //loop through every range of index, collecting those values into a hashset
    //if the hashset ==4 then you take the end range and + 1 and return that.
    //answer
    let mut answer: usize = 0;
    //Vector of chars
    let string_vec: Vec<char> = contents.chars().collect();
    let len = string_vec.len();
    //Hashset used to check
    let mut set: HashSet<char> = HashSet::new();
    //Now for the loop
    for initial in 0..=len {
        let end = initial + 3;

        //insert the range in the vec into the hashset
        set.extend(&string_vec[initial..=end]);

        //hashest is only unique chars so check the length
        if set.len() == 4 {
            answer = end + 1;
            break;
        }
        //clear hashset
        set.clear();
    }

    println!("Answer: {:?}", answer);
}

//Part 2
pub fn day6p2() {
    let contents = fs::read_to_string("d6input.txt").expect("Failed to read file");
    let mut answer: usize = 0;
    //Vector of chars
    let string_vec: Vec<char> = contents.chars().collect();
    let len = string_vec.len();
    //Hashset used to check
    let mut set: HashSet<char> = HashSet::new();
    //Now for the loop
    for initial in 0..=len {
        //for part 2 just change end variable val
        let end = initial + 13;
        //insert the range in the vec into the hashset
        set.extend(&string_vec[initial..=end]);
        //and set length
        if set.len() == 14 {
            answer = end + 1;
            break;
        }
        set.clear();
    }
    println!("Answer: {:?}", answer);
}
