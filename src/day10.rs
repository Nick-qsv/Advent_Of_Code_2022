use std::fs;

pub fn day10() {
    let contents = fs::read_to_string("d10input.txt").unwrap();

    let mut clock_cycle_arr: Vec<i32> = vec![];
    let mut register_arr: Vec<i32> = vec![1];
    let mut register_val: i32 = 1;
    let mut clock_cycle_val: i32 = 0;
    for line in contents.lines() {
        // so theres two things to keep track of
        //the current cycle
        //X
        //noop precludes you from using modulo to track
        //but you can use modulo to track if command is addx
        //for x in 1..3 if x % 2 = 0
        //else add 1 to clock cycle
        //can have two arrays, one tracking clock cycle, the other tracking X
        let arr: Vec<&str> = line.split(" ").collect();
        let command = arr[0];
        let mut x_val: i32 = 0;
        if arr.len() == 2 {
            x_val = arr[1].parse().expect("");
        }
        // println!("Command:{}", command);
        // println!("xval:{}", x_val);
        if command == "noop" {
            clock_cycle_val += 1;
            clock_cycle_arr.push(clock_cycle_val);
            register_arr.push(register_val);
        } else {
            for x in 1..3 {
                if x % 2 == 0 {
                    clock_cycle_val += 1;
                    clock_cycle_arr.push(clock_cycle_val);
                    register_val += x_val;
                    register_arr.push(register_val);
                } else {
                    clock_cycle_val += 1;
                    clock_cycle_arr.push(clock_cycle_val);
                    register_arr.push(register_val);
                }
            }
        }
    }
    let answer = register_arr[19] * clock_cycle_arr[19]
        + register_arr[59] * clock_cycle_arr[59]
        + register_arr[99] * clock_cycle_arr[99]
        + register_arr[139] * clock_cycle_arr[139]
        + register_arr[179] * clock_cycle_arr[179]
        + register_arr[219] * clock_cycle_arr[219];
    println!("answer = {}", answer);
    // println!("first pass = {:?}", register_arr,);
}

pub fn day10p2() {
    let contents = fs::read_to_string("d10input.txt").unwrap();

    let mut clock_cycle_arr: Vec<i32> = vec![];
    let mut register_arr: Vec<i32> = vec![1];
    let mut register_val: i32 = 1;
    let mut clock_cycle_val: i32 = 0;
    for line in contents.lines() {
        // so theres two things to keep track of
        //the current cycle
        //X
        //noop precludes you from using modulo to track
        //but you can use modulo to track if command is addx
        //for x in 1..3 if x % 2 = 0
        //else add 1 to clock cycle
        //can have two arrays, one tracking clock cycle, the other tracking X
        let arr: Vec<&str> = line.split(" ").collect();
        let command = arr[0];
        let mut x_val: i32 = 0;
        if arr.len() == 2 {
            x_val = arr[1].parse().expect("");
        }
        // println!("Command:{}", command);
        // println!("xval:{}", x_val);
        if command == "noop" {
            clock_cycle_val += 1;
            clock_cycle_arr.push(clock_cycle_val);
            register_arr.push(register_val);
        } else {
            for x in 1..3 {
                if x % 2 == 0 {
                    clock_cycle_val += 1;
                    clock_cycle_arr.push(clock_cycle_val);
                    register_val += x_val;
                    register_arr.push(register_val);
                } else {
                    clock_cycle_val += 1;
                    clock_cycle_arr.push(clock_cycle_val);
                    register_arr.push(register_val);
                }
            }
        }
    }
    //i think you can just use the register arr array
    //if register arr[x] or register arr[x-1] or register arr[x+1] == x then print(#) else print (.)
    for y in 1..7 {
        for x in 1..41 {
            let index = y * x;
            //index can't be negative one so check if its 0
            // if index == 0 {
            //     if register_arr[index] == x as i32 || register_arr[index + 1] == x as i32 {
            //         print!("#");
            //     } else {
            //         print!(".");
            //     }
            // }
            if (index != 0) && (index + 1 < register_arr.len()) {
                if register_arr[index] == x as i32
                    || register_arr[index + 1] == x as i32
                    || register_arr[index - 1] == x as i32
                {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
    // println!("register arr = {:?}", register_arr,);
    // println!("clock arr = {:?}", clock_cycle_arr,);
}
