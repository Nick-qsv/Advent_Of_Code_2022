use petgraph::{graph::Node, Graph};
use std::fs;

enum NodeType {
    Normal,
    Start,
    Goal,
}

//Part 1
pub fn day12() {
    let contents = fs::read_to_string("d12input.txt").expect("Failed to read file");

    //initialize the graph
    let mut g: Graph<NodeType, ()> = Graph::new();

    //initialize the grid
    let mut grid: Vec<Vec<char>> = vec![];

    //make the grid
    for line in contents.lines() {
        let vec: Vec<char> = line.chars().collect();
        grid.push(vec);
    }

    //so we need a good data structure to represent the map
    //what data structure did we use for
    //looks like we need to establish the graph to start off.  so thats step one. make the graph of the input
    //then you search it.  two parts
    //so we have a graph... need to add  all the nodes and edges

    //CHECK IF IT IS "S" OR "E" CAUSE then its special
    //otherwise then you can add it to the graph with the counter..
    //do we need multiple counters or is it going to get huge?  i don't think it will get that big.

    //make a counter variable
    let mut c = 0;

    //iterate through the whole grid
    // for(row_idx,row) in grid.iter().enumerate(){
    //     let mut row_iterator = row.iter().peekable();
    //     while let Some(c1) = row_iterator.next(){
    //         if let Some(row_below) = grid.get(row_idx+1){
    //             if let Some(c2) = row_below.get(row_iterator.index()){

    //             }
    //         }
    //     }
    // }

    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[row_idx].len() {
            let c1 = grid[row_idx][col_idx];
            if let Some(c2) = grid.get(row_idx).and_then(|row| row.get(col_idx + 1)) {
                if c1 == (*c2 as u8 + 1) as char || c1 == (*c2 as u8 - 1) as char {}
            }
        }
    }
}

// Sabqponm
// abcryxxl
// accszExk
// acctuvwj
// abdefghi
