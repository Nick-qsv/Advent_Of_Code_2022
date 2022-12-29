use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

//implement hash for point so you can add it to the hashset
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
}

//Part 1
pub fn day9() {
    let contents = fs::read_to_string("d9input.txt").expect("Failed to read file");

    let mut point_set = HashSet::new();
    let grid = Grid {
        width: 10000,
        height: 10000,
    };
    let mut tail = Tail::new(grid);
    let mut head = Head::new(grid);

    for line in contents.lines() {
        //get a vec of &str
        let c: Vec<&str> = line.split(" ").collect();
        let dir = c[0];
        let val: i32 = c[1].parse().expect("couldn't parse");
        for _ in 0..val {
            head.shift_head(dir);

            //now we need to calculate the difference between x & y
            let dx = head.position.x - tail.position.x;
            let dy = head.position.y - tail.position.y;

            // Check if dx is 2 or -2
            if dx == 2 || dx == -2 {
                // If dx is 2 or -2, move the x position by dx / 2
                tail.position.x += dx / 2;
                tail.position.y += dy;

            // If dx is not 2 or -2, check if dy is 2 or -2
            } else if dy == 2 || dy == -2 {
                // If dy is 2 or -2, move the y position by dy / 2
                tail.position.y += dy / 2;
                tail.position.x += dx;
            }
            //insert the tail position to the hashset
            let new_point = tail.position;
            point_set.insert(new_point);
            // println!("head position:{:?}", head.position);
            // println!("tail position:{:?}", tail.position);
            // println!("-------------------------")
        }
    }
    println!("Answer: {:?}", point_set.len())
}

//Part 2
pub fn day9p2() {
    let contents = fs::read_to_string("d9input.txt").expect("Failed to read file");

    let mut point_set = HashSet::new();
    let grid = Grid {
        width: 10000,
        height: 10000,
    };
    let mut tail_1 = Tail::new(grid);
    let mut head = Head::new(grid);
    let mut tail_2 = Tail::new(grid);
    let mut tail_3 = Tail::new(grid);
    let mut tail_4 = Tail::new(grid);
    let mut tail_5 = Tail::new(grid);
    let mut tail_6 = Tail::new(grid);
    let mut tail_7 = Tail::new(grid);
    let mut tail_8 = Tail::new(grid);
    let mut tail_9 = Tail::new(grid);

    for line in contents.lines() {
        //get a vec of &str
        let c: Vec<&str> = line.split(" ").collect();
        let dir = c[0];
        let val: i32 = c[1].parse().expect("couldn't parse");
        for _ in 0..val {
            head.shift_head(dir);

            //TAIL 1
            //now we need to calculate the difference between x & y
            let dx = head.position.x - tail_1.position.x;
            let dy = head.position.y - tail_1.position.y;

            // Check if dx is 2 or -2
            if dx == 2 || dx == -2 {
                // If dx is 2 or -2, move the x position by dx / 2
                tail_1.position.x += dx / 2;
                tail_1.position.y += dy;
            // If dx is not 2 or -2, check if dy is 2 or -2
            } else if dy == 2 || dy == -2 {
                // If dy is 2 or -2, move the y position by dy / 2
                tail_1.position.y += dy / 2;
                tail_1.position.x += dx;
            }

            //TAIL 2
            let dx_1 = tail_1.position.x - tail_2.position.x;
            let dy_1 = tail_1.position.y - tail_2.position.y;
            if dx_1.abs() == 2 && dy_1.abs() == 2 {
                tail_2.position.x += dx_1 / 2;
                tail_2.position.y += dy_1 / 2;
            } else if dx_1 == 2 || dx_1 == -2 {
                tail_2.position.x += dx_1 / 2;
                tail_2.position.y += dy_1;
            } else if dy_1 == 2 || dy_1 == -2 {
                tail_2.position.y += dy_1 / 2;
                tail_2.position.x += dx_1;
            }

            //TAIL 3
            let dx_2 = tail_2.position.x - tail_3.position.x;
            let dy_2 = tail_2.position.y - tail_3.position.y;
            if dx_2.abs() == 2 && dy_2.abs() == 2 {
                tail_3.position.x += dx_2 / 2;
                tail_3.position.y += dy_2 / 2;
            } else if dx_2 == 2 || dx_2 == -2 {
                tail_3.position.x += dx_2 / 2;
                tail_3.position.y += dy_2;
            } else if dy_2 == 2 || dy_2 == -2 {
                tail_3.position.y += dy_2 / 2;
                tail_3.position.x += dx_2;
            }

            //TAIL 4
            let dx_3 = tail_3.position.x - tail_4.position.x;
            let dy_3 = tail_3.position.y - tail_4.position.y;
            if dx_3.abs() == 2 && dy_3.abs() == 2 {
                tail_4.position.x += dx_3 / 2;
                tail_4.position.y += dy_3 / 2;
            } else if dx_3 == 2 || dx_3 == -2 {
                tail_4.position.x += dx_3 / 2;
                tail_4.position.y += dy_3;
            } else if dy_3 == 2 || dy_3 == -2 {
                tail_4.position.y += dy_3 / 2;
                tail_4.position.x += dx_3;
            }

            //TAIL 5
            let dx_4 = tail_4.position.x - tail_5.position.x;
            let dy_4 = tail_4.position.y - tail_5.position.y;
            if dx_4.abs() == 2 && dy_4.abs() == 2 {
                tail_5.position.x += dx_4 / 2;
                tail_5.position.y += dy_4 / 2;
            } else if dx_4 == 2 || dx_4 == -2 {
                tail_5.position.x += dx_4 / 2;
                tail_5.position.y += dy_4;
            } else if dy_4 == 2 || dy_4 == -2 {
                tail_5.position.y += dy_4 / 2;
                tail_5.position.x += dx_4;
            }

            //TAIL 6
            let dx_5 = tail_5.position.x - tail_6.position.x;
            let dy_5 = tail_5.position.y - tail_6.position.y;
            if dx_5.abs() == 2 && dy_5.abs() == 2 {
                tail_6.position.x += dx_5 / 2;
                tail_6.position.y += dy_5 / 2;
            } else if dx_5 == 2 || dx_5 == -2 {
                tail_6.position.x += dx_5 / 2;
                tail_6.position.y += dy_5;
            } else if dy_5 == 2 || dy_5 == -2 {
                tail_6.position.y += dy_5 / 2;
                tail_6.position.x += dx_5;
            }
            //TAIL 7
            let dx_6 = tail_6.position.x - tail_7.position.x;
            let dy_6 = tail_6.position.y - tail_7.position.y;
            if dx_6.abs() == 2 && dy_6.abs() == 2 {
                tail_7.position.x += dx_6 / 2;
                tail_7.position.y += dy_6 / 2;
            } else if dx_6 == 2 || dx_6 == -2 {
                tail_7.position.x += dx_6 / 2;
                tail_7.position.y += dy_6;
            } else if dy_6 == 2 || dy_6 == -2 {
                tail_7.position.y += dy_6 / 2;
                tail_7.position.x += dx_6;
            }

            //TAIL 8
            let dx_7 = tail_7.position.x - tail_8.position.x;
            let dy_7 = tail_7.position.y - tail_8.position.y;
            if dx_7.abs() == 2 && dy_7.abs() == 2 {
                tail_8.position.x += dx_7 / 2;
                tail_8.position.y += dy_7 / 2;
            } else if dx_7 == 2 || dx_7 == -2 {
                tail_8.position.x += dx_7 / 2;
                tail_8.position.y += dy_7;
            } else if dy_7 == 2 || dy_7 == -2 {
                tail_8.position.y += dy_7 / 2;
                tail_8.position.x += dx_7;
            }

            //TAIL 9
            let dx_8 = tail_8.position.x - tail_9.position.x;
            let dy_8 = tail_8.position.y - tail_9.position.y;
            if dx_8.abs() == 2 && dy_8.abs() == 2 {
                tail_9.position.x += dx_8 / 2;
                tail_9.position.y += dy_8 / 2;
            } else if dx_8 == 2 || dx_8 == -2 {
                tail_9.position.x += dx_8 / 2;
                tail_9.position.y += dy_8;
            } else if dy_8 == 2 || dy_8 == -2 {
                tail_9.position.y += dy_8 / 2;
                tail_9.position.x += dx_8;
            }
            //insert the tail_9 position to the hashset
            let new_point = tail_9.position;
            point_set.insert(new_point);
            // println!("Pointset!{:?}", point_set);
            // println!("head position:{:?}", head.position);
            // println!("tail_1 position:{:?}", tail_1.position);
            // println!("tail_2 position:{:?}", tail_2.position);
            // println!("tail_3 position:{:?}", tail_3.position);
            // println!("tail_4 position:{:?}", tail_4.position);
            // println!("tail_5 position:{:?}", tail_5.position);
            // println!("tail_6 position:{:?}", tail_6.position);
            // println!("tail_7 position:{:?}", tail_7.position);
            // println!("tail_8 position:{:?}", tail_8.position);
            // println!("tail_9 position:{:?}", tail_9.position);
            // println!("-------------------------")
        }
    }
    println!("Answer: {:?}", point_set.len());
    // println!("Point Set!{:?}", point_set)
}

pub fn test_openai() {
    let points = vec![
        Point { x: -8, y: 1 },
        Point { x: -6, y: -1 },
        Point { x: 9, y: 1 },
        Point { x: 5, y: 5 },
        Point { x: 7, y: 3 },
        Point { x: 3, y: -5 },
        Point { x: 2, y: 2 },
        Point { x: 1, y: 3 },
        Point { x: 1, y: 1 },
        Point { x: -2, y: -5 },
        Point { x: 0, y: -5 },
        Point { x: 4, y: 5 },
        Point { x: 2, y: -5 },
        Point { x: 10, y: 0 },
        Point { x: -9, y: 2 },
        Point { x: -11, y: 4 },
        Point { x: 5, y: -5 },
        Point { x: -11, y: 6 },
        Point { x: -3, y: -4 },
        Point { x: -5, y: -2 },
        Point { x: 1, y: -5 },
        Point { x: -10, y: 3 },
        Point { x: 6, y: 4 },
        Point { x: 8, y: 2 },
        Point { x: -7, y: 0 },
        Point { x: 4, y: -5 },
        Point { x: -1, y: -5 },
        Point { x: 6, y: -4 },
        Point { x: -4, y: -3 },
        Point { x: 7, y: -3 },
        Point { x: 0, y: 0 },
        Point { x: 3, y: 5 },
        Point { x: -11, y: 5 },
        Point { x: 2, y: 4 },
        Point { x: 9, y: -1 },
        Point { x: 8, y: -2 },
    ];

    for y in -11..=6 {
        for x in -11..=10 {
            if points.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
