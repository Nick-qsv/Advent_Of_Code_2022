use petgraph::Graph;
use std::fs;

//Part 1
pub fn day12() {
    let contents = fs::read_to_string("d12input.txt").expect("Failed to read file");

    //initialize the graph
    let mut g: Graph<&str, &str> = Graph::new();

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
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            //check if it is "S" or "E"

            //if not then add the node + number?

            //do two loops need to be done?
            //checking if each is has an edge? adding the edges?
            //
        }
    }
}
