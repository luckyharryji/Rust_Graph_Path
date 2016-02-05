
use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};
use std::io::{Write, stdout};


pub type Graph = HashMap<String, Vec<String>>;
pub type VertexId = HashMap<String, usize>; 

pub struct NewGraph{
    vertices: Vec<String>,
    indices: VertexId,
    vertive_num: usize,
    adj_matrix: Vec<Vec<usize>>,
}

impl NewGraph{
    pub fn new(input:Graph)->Self{
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

    pub fn build(vertex: Vec<String>, id: VertexId, num: usize, mat: Vec<Vec<usize>>) -> Self{
        NewGraph{vertices:vertex, indices: id, vertive_num:num,adj_matrix:mat}
    }

    pub fn get_matrix(&self) -> Vec<Vec<usize>>{
        self.adj_matrix.clone()
    }

    pub fn get_vertex(&self) -> Vec<String> {
        self.vertices.clone()
    }

    pub fn get_index(&self) -> VertexId {
        self.indices.clone()
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

    pub fn dfs_path(&self, start:String, end:String) -> String {
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

