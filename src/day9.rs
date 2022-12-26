use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Grid {
    width: i32,
    height: i32,
}

#[derive(Debug)]
pub struct Head {
    grid: Grid,
    position: Point,
}

impl Head {
    fn new(grid: Grid) -> Self {
        Self {
            grid,
            position: Point { x: 0, y: 0 },
        }
    }
    //can make a function that moves either left right or down based on the input...
    //so you take two inputs, an i32 and a char or &str
    fn shift_head(&mut self, direction: &str) {
        match direction {
            "U" => self.position.y += 1,
            "D" => self.position.y -= 1,
            "L" => self.position.x -= 1,
            "R" => self.position.x += 1,
            _ => println!("SMT W WRONG"),
        }
    }
}

#[derive(Debug)]
pub struct Tail {
    grid: Grid,
    position: Point,
}

impl Tail {
    fn new(grid: Grid) -> Self {
        Self {
            grid,
            position: Point { x: 0, y: 0 },
        }
    }

    fn shift_tail(&mut self) {
        //so they need to be in the same row or column
        //and only be one apart
    }
}

//Part 1
pub fn day9() {
    let contents = fs::read_to_string("d9input.txt").expect("Failed to read file");

    let mut point_set = HashSet::new();
    let grid = Grid {
        width: 100,
        height: 100,
    };
    let mut tail = Tail::new(grid);
    let mut head = Head::new(grid);
    // println!("Head:{:?}", head);
    // println!("Tail:{:?}", tail);
    // println!("Grid:{:?}", grid);

    for line in contents.lines() {
        //get a vec of &str
        let c: Vec<&str> = line.split(" ").collect();
        let dir = c[0];
        let val: i32 = c[1].parse().expect("couldn't parse");
        for i in 0..=val {
            head.shift_head(dir);
            // println!("Head:{:?}", head);
            //now we need to calculate the difference between x & y
            let dx = head.position.x - tail.position.x;
            let dy = head.position.y - tail.position.y;
            //write some ifs on what to do
            //if dx and dy both equal 1 then you have a diagonal
            //else if its 2,0 then you move x+1
            if (dx == 2 && dy == 0) {
                tail.position.x += 1;
            }
            //else if its 0,2 then you move y+1
            else if (dx == 0 && dy == 2) {
                tail.position.y += 1;
            }
            //else if its -2,0 then you move x-1
            else if (dx == -2 && dy == 0) {
                tail.position.x -= 1;
            }
            //else if its 0,-2 then you move y-1
            else if (dx == 0 && dy == -2) {
                tail.position.y -= 1;
            } else if (dx == 2 && dy == 1) || (dx == 1 && dy == 2) {
                tail.position.x += 1;
                tail.position.y += 1;
            } else if (dx == -1 && dy == 2) || (dx == -2 && dy == 1) {
                tail.position.x -= 1;
                tail.position.y += 1;
            } else if (dx == -1 && dy == -2) || (dx == -2 && dy == -1) {
                tail.position.x -= 1;
                tail.position.y -= 1;
            } else if (dx == 2 && dy == -1) || (dx == 1 && dy == -2) {
                tail.position.x += 1;
                tail.position.y -= 1;
            }
            let new_point = tail.position;
            point_set.insert(new_point);
            println!("head position:{:?}", head.position);
            println!("tail position:{:?}", tail.position);
            println!("-------------------------")
        }
    }
    println!("Answer: {:?}", point_set.len())
}
