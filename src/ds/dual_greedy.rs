use std::{collections::HashSet, hash::Hash};
use crate::MatrixGraph;




// To improve: use matrix multiplication A^2 : gives vertices at distance 2
// Ecrire ça comme un greedy_indep_set du graph à distance au plus 2


/// Add any vertex iteratively until we cannot add anymore
pub fn greedy_lower_bound_ds_v1(g: &MatrixGraph) -> HashSet<usize> {
    let n = g.nb_vertices();
    let mut choosable: Vec<usize> = (0..n).collect();
    let mut subset = HashSet::new();


    while let Some(v) = choosable.pop() {

        subset.insert(v);

        // Recompute choosable
        let mut new_choosable = Vec::new();
        for &w in &choosable {
            if !g.has_edge(v, w) && w != v {
                let mut dist2 = false;
                for z in 0..n {
                    if g.has_edge(v, z) && g.has_edge(w, z) {
                        dist2 = true;
                        break;
                    }
                }
                if !dist2 {
                    new_choosable.push(w);
                }
            }
        }
        choosable = new_choosable;
    }

    subset
}


/// Extend an initial subset of vertices by adding iteratively the min utility vertex.
/// - initial_subset is supposed to be a 2-independent set.
pub fn extend_greedy_2is(g: &MatrixGraph, initial_subset: &mut HashSet<usize>) {
    let n = g.nb_vertices();

    // Compute the vertices which can be chosen:
    // They are the vertices at distance at least 2 from initial_subset
    let mut choosable = vec![];
    for v in 0..n {
        if initial_subset.contains(&v) == false {
            let mut is_ok = true;
            for &x in initial_subset.iter() {
                if g.has_edge(x, v){
                    is_ok = false;
                    break;
                }
                for y in 0..n {
                    if g.has_edge(x, y) && g.has_edge(y, v) {
                        is_ok = false;
                        break;
                    }
                }
                if is_ok == false {
                    break;
                }
            }
            if is_ok {
                choosable.push(v);
            }
        }
    }

    let mut to_dominate: HashSet<usize> = (0..n).collect();
    for &x in initial_subset.iter() {
        to_dominate.remove(&x);
        for v in g.get_neighbors(x) {
            to_dominate.remove(&v);
        }
    }
    

    while choosable.is_empty() == false {

        let mut min_utility = g.nb_edges()+1;
        let mut best_index = 0;
        for (i, &v) in choosable.iter().enumerate() {
            let mut utility = 0;
            for &x in to_dominate.iter() {
                if g.has_edge(x, v) {
                    utility += 1;
                }
            }
            if utility < min_utility {
                min_utility = utility;
                best_index = i;
            }
        }

        let v = choosable[best_index];
        initial_subset.insert(v);

        // Mark neighbors of v as dominated
        for x in g.get_neighbors(v) {
            to_dominate.remove(&x);
        }

        // Recompute choosable
        let mut new_choosable = Vec::new();
        for &w in &choosable {
            if !g.has_edge(v, w) && w != v {
                let mut dist2 = false;
                for z in 0..n {
                    if g.has_edge(v, z) && g.has_edge(w, z) {
                        dist2 = true;
                        break;
                    }
                }
                if !dist2 {
                    new_choosable.push(w);
                }
            }
        }
        choosable = new_choosable;
    }

}



pub fn greedy_dual_ds_v2(g: &MatrixGraph) -> HashSet<usize>{
    let mut dual = HashSet::new();
    extend_greedy_2is(g, &mut dual);
    dual
}


use itertools::Itertools;



/// Check if all vertices of subset are at distance at least 3 from each other
pub fn is_2_independent_set(g: &MatrixGraph, subset: &HashSet<usize>) -> bool {
    let n = g.nb_vertices();
    for &x in subset.iter() {
        for &y in subset.iter() {
            if x < y {
                if g.has_edge(x, y){
                    return false;
                }
                for z in 0..n {
                    if g.has_edge(x, z) && g.has_edge(y, z) {
                        return false;
                    }
                }
            }
        }
    }        
    true
}



pub fn semi_exact_dual_ds(g: &MatrixGraph, k: usize) -> HashSet<usize> {
    let n = g.nb_vertices();
    
    if k == 0 {
        let mut subset: HashSet<usize> = HashSet::new();
        extend_greedy_2is(g, &mut subset);
        return subset;
    }

    if k > n {
        return HashSet::new();
    }

    let vertices: Vec<_> = (0..n).collect();

    let mut best_subset = HashSet::new();

    // Try all subsets of V of size k
    for subset in vertices.into_iter().combinations(k) {
        // println!("{subset:?}");
        let mut subset: HashSet<usize> = subset.into_iter().collect();

        if is_2_independent_set(g, &subset) {
            extend_greedy_2is(g, &mut subset);
        
            if subset.len() > best_subset.len() {
                best_subset = subset.clone();
            }
        }
    }

    best_subset
}