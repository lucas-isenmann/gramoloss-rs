


pub fn print_adj(adj: &Vec<Vec<bool>>){
    for row in adj {
        for &cell in row {
            if cell {
                print!("1");
            } else {
                print!(".")
            }
        }
        println!();
    }
}


use std::fs::File;
use std::io::{BufRead, BufReader};



pub struct MatrixGraph {
    adj_matrix: Vec<Vec<bool>>,
    n: usize,
}

impl MatrixGraph {
    fn new(n: usize) -> Self {
        let adj_matrix = vec![vec![false; n]; n];
        MatrixGraph { adj_matrix, n }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_matrix[u][v] = true;
        self.adj_matrix[v][u] = true;
    }

    pub fn has_edge(&self, u: usize, v: usize) -> bool{
        self.adj_matrix[u][v]
    }

    pub fn get_neighbors(&self, v: usize) -> Vec<usize> {
        (0..self.n)
            .filter(|i| self.adj_matrix[v][*i])
            .collect()
    }

    /// Returns the number of vertices
    pub fn nb_vertices(&self) -> usize {
        self.n
    }

    /// Returns the number of edges
    pub fn nb_edges(&self) -> usize{
        let mut c = 0;
        for i in 0..self.n {
            for j in 0..i {
                if self.adj_matrix[i][j] {
                    c += 1;
                }
            }
        }
        c
    }

    pub fn print(&self){
        print_adj(&self.adj_matrix);
    }

    /// Prints the adjacency list representation of the graph
    pub fn print_adj(&self) {
        for v in 0..self.n {
            print!("{} -> ", v);
            let neighbors: Vec<String> = self.get_neighbors(v)
                .iter()
                .map(|n| n.to_string())
                .collect();
            println!("{}", neighbors.join(" "));
        }
    }

    /// Prints the degree sequence in increasing order
    pub fn print_degree_sequence(&self) {
        let mut degrees: Vec<usize> = (0..self.n)
            .map(|v| self.get_neighbors(v).len())
            .collect();
        degrees.sort();
        
        println!("Degree sequence (increasing):");
        println!("{:?}", degrees);
    }

    /// Prints the degree of each vertex
    pub fn print_degrees(&self) {
        println!("Vertex degrees:");
        for v in 0..self.n {
            let degree = self.get_neighbors(v).len();
            println!("Vertex {}: degree = {}", v, degree);
        }
    }


    pub fn load_from_edge_list_file(file_path: &str) -> Result<Self, String> {
        // Open and read the file
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Could not open file: {}", e)),
        };
        
        let reader = BufReader::new(file);
        let lines: Vec<String> = match reader.lines().collect::<Result<Vec<_>, _>>() {
            Ok(lines) => lines,
            Err(e) => panic!("Error reading lines: {}", e),
        };

        // Read first line for vertices and edges count
        let first_line = match lines.first() {
            Some(line) => line,
            None => return Err("File is empty".to_string()),
        };
        
        // Parse vertices and edges count
        let mut parts = first_line.trim().split_whitespace();
        let n = match parts.next() {
            Some(s) => match s.parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err("Invalid number of vertices".to_string()),
            },
            None => return Err("Missing number of vertices".to_string()),
        };
        
        // Create new graph
        let mut graph = MatrixGraph::new(n);
        
        // Read and add edges
        for (line_num, line) in lines.iter().skip(1).enumerate() {
            
            let mut parts = line.trim().split_whitespace();
            let u = match parts.next() {
                Some(s) => match s.parse::<usize>() {
                    Ok(u) => u,
                    Err(_) => return Err(format!("Invalid vertex number at line {}: {}", line_num + 2, s)),
                },
                None => return Err(format!("Missing vertex at line {}", line_num + 2)),
            };
            
            let v = match parts.next() {
                Some(s) => match s.parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => return Err(format!("Invalid vertex number at line {}: {}", line_num + 2, s)),
                },
                None => return Err(format!("Missing vertex at line {}", line_num + 2)),
            };
            
            // Validate vertex indices
            if u >= n+1 || v >= n+1 {
                return Err(format!("Vertex index out of bounds at line {}: {} or {}", line_num + 2, u, v));
            }
            
            graph.add_edge(u-1, v-1);
        }
        
        Ok(graph)
    }








}