use std::collections::HashSet;
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
    println!("first pass = {:?}", register_arr,);
}

pub fn day10p2() {
    let contents = fs::read_to_string("d10input.txt").unwrap();

    let mut register_arr: Vec<i32> = vec![1];
    let mut register_val: i32 = 1;
    for line in contents.lines() {
        let arr: Vec<&str> = line.split(" ").collect();
        let command = arr[0];
        let mut x_val: i32 = 0;
        if arr.len() == 2 {
            x_val = arr[1].parse().expect("");
        }
        //creating the register array
        if command == "noop" {
            register_arr.push(register_val);
        } else {
            for x in 1..3 {
                if x % 2 == 0 {
                    register_val += x_val;
                    register_arr.push(register_val);
                } else {
                    register_arr.push(register_val);
                }
            }
        }
    }
    //counter variable for clock cycle to keep track of index in register ARR
    let mut clock_cycle = 0;
    for _ in 1..7 {
        for x in 0..40 {
            //define hashset and current register
            let mut curr_set = HashSet::new();
            let curr_register = register_arr[clock_cycle];
            curr_set.insert(curr_register + 1);
            curr_set.insert(curr_register - 1);
            curr_set.insert(curr_register);
            //if set contains x (horizontal position) then print #
            if curr_set.contains(&x) {
                print!("#")
            } else {
                print!(".")
            }
            clock_cycle += 1;
        }
        println!();
    }
}

//Pseudocode

//need a new idea of looking at this
//addx 15 these are the steps:
//can use a for loop same as last ?  if %2 = 0
//current cycle = 1
//sprite position = (1 middle), 0 and 2 as well
//compare x to sprite position +-1
//draw thing
//still in for loop but now on 2
//sprite pos = same
//compare x to sprite pos +-1
//then add 15 to x at the end
//for noop does nothing really happen?  is it important to keep track of clock cycles? i dont think so.
//looping over each line though.  how do you set up the loops?
//one loop going over each line reading the command,
//another loop keeping track of x and y position
//instead of a loop can keep track of x and y using clock cycle?
//or can keep clock cycle... seems like thats the only way...

//So the issue is you are going through the for loop with no concept of what part of the array you are looking at
//realistically you want to keep count in the for loop and use that count to keep track of the index
//meanwhile using the X value to keep track of the position and compare it to the register
//so you can keep count of th eindex using a simple counter variable?  starts at 0 and adds 1 at the end of every
