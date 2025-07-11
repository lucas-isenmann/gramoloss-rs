use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};






use std::env;

// use light_rs::{ check_3_rainbow_colorability, circulant_tournament, compute_dichromatric_number, compute_dichromatric_number_matrix, compute_triangles_poset, gen_grid_poset, greedy_dual_ds_v2, group_tournament, in_degree, in_degrees_sequence, is_2_independent_set, is_light, is_light_matrix, light_extend_v2, list_all_light_extension, print_adj, search6, semi_exact_dual_ds, to_dot, ug_tournament, BitwiseAdjacencyMatrix, MatrixGraph};

use light_rs::*;



fn light_extension(){
    let mut g =  BitwiseAdjacencyMatrix::new(6);
    g.add_arc(0, 1);
    g.add_arc(1, 2);
    g.add_arc(2, 0);
    g.add_arc(3, 4);
    g.add_arc(4, 5);
    g.add_arc(5, 3);
    list_all_light_extension(&mut g);

}


/**
 * 
 */
fn search_grid_poset(n: usize, m :usize){

    let file = File::open("2T3_light_extensions.csv").unwrap();
    let reader = BufReader::new(file);
    
    let mut matrix = Vec::new();
    
    for line in reader.lines() {
        if let Ok(l) = line {
            let row: Vec<usize> = l
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
            matrix.push(row);
        }
    }



    let mut g = BitwiseAdjacencyMatrix::new(3*n*m);
    gen_grid_poset(&mut g, n, m, &matrix);
    while is_light(&g) == false {
        gen_grid_poset(&mut g, n, m, &matrix);
    }
    
    g.to_dot();
    println!("nb arcs: {}", g.nb_arcs());
    println!("light: {}", is_light(&g));
    println!("chi: {}", compute_dichromatric_number(&g));
}



/**
 A light antichain is a set of disjoint triangles which are all together light.
 This program is a bruteforce algorithm testing all the ways 
 we can connect triangles together so that it is light 
 using the precomputed list of possible light connexions between 2 incomparable triangles
 */
fn search_light_antichain(n: usize){
    println!("search light antichain");

    let file = File::open("2T3_light_extensions.csv").unwrap();
    let reader = BufReader::new(file);
    
    let mut matrix = Vec::new();
    for line in reader.lines() {
        if let Ok(l) = line {
            let row: Vec<usize> = l
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
            matrix.push(row);
        }
    }

    let limit = 10000;
    let mut sieve_array = vec![true; limit + 1];
    let mut primes = vec![];
    
    // 0 and 1 are not prime numbers
    sieve_array[0] = false;
    sieve_array[1] = false;
    
    // Mark all even numbers as composite (except 2)
    for i in (4..limit + 1).step_by(2) {
        sieve_array[i] = false;
    }
    
    // Only need to check up to square root of limit
    let mut i = 3;
    while i * i <= limit {
        if sieve_array[i] {
            
            // Mark multiples as composite, starting from i*i
            for j in ((i * i)..limit + 1).step_by(i * 2) {
                sieve_array[j] = false;
            }
        }
        i += 2; // Skip even numbers
    }
    for i in 60..limit {
        if sieve_array[i] {
            primes.push(i);
        }
    }



    let s = 3*n;
    let mut g = BitwiseAdjacencyMatrix::new(s);
    let mut v: Vec<Vec<usize>> = vec![vec![0;3];n];


    for i in 0..n {
        for k in 0..3 {
            v[i][k] = k+3*(i);
        }
    }

    for i in 0..n {
        g.add_arc(v[i][0], v[i][1]);
        g.add_arc(v[i][1], v[i][2]);
        g.add_arc(v[i][2], v[i][0]);
    }


    let mut todo = vec![];
    for i in 0..n {
        for j in i+1..n {
            todo.push((i,j,0));
        }
    }
    println!("{todo:?}");
    println!("nb of connections: {}", todo.len());


    let mut done = vec![];
    let mut c = 0;
    
    loop {
        if let Some((i,j,r)) = todo.pop(){
            let vtriangles = vec![v[i][0], v[i][1], v[i][2], v[j][0], v[j][1], v[j][2]];
            let rp = (r*primes[i*n+j])% matrix.len();
            for k in 0..9 {
                g.add_arc(vtriangles[matrix[rp][2*k]], vtriangles[matrix[rp][2*k+1]]);
            }

            done.push((i,j,r));
        } 
        else {
            // End of branch
            if is_light(&g) {
                c += 1;
                if check_3_rainbow_colorability(&g) == false {
                    println!("bug")
                }
                // println!("{c} {}", check_3_rainbow_colorability(&g));
                // let chi =  bin_adj_matrices::compute_dichromatric_number(&g);
                // if chi >= 4{
                //     println!("chi: {chi}",);
                // }
                // println!("light {done:?}");
            }
            let mut finito = true;
            while let Some((i,j,r)) = done.pop(){
                let vtriangles = vec![v[i][0], v[i][1], v[i][2], v[j][0], v[j][1], v[j][2]];
                let rp = (r*primes[i*n+j])% matrix.len();
                for k in 0..9 {
                    g.delete_arc(vtriangles[matrix[rp][2*k]], vtriangles[matrix[rp][2*k+1]]);
                }
                

                if r+1 < matrix.len() {
                    finito = false;
                    todo.push((i,j,r+1));
                    break;
                } else {
                    todo.push((i,j,0));
                }
            }
            if finito {
                return;
            }
        }
    }
}


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
            

            if is_light_matrix(&adj){
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
        if is_light_matrix(&adj){
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
                let dn = compute_dichromatric_number_matrix(&adj);
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
        if is_light_matrix(&g){
            
            let chi = compute_dichromatric_number_matrix(&g);
            
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
        println!("{}", is_light_matrix(&g));
        let chi = compute_dichromatric_number_matrix(&g);
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
        if is_light(&g2) {
            let chi = compute_dichromatric_number(&g2);
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
    println!("is_light={}", is_light(&g));
    let chi = compute_dichromatric_number(&g);
    println!("chi={chi}");

    g.to_dot();
    compute_triangles_poset(&g);
}



fn check_hereditary_subtournament(){

    // 
    let mut g = BitwiseAdjacencyMatrix::new(11);
    g.add_arc(0, 1);
    g.add_arc(0, 2);
    g.add_arc(0 ,3);
    g.add_arc(1, 2);
    g.add_arc(1, 3);
    g.add_arc(2, 3);

    g.add_arc(4, 5);
    g.add_arc(4, 6);
    g.add_arc(4, 7);
    g.add_arc(5, 6);
    g.add_arc(5, 7);
    g.add_arc(6, 7);

    g.add_arc(8, 9);
    g.add_arc(9, 10);
    g.add_arc(10, 8);

    g.add_arc(10, 0);
    g.add_arc(1, 10);
    g.add_arc(10, 4);
    g.add_arc(5, 10);

    g.add_arc(9, 2);
    g.add_arc(3, 9);
    g.add_arc(9, 6);
    g.add_arc(7, 9);

    let mut g = BitwiseAdjacencyMatrix::new(9);
    g.add_arc(0, 1);
    g.add_arc(1, 2);
    g.add_arc(2, 0);

    g.add_arc(0, 3);
    g.add_arc(3, 4);
    g.add_arc(4, 0);

    g.add_arc(0, 5);
    g.add_arc(5, 6);
    g.add_arc(6, 0);

    g.add_cycle(vec![5,3,1]);
    g.add_cycle(vec![6,4,2]);

    g.add_cycle(vec![0,7,8]);


    // println!("{}", bin_adj_matrices::compute_dichromatric_number(&g));

    // g.print_in_degrees();

    list_all_light_extension(&mut g);
}







fn check_rainbow_random_grid_poset(n: usize, m :usize){

    let file = File::open("2T3_light_extensions.csv").unwrap();
    let reader = BufReader::new(file);
    
    let mut matrix = Vec::new();
    
    for line in reader.lines() {
        if let Ok(l) = line {
            let row: Vec<usize> = l
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
            matrix.push(row);
        }
    }

    let mut g = BitwiseAdjacencyMatrix::new(3*n*m);

    loop {
        gen_grid_poset(&mut g, n, m, &matrix);
        while is_light(&g) == false {
            gen_grid_poset(&mut g, n, m, &matrix);
        }
        let is_3rainbow = check_3_rainbow_colorability(&g);
        if is_3rainbow == false {
            g.to_dot();
            println!("3-rainbow: {}", is_3rainbow);
            println!("nb arcs: {}", g.nb_arcs());
            println!("light: {}", is_light(&g));
            println!("chi: {}", compute_dichromatric_number(&g));
            break;
        }
        println!("3-rainbow: {}", is_3rainbow);
        
    }
    
}



fn main() {
    let args: Vec<String> = std::env::args().collect();


    // let file_path = args.get(1).unwrap();
    // let g = BitwiseAdjacencyMatrix::from_dot_file(file_path).unwrap();

    // println!("n: {}", g.nb_vertices());
    // println!("m: {}", g.nb_arcs());
    // println!("connected: {}", g.is_strongly_connected());
    // println!("light: {}", is_light(&g));
    // println!("chi: {}", compute_dichromatric_number(&g));
    // return;

    let k: usize = args.get(1).unwrap().parse().unwrap();
    search6(k);
    return;

    // DS

    let g = MatrixGraph::load_from_edge_list_file(args.get(1).unwrap()).unwrap();
    let verbose: usize = args.get(2).unwrap().parse().unwrap();
    g.print_adj();
    g.print_degree_sequence();
    // g.print_degrees();



    
    for k in 0..3 {
        let subset = semi_exact_dual_ds(&g, k);
        println!("semi greedy kernel {k}: {} {}", subset.len(), is_2_independent_set(&g, &subset));
    }

    for k in 0..3 {
        let subset = semi_greedy_ds(&g, k);
        println!("semi greedy ds {k}: {}", subset.len());
    }

    

    let ds = min_dominating_set(&g, verbose);
    println!("dn: {:?}", ds.len());
    println!("mds: {:?}", ds);
    return ;

    
    
    
    // Parse the arguments into integers
    let n: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter valid numbers");
            return;
        }
    };
    
    let m: usize = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter valid numbers");
            return;
        }
    };

    // check_rainbow_random_grid_poset(n, m);
    // check_hereditary_subtournament();
    // search_light_antichain(n);
    // light_extension();
    // search_grid_poset(n, m);
    // print_triangles_poset();
    

    // for n in 15..20{
    //     search_ug_tournament(1+2*n);
    // }
    
    search3(17);

    // search2(9);

    return ;

    // search(11);

    // search2(11);

    

    if false
    {
        let p7 = circulant_tournament(vec![true, true, false]);
        let mut g = vec![vec![false; 6];6];
        for i in 0..6{
            for j in 0..6{
                g[i][j] = p7[i][j];
            }
        }
        println!("{}", is_light_matrix(&g));
        print_adj(&g);
        let chi = compute_dichromatric_number_matrix(&g);
        println!("chi: {chi}");
        println!("light critic: {}", is_light_critic(&g));
    }

    if false 
    {
        let g = circulant_tournament(vec![true, true, false]);
        println!("{}", is_light_matrix(&g));
        print_adj(&g);
        let chi = compute_dichromatric_number_matrix(&g);
        println!("chi: {chi}");
        println!("light critic: {}", is_light_critic(&g));
    }

    if false {
        let g = ug_tournament(11, &HashSet::from([3,6]));
        println!("{}", is_light_matrix(&g));
        print_adj(&g);
        let chi = compute_dichromatric_number_matrix(&g);
        println!("{chi}");
    }
    
}
