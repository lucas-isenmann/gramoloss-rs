use std::collections::HashSet;

use adj_matrices::print_adj;
use degrees::{in_degree, in_degrees_sequence};
use dichromatic_number::{compute_dichromatric_number, to_dot};
use lightness::{is_light, is_light_critic};
use search::search2;
use tournaments_generators::{circulant_tournament, ug_tournament};

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


fn main() {
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
        let g = ug_tournament(11, HashSet::from([3,6]));
        println!("{}", is_light(&g));
        print_adj(&g);
        let chi = dichromatic_number::compute_dichromatric_number(&g);
        println!("{chi}");
    }
    
}
