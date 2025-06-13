use std::{collections::HashSet, ops::Index};

use crate::{semi_greedy_ds, MatrixGraph};

use super::{greedy_ds, greedy_ds_v2, greedy_lower_bound_ds_v1, extend_greedy_2is, semi_exact_dual_ds};

/*
Unit test:

test1: 3
bremen_20: 9


*/



fn aux_min_dominating_set(
    g: &MatrixGraph,
    subset: &mut HashSet<usize>,
    to_dominate: &Vec<usize>,
    choosable: &mut Vec<usize>,
    current_min: &mut HashSet<usize>,
    kernel: & Vec<usize>,
    kernel_domination: &mut Vec<bool>,
    nb_undominated_kernel: &mut usize,
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
                println!("Better DS: {} >= {lower_bound}", subset.len());
            }
            *current_min = subset.clone();
        }
        return;
    }

    if choosable.is_empty() {
        return;
    }

    if subset.len() + *nb_undominated_kernel >= current_min.len() {
        return;
    }

    if current_min.len() == lower_bound {
        return;
    }

    // Pivot choice
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
            // println!("pivot: {v} utility: {best_utility}");
        }

        // Insert v in current subset
        let mut new_to_dominate = Vec::new();
        for &x in to_dominate {
            if g.has_edge(v, x)== false && x != v {
                new_to_dominate.push(x);
            }
        }

        choosable.remove(best);
        subset.insert(v);

        // Reduction
        let mut excluded = vec![];
        for x in choosable.iter() {
            if g.has_edge(*x, v) {
                let mut x_undom_neighbors = vec![];
                for z in new_to_dominate.iter() {
                    if g.has_edge(*x, *z) {
                        x_undom_neighbors.push(*z);
                    }
                }
                for y in choosable.iter() {
                    if g.has_edge(*x, *y) {
                        let mut is_better = true;
                        for z in x_undom_neighbors.iter() {
                            if g.has_edge(*y, *z) == false {
                                is_better = false;
                                break;
                            }
                        }
                        if is_better {
                            excluded.push(*x);
                            break;
                        }
                    }
                }
            }
        }

        for &x in excluded.iter() {
            for i in 0..choosable.len() {
                if choosable[i] == x {
                    choosable.remove(i);
                    break;
                }
            }
        }

       

        // Update lower bound (with kernel domination)
        let mut dom = None;
        for (i, &x) in kernel.iter().enumerate(){
            if (v == x || g.has_edge(x, v)) && kernel_domination[i] == false {
                kernel_domination[i] = true;
                *nb_undominated_kernel -= 1;
                dom = Some(i);
                break;
            }
        }

        aux_min_dominating_set(
            g,
            subset,
            &new_to_dominate,
            choosable,
            current_min,
            kernel,
            kernel_domination,
            nb_undominated_kernel,
            lower_bound,
            depth+1,
            verbose
        );


        // Cancel modifications
        if let Some(i) = dom {
            kernel_domination[i] = false;
            *nb_undominated_kernel += 1;
        }

        subset.remove(&v);
        for x in excluded {
            choosable.push(x);
        }


        if current_min.len() > lower_bound {
            aux_min_dominating_set(
                g,
                subset,
                to_dominate,
                choosable,
                current_min,
                kernel,
                kernel_domination,
                nb_undominated_kernel,
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

    let dual1 = semi_exact_dual_ds(g, 2);
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

    let kernel: Vec<usize> = dual1.iter().map(|x| *x).collect();
    let mut kernel_domination = vec![false; kernel.len()];
    let mut nb_undominated_kernel = kernel.len();


    let to_dominate: Vec<usize> = (0..n).collect();
    let mut current_min = semi_greedy_ds(&g, 2);
    let mut choosable: Vec<usize> = (0..n).collect();

    if verbose >= 1 {
        println!("################# Compute Min Dominating Set");
        println!("Graph size: n={} m={}", g.nb_vertices(), g.nb_edges());
        println!("lower_bound1: {}", dual1.len());
        println!("kernel: {dual1:?}");
        println!("to_dom: {to_dominate:?}");
        println!("upper_bound: {current_min:?}");
        println!("upper bound: {}", current_min.len());
        println!("choosable: {choosable:?}");
    }


    

    aux_min_dominating_set(
        g,
        &mut HashSet::new(),
        &to_dominate,
        &mut choosable,
        &mut current_min,
        &kernel,
        &mut kernel_domination,
        &mut nb_undominated_kernel,
        kernel.len(),
        0,
        verbose
    );
    current_min
}


pub fn domination_number(g: &MatrixGraph) -> usize {
    min_dominating_set(g, 0).len()
}