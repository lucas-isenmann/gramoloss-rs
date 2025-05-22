use std::collections::HashSet;

use crate::MatrixGraph;

use super::{greedy_ds, greedy_ds_v2, greedy_lower_bound_ds_v1, extend_greedy_2is, semi_exact_dual_ds};




fn aux_min_dominating_set(
    g: &MatrixGraph,
    subset: &mut HashSet<usize>,
    to_dominate: &Vec<usize>,
    choosable: &mut Vec<usize>,
    current_min: &mut HashSet<usize>,
    lower_bound: usize,
    depth: usize,
    verbose: usize
)  {

    if verbose >= 2 {
        println!("--------- AUX {depth}");
        println!("subset: {subset:?}");
        println!("to_dom: {to_dominate:?}\ncurrent_mds: {current_min:?}\n choosable: {choosable:?}");
    }
    

    if to_dominate.is_empty() {
        if subset.len() < current_min.len() {
            if verbose >= 1 {
                println!("BEST: {lower_bound} <= {}", subset.len());
            }
            *current_min = subset.clone();
        }
        return;
    }

    if choosable.is_empty() {
        return;
    }

    if subset.len() + 1 >= current_min.len() {
        return;
    }

    if current_min.len() == lower_bound {
        return;
    }

    let mut best_index = None;
    let mut best_utility = 0;

    for (i, &v) in choosable.iter().enumerate() {
        let mut utility = 0;
        for &x in to_dominate {
            if g.has_edge(v, x) || x == v {
                utility += 1;
            }
        }
        if utility > best_utility {
            best_utility = utility;
            best_index = Some(i);
        }
    }



    if let Some(best) = best_index {
        let v = choosable[best];
        if verbose >= 2 {
            println!("pivot: {v} utility: {best_utility}");
        }

        let mut new_to_dominate = Vec::new();
        for &x in to_dominate {
            if g.has_edge(v, x)== false && x != v {
                new_to_dominate.push(x);
            }
        }

        choosable.remove(best);

            
        subset.insert(v);
        aux_min_dominating_set(
            g,
            subset,
            &new_to_dominate,
            choosable,
            current_min,
            lower_bound,
            depth+1,
            verbose
        );

        subset.remove(&v);
        if current_min.len() > lower_bound {
            aux_min_dominating_set(
                g,
                subset,
                to_dominate,
                choosable,
                current_min,
                lower_bound,
                depth+1,
                verbose
            );
        }

        choosable.push(v);
    }

}






pub fn min_dominating_set(g: &MatrixGraph, verbose: usize) -> HashSet<usize> {
    let n = g.nb_vertices();

    let dual1 = semi_exact_dual_ds(g, 0);
    let mut is_dominating = true;
    for v_id in 0..n {
        let mut dominated = false;
        if dual1.contains(&v_id) {
            continue;
        }
        for &v_neighbor in &g.get_neighbors(v_id) {
            if dual1.contains(&v_neighbor) {
                dominated = true;
                break;
            }
        }
        if !dominated {
            is_dominating = false;
            break;
        }
    }

    if is_dominating {
        return dual1;
    }

    let dual0 = greedy_lower_bound_ds_v1(g);



    let to_dominate: Vec<usize> = (0..n).collect();
    let mut current_min = greedy_ds(&g);
    let mut current_min2 = greedy_ds_v2(&g);
    let mut choosable: Vec<usize> = (0..n).collect();

    if verbose >= 1 {
        println!("#################");
        println!("n= {} m= {}", g.nb_vertices(), g.nb_edges());
        println!("Init");
        println!("lower_bound0: {}", dual0.len());
        println!("lower_bound1: {}", dual1.len());
        println!("2IS: {dual1:?}");
        println!("to_dom: {to_dominate:?}");
        println!("upper_bound: {current_min:?}");
        println!("upper bound: {}", current_min.len());
        println!("upper bound2: {current_min2:?}");
        println!("upper bound2: {}", current_min2.len());
        println!("choosable: {choosable:?}");
    }
    

    aux_min_dominating_set(
        g,
        &mut HashSet::new(),
        &to_dominate,
        &mut choosable,
        &mut current_min,
        dual1.len(),
        0,
        verbose
    );
    current_min
}


pub fn domination_number(g: &MatrixGraph) -> usize {
    min_dominating_set(g, 0).len()
}