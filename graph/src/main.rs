

use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader};


fn main() {
    let graph = get_graph();
}


fn get_graph()->Vec<Vec<u8>>{
	let args:Vec<_> = env::args().collect();
    if args.len()!=2{
    	panic!("Error with the Graph File reading");
    }
    let file = File::open(&args[1]).expect("Error");
    let mut reader = BufReader::new(file).lines();
    let mut graph = Vec::<Vec<u8>>::new();

    while let Some(Ok(line)) = reader.next(){
    	let node_info = line.to_owned();
        let nodes: Vec<&str> = node_info.split_whitespace().collect();
        let mut gather_node = Vec::new();
    	for node in nodes{
    		gather_node.push(node.to_owned().as_bytes()[0]);
    	}
    	graph.push(gather_node);
    }
    graph
}

