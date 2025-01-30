use std::collections::HashSet;

pub fn circulant_tournament(l: Vec<bool>) -> Vec<Vec<bool>> {
    let n = l.len()*2 +1;
    let mut adj = vec![vec![false; n];n];

    for i in 0..n{
        for j in 0..l.len() {
            if l[j] {
                adj[i][(i+j+1)%n] = true;
            } else {
                adj[i][(i-(j+1)+n)%n] = true;
            }
        }
    }

    adj
}


pub fn u_tournament(n: usize, k: usize) -> Vec<Vec<bool>> {
    let mut adj = vec![vec![false; n];n];

    for i in 0..n{
        for j in (i+1)..n {
            if j-i <= k {
                adj[j][i] = true;
            } else {
                adj[i][j] = true;
            }
        }
    }

    adj
}



pub fn ug_tournament(n: usize, gaps: HashSet<usize>) -> Vec<Vec<bool>> {
    let mut adj = vec![vec![false; n];n];

    for i in 0..n{
        for j in (i+1)..n {
            if gaps.contains(&(j-i)) {
                adj[j][i] = true;
            } else {
                adj[i][j] = true;
            }
        }
    }

    adj
}
