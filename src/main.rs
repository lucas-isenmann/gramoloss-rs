use std::collections::HashSet;

use adj_matrices::print_adj;
use bin_adj_matrices::{ search3, BitwiseAdjacencyMatrix};
use degrees::{in_degree, in_degrees_sequence};
use dichromatic_number::{compute_dichromatric_number, to_dot};
use lightness::{is_light, is_light_critic};
use search::search2;
use tournaments_generators::{circulant_tournament, group_tournament, ug_tournament};
use triangles_poset::compute_triangles_poset;

mod triangles_poset;
mod dichromatic_number;
mod adj_matrices;
mod lightness;
mod tournaments_generators;
mod search;
mod degrees;
mod bin_adj_matrices;


/**
 * search a light strongly connected tournament of dichromatic number >= 3
 */
fn search(n: usize){
    let mut adj = vec![vec![false; n];n];

    for  i in 0..n {
        adj[i][(i+1)%n] = true;
    }

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
            

            if is_light(&adj){
                done.push((i,j));
                continue;
            } else {
                adj[i][j] = false;
                if i < j {
                    adj[j][i] = true;
                    done.push((j,i));
                } else {
                    todo.push((i,j));
                    while let Some((x,y)) = done.pop(){
                        adj[x][y] = false;
                        if x < y {
                            todo.push((y,x));
                            break;
                        }
                        else {
                            todo.push((y,x))
                        }
                    }
                }
            }
        }

        // Light tournament found
        if is_light(&adj){
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
                        println!("twin {u} {v}");
                        is_twin = true;
                        break;
                    }
                }
            }


            if is_twin == false {
                let dn = compute_dichromatric_number(&adj);
                if dn >= 2 {
                    let mut is_increasing = true;
                    for k in 1..n{
                        if in_degree(&adj, 0) > in_degree(&adj, k){
                            is_increasing = false;
                            break;
                        }
                    }
                    if is_increasing {
                        println!("{dn} {done:?}");
                        println!("{:?}", in_degrees_sequence(&adj));
                        to_dot(&adj);
                        c += 1;
                    }
                    
                }
                


            }
            
        }
        
        // println!("light found");
        // println!("done= {done:?}");
        // println!("{}", dichromatic_number::compute_dichromatric_number(&adj));

        // Backtrack
        let mut finito = true;
        while let Some((x,y)) = done.pop(){
            adj[x][y] = false;
            if x < y {
                finito = false;
                todo.push((y,x));
                break;
            }
            else {
                todo.push((y,x))
            }
        }
        if done.len() == 0 && finito{
            println!("Number of light tournaments with Chi >= 3 and without twins: {c}");
            return;
        }
    }
    

}


fn group_tournament_test() {
    let n = 9;
    let m = 5;

    let mut l = vec![];

    for i in 0..n {
        for j in 0..m {
            if i == 0 && j == 0{
                continue;
            }
            if !l.contains(&((n-i)%n,(m-j)%m)) {
                l.push((i,j))
            }
        }
    }
    println!("{l:?}");
    let k = l.len();
    for a in 0..(1 << k) {
        let mut positives = vec![];
        for b in 0..k{
            if a & (1 << b) == 0 {
                positives.push(l[b])
            } else {
                let (x,y) = l[b];
                positives.push(((n-x)%n, (m-y)%m));
            }
        }
        let g = group_tournament(n, m, positives.clone());
        if is_light(&g){
            
            let chi = dichromatic_number::compute_dichromatric_number(&g);
            
            if chi >= 3{
                println!("{positives:?}");
                println!("chi: {chi}");
            }
        }
        
    }

    

    // let g = group_tournament(5, 3, vec![(1,0), (2,0), (0,1),(4,2), (3,2),(4,1),(2,2)]);
    // print_adj(&g);
    // println!("---");
    // println!("{}", is_light(&g));
    // let chi = dichromatic_number::compute_dichromatric_number(&g);
    // println!("chi: {chi}");

    if (false) {
        let g = group_tournament(3, 3, vec![(0,2), (2,0), (1,1), (1,2)]);
        print_adj(&g);
        println!("---");
        println!("{}", is_light(&g));
        let chi = dichromatic_number::compute_dichromatric_number(&g);
        println!("chi: {chi}");
    }
    

}


fn search_ug_tournament(n: usize){
    let k = n-1;
    println!("{n} {k}");

    for a in 0..(1 << k){
        let mut gaps = HashSet::new();
        for i in 0..k {
            if a & (1 << i) != 0{
                gaps.insert(i+1);
            }
        }

        let g = ug_tournament(n, &gaps);
        let mut g2 = BitwiseAdjacencyMatrix::new(n);
        for i in 0..n{
            for j in 0..n {
                if g[i][j] {
                    g2.add_arc(i, j);
                }
            }
        }
        if bin_adj_matrices::is_light(&g2) {
            let chi = bin_adj_matrices::compute_dichromatric_number(&g2);
            if chi >= 4 {
                println!("\t{gaps:?}");
                g2.to_dot();
                println!("\tchi={chi}");
            }
        }
        // if is_light(&g){
            
        //     let chi = dichromatic_number::compute_dichromatric_number(&g);
            
        //     if chi >= 3 {
        //         println!("\t{gaps:?}");
        //         println!("\tchi={chi}");
        //     }
            
        // }
        
    }
    
    
}


fn print_triangles_poset(){
    let g = BitwiseAdjacencyMatrix::from_adj_matrix(&circulant_tournament(vec![true, true, true, true]));
    let g = BitwiseAdjacencyMatrix::from_dot_file("t13.dot").unwrap();
    println!("is_light={}", bin_adj_matrices::is_light(&g));
    let chi = bin_adj_matrices::compute_dichromatric_number(&g);
    println!("chi={chi}");

    g.to_dot();
    compute_triangles_poset(&g);
}


fn main() {

    print_triangles_poset();
    

    // for n in 15..20{
    //     search_ug_tournament(1+2*n);
    // }
    
    // search3(13);

    // search2(9);

    return ;

    // search(11);

    search2(11);

    

    if false
    {
        let p7 = circulant_tournament(vec![true, true, false]);
        let mut g = vec![vec![false; 6];6];
        for i in 0..6{
            for j in 0..6{
                g[i][j] = p7[i][j];
            }
        }
        println!("{}", is_light(&g));
        print_adj(&g);
        let chi = dichromatic_number::compute_dichromatric_number(&g);
        println!("chi: {chi}");
        println!("light critic: {}", is_light_critic(&g));
    }

    if false 
    {
        let g = circulant_tournament(vec![true, true, false]);
        println!("{}", is_light(&g));
        print_adj(&g);
        let chi = dichromatic_number::compute_dichromatric_number(&g);
        println!("chi: {chi}");
        println!("light critic: {}", is_light_critic(&g));
    }

    if false {
        let g = ug_tournament(11, &HashSet::from([3,6]));
        println!("{}", is_light(&g));
        print_adj(&g);
        let chi = dichromatic_number::compute_dichromatric_number(&g);
        println!("{chi}");
    }
    
}
