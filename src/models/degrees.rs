

pub fn in_degree(adj: &Vec<Vec<bool>>, i: usize) -> usize {
    let mut d = 0;
    for j in 0..adj.len() {
        if adj[j][i] {
            d += 1;
        }
    }
    d
}


pub fn in_degrees_sequence(adj: &Vec<Vec<bool>>) -> Vec<usize> {
    let n = adj.len();
    let mut in_degree = vec![0; n];
    for i in 0..n{
        for j in 0..n {
            if adj[j][i] {
                in_degree[i] += 1;
            }
        }
    }

    

    in_degree
}