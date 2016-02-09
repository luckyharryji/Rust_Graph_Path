#[doc="graph traversal

__author__ = 'Xiangyu Ji,  Nianzu Li'

The program get the file representating undirected graph,
and from the standrad input was given pair of nodes inside a graph
find the path between them. 

INPUT:
    The program get the second parameter of the run command as the file name of the graph.
    Then it get queries on stdin, two alpha per line at a time. 
    A query consists of two node names, a starting node and an ending node:
        i.e: 
            a b


OUTPUT:
    If the input file name is invalid, program end with panic, i.e:
        Error with the Graph File reading

    If the number of start node and end node exceed 2, print the message:
        Invalid path start and end point

    If the input start/end node is not a valid node inside the graph represented by the file,
    print the message i.e:
        Input Start Point invalid
        Input End Point invalid

    If graph contains path between the input nodes, prints out nodes along the path between the nodes.
    If there isn't such path exists, print out the message.
        a d b
        No path

Assumptions:

 - Graph represented by the file is undirected Graph.

 - No self-loop

 - all nodes are reprented as lower cased letter

"]

pub mod lib;


use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};
use std::io::{Write, stdout};
use lib::{Graph, NewGraph};


fn main() {

    let args:Vec<_> = env::args().collect();
    if args.len()!=2{
        panic!("Error with the Graph File reading");
    }
    let file = File::open(&args[1]).expect("Error");

    let graph_info = get_graph(file);
    let new_graph = NewGraph::new(graph_info);
    find_n_show_path(stdin(), &mut stdout(), new_graph);
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


fn find_n_show_path<R:Read, W:Write>(reader:R, writer: &mut W, graph:NewGraph){
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next(){
        let points:Vec<&str> = line.split_whitespace().collect();
        match points.len(){
            2 => {
                let start_point = points[0].to_owned();
                let end_point = points[1].to_owned();
                let path_str = graph.dfs_path(start_point,end_point);

                if let Err(_) = (*writer).write(&*(format!("{}\n", path_str).into_bytes())){
                    panic!("Fail writing");
                }
            },
            _ => {
                panic!("Invalid path start and end point");
            },
        }
    }
}


#[cfg(test)]
mod read_src_n_dst_test {
    use super::{find_n_show_path};
    use lib::{Graph, NewGraph};
    use std::io::{Read, Result};


    #[test]
    fn no_path() {
        let graph = build_graph();
        let mock_read = StringReader::new("a e\na f".to_owned());
        let expected = String::from("No path\nNo path\n");

        let mut buf: Vec<u8> = Vec::new();
        find_n_show_path(mock_read, &mut buf, graph);
        assert_eq!(String::from_utf8(buf).unwrap(),expected);
    }

    #[test]
    fn read_one_line() {
        let graph = build_graph();
        let mock_read = StringReader::new("a b\n".to_owned());
        let expected = vec![String::from("a b\n"), String::from("a d b\n")];

        let mut buf: Vec<u8> = Vec::new();
        find_n_show_path(mock_read, &mut buf, graph);
        assert!(expected.contains(&String::from_utf8(buf).unwrap()));
    }


    #[test]
    fn read_more_lines() {
        let graph = build_graph();
        let mock_read = StringReader::new("a b\na e\n".to_owned());
        let expected = vec![String::from("a b\nNo path\n"), String::from("a d b\nNo path\n")];

        let mut buf: Vec<u8> = Vec::new();
        find_n_show_path(mock_read, &mut buf, graph);
        assert!(expected.contains(&String::from_utf8(buf).unwrap()));
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


#[cfg(test)]
mod read_n_build_graph {
    use super::{get_graph};
    use lib::{Graph, NewGraph, VertexId};
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
        //let expected = NewGraph::build(vertex,index,n,mat);

        let graph = NewGraph::new(g_info);
        assert_eq!(graph.get_matrix(), mat);
        assert_eq!(graph.get_index(), index);
        assert_eq!(graph.get_vertex(), vertex);

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
