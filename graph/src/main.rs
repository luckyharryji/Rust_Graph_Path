


use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};


type PathList = HashMap<Vec<String>, Vec<String>>;
type Graph = HashMap<String, Vec<String>>;

fn main() {
    let graph = get_graph();
}


fn get_graph()->Graph{
	let args:Vec<_> = env::args().collect();
    if args.len()!=2{
    	panic!("Error with the Graph File reading");
    }
    let file = File::open(&args[1]).expect("Error");
    let mut reader = BufReader::new(file).lines();
    let mut graph = Graph::new();

    while let Some(Ok(line)) = reader.next(){
    	let node_info = line.to_owned();
        let nodes: Vec<&str> = node_info.split_whitespace().collect();

    	let mut neighbor = Vec::<String>::new();
    	for i in 1..nodes.len(){
    		neighbor.push(nodes[i].to_owned());
    	}
    	graph.insert(nodes[0].to_owned(),neighbor);
    }
    graph
}


