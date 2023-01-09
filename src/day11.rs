// use debug_fn::DebugFn;
use std::fmt::{self, Formatter};
use std::fs;
use std::iter::Iterator;
use std::process::id;

struct Monkey {
    identifier: i32,
    starting_items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(f64) -> bool>,
    t_t: Option<i32>,
    f_t: Option<i32>,
}

impl Monkey {
    fn new(
        identifier: i32,
        starting_items: Vec<i32>,
        operation: Box<dyn Fn(i32) -> i32>,
        test: Box<dyn Fn(f64) -> bool>,
        t_t: Option<i32>,
        f_t: Option<i32>,
    ) -> Self {
        Self {
            identifier,
            starting_items,
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
            "Monkey {{ identifier: {}, starting_items: {:?}, operation: <closure>, test: <closure>, t_t: {:?}, f_t: {:?} }}",
            self.identifier,
            self.starting_items,
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
        // println!("vec_words: {:?}", fin_v_words);
        //make sure vec_words isn't empty
        if fin_v_words.len() > 0 {
            //get statement identifier
            let identity = &fin_v_words[0];
            //for the throws
            let throw = &fin_v_words[1];
            if identity == "Monkey" {
                let identifier: i32 = fin_v_words[1].parse().expect("couldnt unwrap iden");
                //create new monkey
                let monkey = Monkey::new(
                    identifier,
                    vec![],
                    Box::new(|x| x),
                    Box::new(|x| x > 0.0),
                    None,
                    None,
                );
                monkey_vec.push(monkey);
            } else if identity == "Starting" {
                //get the last index of the monkey array (current monkey)
                let last_idx = monkey_vec.len() - 1;
                let curr_monkey = &mut monkey_vec[last_idx];
                //iterate over each word and see if it is an i32
                for word in fin_v_words {
                    if let Ok(num) = word.parse::<i32>() {
                        curr_monkey.starting_items.push(num);
                    }
                }
            } else if identity == "Operation" {
                let last_idx = monkey_vec.len() - 1;
                let curr_monkey = &mut monkey_vec[last_idx];
                //the indexes are the same
                //so define a few variables
                //vw[3] always equals old
                //vw[4] == operation
                //vw[5] == number or x
                //this is just used for a match statement of +,-,*,/
                let operand = vec_words[4];
                //this can be checked if its old, if its old then it is equal to x
                let var = vec_words[5];
                match operand {
                    "*" => {
                        if let Ok(num) = var.parse::<i32>() {
                            curr_monkey.operation = Box::new(move |x| x * num);
                        } else {
                            curr_monkey.operation = Box::new(|x| x * x);
                        }
                    }
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
                // if let Some(op) = formula {
                //     curr_monkey.operation = op;
                // }
            } else if identity == "Test" {
                let last_idx = monkey_vec.len() - 1;
                let curr_monkey = &mut monkey_vec[last_idx];
                if let Ok(num) = fin_v_words[3].parse::<f64>() {
                    curr_monkey.test = Box::new(move |x| x % num == 0.0);
                }
            } else if throw == "true" {
                let last_idx = monkey_vec.len() - 1;
                let curr_monkey = &mut monkey_vec[last_idx];
                if let Ok(id) = fin_v_words[5].parse::<i32>() {
                    curr_monkey.t_t = Some(id);
                }
            } else if throw == "false" {
                let last_idx = monkey_vec.len() - 1;
                let curr_monkey = &mut monkey_vec[last_idx];
                if let Ok(id) = fin_v_words[5].parse::<i32>() {
                    curr_monkey.f_t = Some(id);
                }
            }
        }
    }
    //now we have the monkeys set up (I THINK)
    //so need to do the rounds
    //When a monkey throws an item to another monkey, the item goes on the end of the recipient monkey's list. A monkey that starts a round with no items could end up inspecting and throwing many items by the time its turn comes around. If a monkey is holding no items at the start of its turn, its turn ends.

    //to avoid copying the monkey_vec, need to make a new data structure that will have the output
    //maybe instead of having starting items inside the monkey Struct can have it in a standalone hashmap?
    //so start by looping over each monkey in the vec
    //monkey 0 with a hashmap of 1
    //loop through the starting items, youre inside a monkey within monkey_vec and the starting items hashmap
    //get the data item and manipulate it using the monkey operation
    //get the data item and manipulate
    //loop through each monkey
    for mut monkey in monkey_vec {
        //check if starting items is empty
        if monkey.starting_items.len() > 0 {
            //loop through the starting items
            for mut item in monkey.starting_items {
                item = (monkey.operation)(item);
                let mut new_item = 0.0;
                new_item = item as f64 / 3.0;
                new_item = new_item.floor();
                if (monkey.test)(new_item) {
                    //so if the test is true
                    //need to convert new_item to i32
                    //then either find or loop through all the monkeys
                    //get identifier
                    let monkey_id = monkey.t_t.expect("t_t error");
                    // monkey_vec[monkey_id as usize]
                    //     .starting_items
                    //     .push(new_item as i32);
                } else {
                    let monkey_id = monkey.f_t.expect("f_t error");
                }
            }
        }
    }
    // println!("monkey vec: {:?}", monkey_vec);
}
