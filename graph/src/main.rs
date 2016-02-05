


use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};
use std::io::{Write, stdout};


type Graph = HashMap<String, Vec<String>>;
type VertexId = HashMap<String, usize>; 

struct NewGraph{
    vertices: Vec<String>,
    indices: VertexId,
    vertive_num: usize,
    adj_matrix: Vec<Vec<usize>>,
}

impl NewGraph{
    fn new(input:Graph)->Self{
        let mut vertex = Vec::<String>::new();
        let mut index = VertexId::new();
        let mut matrix = Vec::<Vec<usize>>::new();

        let mut size = 0;


        for (node,neighbor) in input.iter(){
            if !index.contains_key(node){
                    vertex.push(node.to_owned());
                    index.insert(node.to_owned(), size);
                    size += 1;
            }

            for i in 0..neighbor.len(){
                if !index.contains_key(&neighbor[i]){
                    vertex.push(neighbor[i].to_owned());
                    index.insert(neighbor[i].to_owned(), size);
                    size += 1;
                }
            }
        }

        let mut temp = Vec::<usize>::new();
        for j in 0..size {
            temp.push(0);
        }

        for i in 0..size {

            matrix.push(temp.clone());
        }

        for (node,neighbor) in input.iter(){
            match index.get(&node.to_owned()){
                None => panic!("Node invalid!!"),
                Some(row) => {
                    for i in 0..neighbor.len(){
                        match index.get(&neighbor[i].to_owned()){
                            None => panic!("Node invalid!!"),
                            Some(colum) =>{
                                matrix[row.to_owned()][colum.to_owned()] = 1;
                                matrix[colum.to_owned()][row.to_owned()] = 1;
                            }
                        }   
                    }
                }
            }
        }

        NewGraph{vertices:vertex, indices: index, vertive_num:size,adj_matrix:matrix}
    }

    fn char_to_index(&self, alpha:String)->Option<usize>{
        if let Some(id) = self.indices.get(&alpha){
            return Some(id.to_owned());
        }
        None
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


    fn dfs_find(&self,node_index:usize,end_index:&usize,visited:&mut Vec<usize>,depth_temp: usize,path:&mut Vec<usize>)->Option<usize>{
        visited[node_index] = 1;
        if depth_temp>=path.len(){
            path.push(node_index);
        }else{
            path[depth_temp] = node_index;
        }
        let depth = depth_temp + 1;
        let mut get_first_neighbor = self.find_first_neighbor(node_index);
        while let Some(k) = get_first_neighbor{
            if k == end_index.to_owned(){
                return Some(depth);
            }
            if visited[k]==0{
                if let Some(index) = self.dfs_find(k,end_index,visited,depth,path){
                    return Some(index);
                }
            }
            get_first_neighbor = self.find_next_neighbor(node_index,k);
        }
        None
    }

    fn dfs_path(&self, start:String, end:String) -> String {
        let mut visited = Vec::<usize>::new();
        let mut path = Vec::<usize>::new();
        for i in 0..self.vertive_num{
            visited.push(0);
        }
        let start_index = match self.char_to_index(start){
            Some(index) => index,
            None => panic!("Input Start Point invalid"),
        };
        let end_index = match self.char_to_index(end){
            Some(index) => index,
            None => panic!("Input End Point invalid"),
        };

        path.push(start_index);
        let depth = 0;
        match self.dfs_find(start_index,&end_index, &mut visited,depth,&mut path){
            None => return String::from("No path"),
            Some(all_depth)=>{
                let mut path_str = String::from("");
                for i in 0..all_depth{
                    path_str.push_str(&self.vertices[path[i]]);
                    path_str.push_str(" ");
                }
                path_str.push_str(&self.vertices[end_index]);
                return path_str;
            },
        }
    }
}

fn find_n_show_path<R:Read, W:Write>(reader:R, writer: &mut W, graph:NewGraph){
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next(){
        let points:Vec<&str> = line.split_whitespace().collect();
        match points.len(){
            2 => {
                let start_point = points[0].to_owned();
                let end_point = points[1].to_owned();
                let path_str = graph.dfs_path(start_point,end_point);

                if let Err(x) = (*writer).write(&*(format!("{}\n", path_str).into_bytes())){
                    panic!("Fail writing");
                }
            },
            _ => {
                panic!("invalid path start and end point");
            },
        }
    }
}


#[cfg(test)]
mod read_src_n_dst_test {
    use super::{find_n_show_path, Graph, NewGraph};
    use std::io::{Read, Result};


    #[test]
    fn read_one_line() {
        let graph = build_graph();
        let mock_read = StringReader::new("a d\n".to_owned());
        let expected = String::from("a b d\n");

        let mut buf: Vec<u8> = Vec::new();
        find_n_show_path(mock_read, &mut buf, graph);
        assert_eq!(String::from_utf8(buf).unwrap(), expected);
    }


    #[test]
    fn read_two_lines() {
        let graph = build_graph();
        let mock_read = StringReader::new("a b\na e\n".to_owned());
        let expected = String::from("a b\nNo path\n");

        let mut buf: Vec<u8> = Vec::new();
        find_n_show_path(mock_read, &mut buf, graph);
        assert_eq!(String::from_utf8(buf).unwrap(), expected);
    }



    fn build_graph() -> NewGraph {
        let mut g = Graph::new();
        g.insert("a".to_owned(), vec!["b".to_owned(),"d".to_owned()]);
        g.insert("b".to_owned(), vec!["a".to_owned(),"d".to_owned()]);
        g.insert("c".to_owned(), vec![]);
        g.insert("d".to_owned(), vec!["c".to_owned()]);
        g.insert("e".to_owned(), vec!["f".to_owned()]);

        let graph_mat = NewGraph::new(g);
        return graph_mat;
    }


    struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }

    impl StringReader {
        fn new(s: String) -> Self {
            StringReader {
                contents: s.into_bytes(),
                position: 0,
            }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;

            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }

            return Ok(count);
        }
    }
}


fn get_graph<R:Read>(reader: R)->Graph{

    let mut reader = BufReader::new(reader).lines();
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

#[cfg(test)]
mod read_n_build_graph {
    use super::{get_graph, Graph, NewGraph, VertexId};
    use std::io::{Read, Result};


    #[test]
    fn read_simple_graph() {
        let mut expected = Graph::new();
        expected.insert("a".to_owned(), vec!["b".to_owned(),"c".to_owned()]);
        let mock_read = StringReader::new("a b c\n".to_owned());

        assert_eq!(get_graph(mock_read), expected);
    }


    #[test]
    fn read_complex_graph() {
        let expected = build_graph_info();
        let mock_read = StringReader::new("a b d\nb a d\nc \nd c\ne f\n".to_owned());

        assert_eq!(get_graph(mock_read), expected);
    }

    #[test]
    fn bulid_simple_graph() {
        let mock_read = StringReader::new("a b c\n".to_owned());
        let g_info = get_graph(mock_read);
        let vertex = vec!["a".to_owned(),"b".to_owned(),"c".to_owned()];
        let mut index = VertexId::new();
        index.insert("a".to_owned(), 0);
        index.insert("b".to_owned(), 1);
        index.insert("c".to_owned(), 2);
        let n = 3;
        let mut mat = Vec::<Vec<usize>>::new();
        mat.push(vec![0, 1, 1]);
        mat.push(vec![1, 0, 0]);
        mat.push(vec![1, 0, 0]);
        let expected = NewGraph{vertices:vertex,indices:index,vertive_num:n,adj_matrix:mat};

        let graph = NewGraph::new(g_info);
        assert_eq!(graph.adj_matrix, expected.adj_matrix);

    }


    fn build_graph_info() -> Graph {
        let mut g = Graph::new();
        g.insert("a".to_owned(), vec!["b".to_owned(),"d".to_owned()]);
        g.insert("b".to_owned(), vec!["a".to_owned(),"d".to_owned()]);
        g.insert("c".to_owned(), vec![]);
        g.insert("d".to_owned(), vec!["c".to_owned()]);
        g.insert("e".to_owned(), vec!["f".to_owned()]);

        return g;
    }


    fn build_graph() -> NewGraph {
        let mut g = Graph::new();
        g.insert("a".to_owned(), vec!["b".to_owned(),"d".to_owned()]);
        g.insert("b".to_owned(), vec!["a".to_owned(),"d".to_owned()]);
        g.insert("c".to_owned(), vec![]);
        g.insert("d".to_owned(), vec!["c".to_owned()]);
        g.insert("e".to_owned(), vec!["f".to_owned()]);

        let graph_mat = NewGraph::new(g);
        return graph_mat;
    }



    struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }

    impl StringReader {
        fn new(s: String) -> Self {
            StringReader {
                contents: s.into_bytes(),
                position: 0,
            }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;

            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }

            return Ok(count);
        }
    }
}


fn main() {

    let args:Vec<_> = env::args().collect();
    if args.len()!=2{
        panic!("Error with the Graph File reading");
    }
    let file = File::open(&args[1]).expect("Error");

    let graph = get_graph(file);
    let new_graph = NewGraph::new(graph);
    find_n_show_path(stdin(), &mut stdout(), new_graph);
}
