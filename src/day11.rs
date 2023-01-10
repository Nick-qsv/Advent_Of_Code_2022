// use debug_fn::DebugFn;
use std::collections::HashMap;
use std::fmt::{self, Formatter};
use std::fs;
use std::process::id;

struct Monkey {
    identifier: i32,
    operation: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(f64) -> bool>,
    t_t: Option<i32>,
    f_t: Option<i32>,
}

impl Monkey {
    fn new(
        identifier: i32,
        operation: Box<dyn Fn(i32) -> i32>,
        test: Box<dyn Fn(f64) -> bool>,
        t_t: Option<i32>,
        f_t: Option<i32>,
    ) -> Self {
        Self {
            identifier,
            operation,
            test,
            t_t,
            f_t,
        }
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Monkey {{ identifier: {}, operation: <closure>, test: <closure>, t_t: {:?}, f_t: {:?} }}",
            self.identifier,
            self.t_t,
            self.f_t
        )
    }
}
//Part 1
pub fn day11() {
    let contents = fs::read_to_string("d11input.txt").expect("Failed to read file");

    //make a vec of monkeys to iterate through later
    let mut monkey_vec: Vec<Monkey> = vec![];

    //make a hashmap of the starting items to manipulate later
    let mut item_map: HashMap<i32, Vec<i32>> = HashMap::new();

    //make a hashmap of the total inspections
    let mut inspection_map: HashMap<i32, i32> = HashMap::new();

    //create the vec of monkeys
    for line in contents.lines() {
        //collect each word in each line into a string
        let mut vec_words: Vec<&str> = line.split(" ").collect();

        //remove the entries created by spaces
        vec_words.retain(|s| *s != "");

        //convert to string vec so can mutate it easier
        let filt_vec_words: Vec<String> = vec_words.iter().map(|x| x.to_string()).collect();

        //remove ":" and "," char from each
        let fin_v_words: Vec<String> = filt_vec_words
            .iter()
            .map(|x| x.replace(":", "").replace(",", ""))
            .collect();

        //make sure vec_words isn't empty
        if fin_v_words.len() > 0 {
            //get statement identifier
            let identity = &fin_v_words[0];

            //for the throws from one monkey to another
            let throw = &fin_v_words[1];

            //match the statement
            if identity == "Monkey" {
                let identifier: i32 = fin_v_words[1].parse().expect("couldnt unwrap iden");
                //create new monkey and add it to the monkey_vec
                let monkey = Monkey::new(
                    identifier,
                    Box::new(|x| x),
                    Box::new(|x| x > 0.0),
                    None,
                    None,
                );
                monkey_vec.push(monkey);

                //initialize the inspection hashmap
                inspection_map.insert(identifier, 0);
            } else if identity == "Starting" {
                //define the vec thats gonna be pushed into the HashMap
                let mut start_vec: Vec<i32> = vec![];

                //iterate over each word and see if it is an i32
                for word in fin_v_words {
                    if let Ok(num) = word.parse::<i32>() {
                        start_vec.push(num);
                    }
                }

                //find the identifier
                let last_idx = monkey_vec.len() - 1;
                let curr_monkey = &mut monkey_vec[last_idx];
                let id = curr_monkey.identifier;

                //insert the start_vec into the hash_map
                item_map.insert(id, start_vec);
            } else if identity == "Operation" {
                //find the identifier
                let last_idx = monkey_vec.len() - 1;
                let curr_monkey = &mut monkey_vec[last_idx];

                //this is used for a match statement of +,-,*,/
                let operand = vec_words[4];

                //var is either "old" or an i32, if its old then it is equal to x
                let var = vec_words[5];

                match operand {
                    "*" => {
                        //if it is not == to old
                        if let Ok(num) = var.parse::<i32>() {
                            //define the operation as multiplication with num as the number
                            curr_monkey.operation = Box::new(move |x| x * num);
                        } else {
                            //definet the operation as multiplication by itself
                            curr_monkey.operation = Box::new(|x| x * x);
                        }
                    }
                    //same logic for the remaining operands
                    "+" => {
                        if let Ok(num) = var.parse::<i32>() {
                            curr_monkey.operation = Box::new(move |x| x + num);
                        } else {
                            curr_monkey.operation = Box::new(|x| x + x);
                        }
                    }
                    "-" => {
                        if let Ok(num) = var.parse::<i32>() {
                            curr_monkey.operation = Box::new(move |x| x - num);
                        } else {
                            curr_monkey.operation = Box::new(|x| x - x);
                        }
                    }
                    "/" => {
                        if let Ok(num) = var.parse::<i32>() {
                            curr_monkey.operation = Box::new(move |x| x / num);
                        } else {
                            curr_monkey.operation = Box::new(|x| x / x);
                        }
                    }
                    _ => {}
                }
            } else if identity == "Test" {
                //get identifier
                let last_idx = monkey_vec.len() - 1;
                let curr_monkey = &mut monkey_vec[last_idx];

                //define the boolean test function checking if remainder is 0
                if let Ok(num) = fin_v_words[3].parse::<f64>() {
                    curr_monkey.test = Box::new(move |x| x % num == 0.0);
                }
            } else if throw == "true" {
                let last_idx = monkey_vec.len() - 1;
                let curr_monkey = &mut monkey_vec[last_idx];

                //give the id of the target monkey if true
                if let Ok(id) = fin_v_words[5].parse::<i32>() {
                    curr_monkey.t_t = Some(id);
                }
            } else if throw == "false" {
                let last_idx = monkey_vec.len() - 1;
                let curr_monkey = &mut monkey_vec[last_idx];

                //give the id of the target monkey if false
                if let Ok(id) = fin_v_words[5].parse::<i32>() {
                    curr_monkey.f_t = Some(id);
                }
            }
        }
    }

    //20 rounds
    for _ in 0..20 {
        //loop through each monkey
        //cant use implicit into iter so need to use 0..x
        let m_v_len = monkey_vec.len();
        for m in 0..m_v_len {
            //define the current monkey
            let monkey = &monkey_vec[m];

            //get the id
            let id = monkey.identifier;

            //get length of items
            let vec_len = item_map.get(&id).expect("Error unwrapping itemvec").len();

            //check if starting items is empty
            if vec_len > 0 {
                //loop through the starting items
                //have to use a for 0..x loop so you don't borrow item_map twice
                for _ in 0..vec_len {
                    //because youre removing the leftmost item each time, index is always 0
                    let mut item = item_map.get(&id).unwrap()[0];

                    //run the current monkey operation on the i32
                    item = (monkey.operation)(item);

                    //make a new f64 value to representing the current worry level
                    let mut new_item = item as f64 / 3.0;
                    new_item = new_item.floor();

                    //if the test is true
                    if (monkey.test)(new_item) {
                        //get identifier
                        let t_monkey_id = monkey.t_t.expect("t_t error");

                        //convert new_item to an i32
                        let i_item = new_item as i32;
                        item_map.get_mut(&t_monkey_id).unwrap().push(i_item);
                        item_map.get_mut(&id).expect("errrrrorrmap").remove(0);

                        //add 1 to the inspection counter
                        if let Some(x) = inspection_map.get_mut(&id) {
                            *x += 1;
                        }
                        //if the test is false
                    } else {
                        //get identifier
                        let t_monkey_id = monkey.f_t.expect("f_t error");

                        //convert new_item to an i32
                        let i_item = new_item as i32;
                        item_map.get_mut(&t_monkey_id).unwrap().push(i_item);
                        item_map.get_mut(&id).expect("errrrrorrmap").remove(0);

                        //add 1 to the inspection counter
                        if let Some(x) = inspection_map.get_mut(&id) {
                            *x += 1;
                        }
                    }
                }
            }
        }
    }
    // println!("monkey vec: {:?}", monkey_vec);
    print!("Twenty Rounds: {:?}", item_map);
    print!("inspection map: {:?}", inspection_map);
    let answer = 312 * 306;
    print!("Answer: {}", answer);
}
