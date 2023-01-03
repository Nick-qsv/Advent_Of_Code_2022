use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Monkey {
    identifier: i32,
    starting_items: Vec<i32>,
    operation: fn(i32) -> i32,
    test: fn(i32) -> bool,
}

impl Monkey {
    fn new(
        identifier: i32,
        starting_items: Vec<i32>,
        operation: fn(i32) -> i32,
        test: fn(i32) -> bool,
    ) -> Self {
        Self {
            identifier,
            starting_items,
            operation,
            test,
        }
    }
}
//Part 1
pub fn day11() {
    let contents = fs::read_to_string("d11input.txt").expect("Failed to read file");
    //gotta parse this string
    //     Monkey 0:
    //make each monkey into a struct? I think that is easier...
    //then do the operations on each... rather than going line by line and trying to do each operation
    //   Starting items: 79, 98
    //this should be a hashset that way can always push it into the monkey and if dupe not included
    //but possibility for dupes...?
    //for loop is duration of .len() of the set or w/e
    //   Operation: new = old * 19
    //now inside the for loop?
    //currval = currval*19?
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3

    //     Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    //define vec of monkeys
    let mut monkey_vec: Vec<Monkey> = vec![];

    for line in contents.lines() {
        //collect each word in each line into a string
        let mut vec_words: Vec<&str> = line.split(" ").collect();
        //remove the entries created by spaces
        vec_words.retain(|s| *s != "");
        //convert to string vec so can mutate it easier
        let filt_vec_words: Vec<String> = vec_words.iter().map(|x| x.to_string()).collect();
        //remove ":" entry from each
        let fin_v_words: Vec<String> = filt_vec_words
            .iter()
            .map(|x| x.replace(":", "").replace(",", ""))
            .collect();
        println!("vec_words: {:?}", fin_v_words);
        let m_len = monkey_vec.len();
        if m_len > 0 {
            let curr_monkey = monkey_vec[m_len];
        }
        //make sure vec_words isn't empty
        if vec_words.len() > 0 {
            //get statement identifier
            let identity = &fin_v_words[0];
            match identity {
                Monkey => {}
                Starting => {}
                Operation => {}
                Test => {}
                If => {}
                _ => {}
            }
        }
    }
}
