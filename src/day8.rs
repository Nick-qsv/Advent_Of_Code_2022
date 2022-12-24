use std::collections::HashMap;
use std::fs;

//Part 1
pub fn day8() {
    let contents = fs::read_to_string("d8input.txt").expect("Failed to read file");

    let mut tree_count: i32 = 0;
    let mut hash_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut key: i32 = 1;
    for line in contents.lines() {
        //feel like a hashmap is the right thing for this
        //get it into a vec of i32
        //ordered vec of i32 for each line
        //then on each index, if not an edge
        //check left and right first
        //then check up and down
        //if left or right doesn't exist then it's an edge and return yes
        //keep a count
        //seems simple enough.  not sure how efficient it will run
        //hashmap pretty efficient i think though
        let num_vec: Vec<i32> = line
            .chars()
            .map(|x| x.to_string().parse().expect("error parsing int"))
            .collect();
        hash_map.insert(key, num_vec);
        key += 1;
    }
    //get # key value pairs in hashmap
    let kvp = hash_map.len() as i32;
    //iterate through each line
    for j in 1..=kvp {
        //get length of the vec
        let vec_len = hash_map.get_mut(&j).unwrap().len();
        //check if it is the first row or last row
        if j == 1 || j == kvp {
            tree_count += vec_len as i32;
        } else {
            //loop over each item in the vec
            for i in 0..vec_len {
                println!("i: {}", i);
            }
        }
    }
    // println!("Hashmap:{:?}", hash_map);
    println!("Answer: {}", tree_count);
}
