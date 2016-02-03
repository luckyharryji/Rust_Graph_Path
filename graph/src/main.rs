


use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};


type PathList = HashMap<Vec<String>, Vec<String>>;
type Graph = HashMap<String, Vec<String>>;

struct NewGraph{
    vertices: Vec<String>,
    vertive_num: usize,
    adj_matrix: Vec<Vec<usize>>,
}

fn char_to_index(vertices:&Vec<String>,alpha:String)->Option<usize>{
    for i in 0..vertices.len(){
        if vertices[i]==alpha{
            return Some(i);
        }
    }
    None
}

impl NewGraph{
    fn new(input:Graph)->Self{
        let mut vertex = Vec::<String>::new();
        let mut matrix = Vec::<Vec<usize>>::new();
        let mut temp = Vec::<usize>::new();


        for ver in input.keys(){
            vertex.push(ver.to_owned());
            temp.push(0);
        }

        for i in 0..vertex.len(){
            matrix.push(temp.clone());
        }
        let length = vertex.len();

        for (node,neighbor) in input.iter(){
            match char_to_index(&vertex,node.to_owned()){
                None => panic!("Node invalid!!"),
                Some(row) => {
                    for i in 0..neighbor.len(){
                        match char_to_index(&vertex,neighbor[i].to_owned()){
                            None => panic!("Node invalid!!"),
                            Some(colum) =>{
                                matrix[row][colum] = 1;
                                matrix[colum][row] = 1;
                            }
                        }   
                    }
                }
            }
        }

        NewGraph{vertices:vertex,vertive_num:length,adj_matrix:matrix}
    }
}

fn main() {
    let graph = get_graph();
    let new_graph = NewGraph::new(graph);
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


