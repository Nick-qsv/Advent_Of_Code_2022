// use debug_fn::DebugFn;
use std::fmt::{self, Formatter};
use std::fs;
use std::process::id;

struct Monkey {
    identifier: i32,
    starting_items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> bool>,
    t_t: Option<i32>,
    f_t: Option<i32>,
}

impl Monkey {
    fn new(
        identifier: i32,
        starting_items: Vec<i32>,
        operation: Box<dyn Fn(i32) -> i32>,
        test: Box<dyn Fn(i32) -> bool>,
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
        let m_len = monkey_vec.len();
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
                    Box::new(|x| x > 0),
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
                if let Ok(num) = fin_v_words[3].parse::<i32>() {
                    curr_monkey.test = Box::new(move |x| x % num == 0);
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
    println!("monkey vec: {:?}", monkey_vec);
}
