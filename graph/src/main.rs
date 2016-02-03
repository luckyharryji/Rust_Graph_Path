


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


    // return the neighbor node  of index
    fn find_first_neighbor(&self, index:usize)->Option<usize>{
        if index >(self.vertive_num-1){
            return None;
        }
        for i in 0..self.vertive_num{
            if self.adj_matrix[index][i] == 1{
                return Some(i);
            }
        }
        return None;

    }

    // return the next neighbor that cound potentionaly not been visited
    fn find_next_neighbor(&self, index:usize,previous:usize)->Option<usize>{
        if index >(self.vertive_num-1)||previous>(self.vertive_num-1) {
            return None;
        }
        for i in previous+1..self.vertive_num{
            if self.adj_matrix[index][i] == 1{
                return Some(i);
            }
        }
        return None;        
    }


    fn dfs_find(&self,node_index:usize,end_index:usize,visited:&mut Vec<usize>,depth_temp: usize,path:&mut Vec<usize>){
        visited[node_index] = 1;
        if depth_temp>=path.len(){
            path.push(node_index);
        }else{
            path[depth_temp] = node_index;
        }
        let depth = depth_temp + 1;
        let mut get_first_neighbor = self.find_first_neighbor(node_index);
        while let Some(k) = get_first_neighbor{
            if k == end_index{
                path.push(k);
                println!("out put one result");
                for i in 0..depth+1{
                    print!("{} {} {}\n", i, self.vertices[path[i]],path[i]);
                }
                print!("\n");
                return   // how to fully jump out the recursion here?
            }
            if visited[k]==0{
                self.dfs_find(k,end_index,visited,depth,path);
            }
            get_first_neighbor = self.find_next_neighbor(node_index,k);
        }
    }

    fn dfs_path(&self, start:String, end:String){
        let mut visited = Vec::<usize>::new();
        let mut path = Vec::<usize>::new();
        for i in 0..self.vertive_num{
            visited.push(0);
        }
        let start_index = match char_to_index(&self.vertices,start){
            Some(index) => index,
            None => panic!("Input Start Point invalid"),
        };

        let end_index = match char_to_index(&self.vertices,end){
            Some(index) => index,
            None => panic!("Input End Point invalid"),
        };
        path.push(start_index);
        let depth = 0;
        self.dfs_find(start_index,end_index, &mut visited,depth,&mut path);
    }
}

fn main() {
    let graph = get_graph();
    let new_graph = NewGraph::new(graph);
    new_graph.dfs_path("a".to_owned(),"c".to_owned());
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


