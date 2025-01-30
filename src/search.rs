use crate::{degrees::in_degrees_sequence, dichromatic_number::{compute_dichromatric_number, to_dot}, lightness::{is_local_light, is_light}};


/// i < j
pub fn check_pair_degrees(in_degree: &Vec<usize>, un_degree: &Vec<usize>, i: usize, j: usize) -> bool{
    in_degree[i] <= in_degree[j] + un_degree[j]
}


pub fn check_degrees(in_degree: &Vec<usize>, un_degree: &Vec<usize>, i: usize) -> bool{
    (i == 0 || check_pair_degrees(in_degree, un_degree, i-1, i)) 
    && (i+1 == in_degree.len() || check_pair_degrees(in_degree, un_degree, i, i+1))
}

pub fn check_arc_degrees(in_degree: &Vec<usize>, un_degree: &Vec<usize>, i: usize, j: usize) -> bool{
    check_degrees(in_degree, un_degree, i) 
    && check_degrees(in_degree, un_degree, j)
    && (if i < j {check_pair_degrees(in_degree, un_degree, i, j)}
         else { check_pair_degrees(in_degree, un_degree, j, i)})
}


fn check_arc_degrees2(in_degree: &Vec<usize>, un_degree: &Vec<usize>, i: usize, j: usize) -> bool {
    for k in 1..in_degree.len() {
        if in_degree[0] > in_degree[k] + un_degree[k]{
            return false
        }
    }
    true
}



///  
pub fn search2(n: usize){
    println!("search 2");
    let mut adj = vec![vec![false; n];n];
    let mut in_degree = vec![0; n];
    let mut un_degree = vec![0; n];

    // We suppose we have an hamiltonian cycle
    for  i in 0..n {
        adj[i][(i+1)%n] = true;
        in_degree[i] = 1;
        un_degree[i] = n-3;
    }

    println!("{in_degree:?}");
    println!("{un_degree:?}");


    let mut todo = vec![];
    for  i in 0..n {
        for j in (i+2)..n {
            if i == 0 && j== n-1 {
                continue;
            }
            todo.push((i,j));
        }
    }

    let mut c = 0;


    let mut done = vec![];
    loop {
        while let Some((i,j)) = todo.pop(){
            adj[i][j] = true;
            in_degree[j] += 1;
            un_degree[j] -= 1;
            un_degree[i] -= 1;

            if check_arc_degrees2(&in_degree, &un_degree, i,j) && is_local_light(&adj, i, j) {
                done.push((i,j));
                continue;
            } else {
                adj[i][j] = false;
                in_degree[j] -= 1;
                un_degree[j] += 1;
                un_degree[i] += 1;

                if i < j {
                    todo.push((j,i));
                } else {
                    // Backtrack
                    let mut finito = true;
                    todo.push((j,i));
                    while let Some((x,y)) = done.pop(){
                        adj[x][y] = false;
                        in_degree[y] -= 1;
                        un_degree[y] += 1;
                        un_degree[x] += 1;
                        if x < y {
                            finito = false;
                            todo.push((y,x));
                            break;
                        }
                        else {
                            todo.push((y,x))
                        }
                    }
                    if finito {
                        println!("Number of light tournaments with Chi >= 3 and without twins: {c}");
                        return;
                    }
                }
            }
        }

        // Light tournament found
        let mut is_twin = false;
        for u in 0..n {
            for v in 0..u{
                let mut twin = true;
                for w in 0..n{
                    if w != u && w != v && adj[u][w] != adj[v][w] {
                        twin = false;
                    }
                }
                if twin {
                    is_twin = true;
                    break;
                }
            }
        }

        if is_twin == false {
            let chi = compute_dichromatric_number(&adj);
            if chi >= 3 {
                println!("chi={chi} {done:?}");
                println!("indegrees: {:?}", in_degrees_sequence(&adj));
                //to_dot(&adj);
                c += 1;
            }
            


        }

            
        
        // println!("light found");
        // println!("done= {done:?}");
        // println!("indeg= {:?}", in_degrees_sequence(&adj));
        // println!("{}", dichromatic_number::compute_dichromatric_number(&adj));

        // Backtrack
        let mut finito = true;
        while let Some((x,y)) = done.pop(){
            adj[x][y] = false;
            in_degree[y] -= 1;
            un_degree[y] += 1;
            un_degree[x] += 1;
            if x < y {
                finito = false;
                todo.push((y,x));
                break;
            }
            else {
                todo.push((y,x))
            }
        }
        if   finito{
            println!("Number of light tournaments with Chi >= 3 and without twins: {c}");
            return;
        }
    }
    

}