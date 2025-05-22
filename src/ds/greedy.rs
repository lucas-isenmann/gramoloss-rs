use std::collections::HashSet;
use crate::matrix_graph::MatrixGraph;




/// To improve: use to_dominate?
pub fn greedy_ds(g: &MatrixGraph) -> HashSet<usize>{
    let n = g.nb_vertices();

    let mut choosable: HashSet<usize> = (0..n).collect();
    let mut chosen = HashSet::new();
    let mut is_dominated: Vec<bool> = vec!(false;n);
    let mut nb_not_dominated = n;

    while nb_not_dominated > 0 {

        // Look for the vertex which dominates the maximum number of not dominated vertices
        let mut best_i = 0;
        let mut best_d = 0;
        for &i in choosable.iter() {
            let mut utility = 0;
            if is_dominated[i] == false {
                utility += 1;
            }
            for j in 0..n {
                if g.has_edge(i,j) && is_dominated[j] == false {
                    utility += 1;
                }
            }
            if utility > best_d {
                best_d = utility;
                best_i = i;
            }
        }

        choosable.remove(&best_i);
        chosen.insert(best_i);
        is_dominated[best_i] = true;
        for j in 0..n {
            if g.has_edge(best_i,j) && is_dominated[j] == false {
                is_dominated[j] = true;
            }
        }
        nb_not_dominated -= best_d;

    }

    chosen
}





pub fn greedy_ds_v2(g: &MatrixGraph) -> HashSet<usize>{
    println!("greedy ds 2");
    let n = g.nb_vertices();

    let mut choosable: HashSet<usize> = (0..n).collect();
    let mut chosen = HashSet::new();
    let mut is_dominated: Vec<bool> = vec!(false;n);
    let mut nb_not_dominated = n;

    while nb_not_dominated > 0 {
        // Look for the vertex which dominates the min number of not dominated vertices
        let mut min_v = 0;
        let mut min_utility = n+1;
        for i in 0..n {
            if is_dominated[i] == false {
                let mut d = 0;
                for j in 0..n {
                    if g.has_edge(i,j) && is_dominated[j] == false {
                        d += 1;
                    }
                }
                if d < min_utility {
                    min_utility = d;
                    min_v = i;
                }
            }
        }

        

        let mut max_utility = 0;
        let mut max_v = 0;
        for x in g.get_neighbors(min_v) {
            let mut utility = 0;
            if is_dominated[x] == false {
                utility += 1;
            }
            for y in g.get_neighbors(x) {
                if is_dominated[y] == false {
                    utility += 1;
                }
            }
            if utility > max_utility {
                max_utility = utility;
                max_v = x;
            }
        }

        

        choosable.remove(&max_v);
        chosen.insert(max_v);
        is_dominated[max_v] = true;
        for j in 0..n {
            if g.has_edge(max_v,j) && is_dominated[j] == false {
                is_dominated[j] = true;
            }
        }
        nb_not_dominated -= max_utility;

    }

    chosen
}
