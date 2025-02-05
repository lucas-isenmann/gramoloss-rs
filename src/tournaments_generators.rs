use std::collections::HashSet;


pub fn group_tournament(n: usize, m: usize, l: Vec<(usize, usize)>) -> Vec<Vec<bool>> {
    let size = n*m;
    let mut adj = vec![vec![false; size]; size];

    for i in 0..n{
        for j in 0..m {
            let x = i + j*n;
            for (a,b) in l.iter() {
                let y = ((i+n+a)%n) + ((j+m+b)%m)*n;
                adj[x][y] = true;
            }

            // for a in 0..n {
            //     for b in 0..m {
            //         let y = ((i+a)%n) + ((j+b)%m)*n;
            //         let w = ((i+n-a)%n) + ((j+m-b)%m)*n;
            //         if l[a][b] {
            //             adj[x][y] = true;
            //         } else {
            //             adj[x][w] = true;
            //         }
            //     }
            // }
        }
    }

    adj
}

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
