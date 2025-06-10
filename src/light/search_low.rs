use crate::{compute_dichromatric_number, is_light, is_local_light, BitwiseAdjacencyMatrix};




/// Insert x in t( u->v ).
/// Return the index where it has been  inserted and None otherwise .
/// t(u,v) is supposed to be a linear ordering of the mixed neighborhood of uv
fn insert_in_partial_order(g: &mut BitwiseAdjacencyMatrix,t: &mut Vec<Vec<Vec<usize>>>, x: usize, u: usize, v: usize) -> Option<usize>{
    let mut insertion_index = None;
    for (index, &y) in t[u][v].iter().enumerate() {
        if insertion_index == None {
            if g.has_arc(x, y) {
                insertion_index = Some(index);
            }
        } else if g.has_arc(y, x) {
            return None;
        }
    }
    
    if let Some(index) = insertion_index {
        t[u][v].insert(index, x);
        return Some(index);
    } else {
        t[u][v].push(x);
        return Some(t[u][v].len()-1);
    }
}

///
/// - t: ordered mixed neighbors (t[i][j] is the mixed neighborhood of ij)
fn compute_partial_orders(g: &mut BitwiseAdjacencyMatrix, t: &mut Vec<Vec<Vec<usize>>>, k: usize) -> bool{

    let mut in_neighbors = vec![];
    let mut out_neighbors = vec![];
    for i in 0..k {
        if g.has_arc(i, k) {
            in_neighbors.push(i);
        } else if g.has_arc(k, i){
            out_neighbors.push(i);
        }
    }

    for &u in in_neighbors.iter() {
        // compute t[u][k]
        t[u][k].clear();

        for &v in out_neighbors.iter() {
            if g.has_arc(v, u) {
                // add v in t[u][k]
                if insert_in_partial_order(g, t, v, u, k) == None {
                    return false;
                }
            }
        }
    }

    for &v in out_neighbors.iter() {
        // compute t[k][v]
        t[k][v].clear();

        for &u in in_neighbors .iter() {
            if g.has_arc(v, u) {
                // add v in t[k][v]
                if insert_in_partial_order(g, t, u, k, v) == None {
                    return false;
                }
            }
        }
    }


    true
}



fn init_partial_orders(g: &mut BitwiseAdjacencyMatrix, t: &mut Vec<Vec<Vec<usize>>>, k: usize) -> bool{

    for x in 0..k {
        for y in 0..k {
            if y != x && g.has_arc(x, y) {
                for z in 0..k {
                    if g.has_arc(y, z) && g.has_arc(z, x) {
                        if insert_in_partial_order(g, t, z, x,y) == None {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}


/// return true if finito
fn backtrack(g: &mut BitwiseAdjacencyMatrix,
             t: &mut Vec<Vec<Vec<usize>>>,
             done: &mut Vec<(usize, usize, bool)>, 
             to_do: &mut Vec<(usize, usize, bool)>,
             added: &mut Vec<(usize, usize, usize, usize, usize, usize)>,
            un_degree: &mut Vec<usize>) -> bool {
    
    // println!("backtrack");
    let mut finito = true;
    while let Some((u, v, b)) = done.pop() {
        // println!("undo {u} {v} {b}");
        
        if b {
            g.delete_arc(u, v);
            un_degree[v] += 1;
            while let Some((x,y,a,i,b,c)) = added.pop(){
                if x == u && y == v {
                    // println!("undo added({x} {y} {a} {i} {b} {c})");
                    let w= t[b][c].remove(i);
                } else {
                    added.push((x,y,a,i,b,c));
                    break;
                }
            }
            // println!("{added:?}");
            finito = false;
            to_do.push((u, v, false));
            break;
        } else {
            g.delete_arc(v, u);
            un_degree[v] += 1;

            while let Some((x,y,a,i,b,c)) = added.pop(){
                if x == u && y == v {
                    // println!("undo added({x} {y} {a} {i} {b} {c})");
                    let w= t[b][c].remove(i);
                    
                } else {
                    added.push((x,y,a,i,b,c));
                    break;
                }
            }

            to_do.push((u, v, true));
        }
        // println!("{added:?}");
    }
    let b = done.len() == 0 && finito;
    if b {
        
        
    }
    b
}


fn print_t(g: &BitwiseAdjacencyMatrix, n: usize, t: &Vec<Vec<Vec<usize>>>){
    for u in 0..n {
        for v in 0..n {
            if g.has_arc(u, v) {
                println!("t[{u},{v}] {:?}", t[u][v]);
            }
        }
    }
}

fn print_mixed_neighbors(g: &BitwiseAdjacencyMatrix, x: usize, y: usize){
    if g.has_arc(x, y){
        let mut l = vec![];
        for z in 0..g.nb_vertices() {
            if g.has_arc(y, z) && g.has_arc(z, x) {
                l.push(z);
            }
        }
        println!("N[{x} {y}] : {l:?}");
    } else {
        std::process::exit(1);
    }
}


pub fn light_extend_v2(g: &mut BitwiseAdjacencyMatrix){
    let n = g.nb_vertices();
    println!("Extension Init==============");
    println!("n={n}");

    g.to_dot();

    let mut un_degree = vec![0;n]; // un_degree[v] = number of neighbors (in and out) < v
    for v in 0..n {
        un_degree[v] = v;
        for x in 0..v {
            if g.has_arc(v, x) || g.has_arc(x, v){
                un_degree[v] -= 1;
            }
        }
    }
    let mut t = vec![vec![vec![];n];n];

    let mut to_do = vec![]; // (u,v,b) maintain that u < v
    let mut done = vec![];
    let mut added = vec![];
    for u in 0..n {
        for v in 0..u {
            if !g.has_arc(u, v) && !g.has_arc(v, u) {
                to_do.insert(0,(v, u, true));
            }
        }
    }

    if let Some((_,v,_)) = to_do.last(){
        if init_partial_orders(g, &mut t, *v) == false {
            return;
        }
    }

    print_t(g, n, &t);

    println!("to_do: {to_do:?}");

    let mut k = 0;
    let mut nb_extensions = 0;

    println!("Extension go =========="); 

    loop {
        // we suppose we have u < v (dir = true means we add u -> v and false, v -> u)
        while let Some((u, v, dir)) = to_do.pop() {
            // println!("BRANCH: {u} {v} {dir}");
            // println!("un_degree[{v}]= {}",un_degree[v]);

            if k != v {
                k = v;

                let lightness = compute_partial_orders(g, &mut t, v);
                // print_t(g, k+1, &t);

                if !lightness {
                    done.push((u,v,dir));
                    if backtrack(g, &mut t, &mut done, &mut to_do, &mut added, &mut un_degree) {
                        return;
                    }
                    continue;
                }
            } 

            let mut is_ok = true;
            if dir {
                g.add_arc(u, v);
                un_degree[v] -= 1;
                // add u to t[v][x] for every x in N+(v)
                for x in 0..v {
                    if x!= u && g.has_arc(v, x) && g.has_arc(x, u) {
                        let b1 = insert_in_partial_order(g, &mut t, x, u, v);
                        
                        if let Some(i1) = b1 {
                            // println!("add ({u} {v} {x} {i1} {u} {v})");
                            added.push((u,v, x, i1, u, v));
                        } else {
                            is_ok = false;
                            break;
                        }

                        let b2 = insert_in_partial_order(g, &mut t, u, v, x);
                        if let Some(i2) = b2 {
                            // println!("add ({u} {v} {u} {i2} {v} {x})");
                            added.push((u,v, u, i2, v, x)); 
                        } else {
                            is_ok = false;
                            break;
                        }

                        // let b3 = insert_in_partial_order(g, &mut t, v, x, u);

                    }
                }

            }
            else {
                g.add_arc(v, u);
                un_degree[v] -= 1;
                // add u to t[v][u] for every x in N-(v)
                for x in 0..v {
                    if x != u && g.has_arc(x, v) && g.has_arc(u, x) {
                        let b1 = insert_in_partial_order(g, &mut t, x, v, u);
                        if let Some(i1) = b1 {
                            // println!("add ({u} {v} {x} {i1} {v} {u})");
                            added.push((u,v, x, i1, v, u));

                        } else {
                            is_ok = false;
                            break;
                        }
                        
                        let b2 = insert_in_partial_order(g, &mut t, u, x, v);
                        if let Some(i2) = b2{
                                // println!("add ({u} {v} {u} {i2} {x} {v})");
                                added.push((u,v, u, i2, x, v));

                        } else {
                            is_ok = false;
                            break;
                        }

                        // let b2 = insert_in_partial_order(g, &mut t, v, u, x);

                    }
                }
            }

            if un_degree[v] == 0 {
                // println!("vertex {v} is saturated");
                for x in 0..v {
                    for y in 0..v {
                        if g.has_arc(x, y) && g.has_arc(y, v) && g.has_arc(v, x) {
                            if let Some(i) = insert_in_partial_order(g, &mut t, v, x, y) {
                                
                                added.push((u,v, v,i, x,y ))
                            } else {
                                is_ok = false;
                                break;
                            }
                        }
                    }
                }

                
            }

            // print_t(g, v+1, &t);

            done.push((u,v,dir));
            // println!("{done:?}");
            // println!("is_ok: {is_ok}");
            if is_ok == false {
                if backtrack(g, &mut t, &mut done, &mut to_do, &mut added, &mut un_degree) {
                    println!("========== END");
                    println!("nb extensions: {nb_extensions}");
                    return;
                }
            } 

            
        }

        nb_extensions += 1;

        

        if  compute_dichromatric_number(g) >= 3{
            println!("---LIGHT EXTENSION {nb_extensions}");
            println!("chi={}", compute_dichromatric_number(g));
            println!("m={}", g.nb_arcs());
            println!("islight: {}", is_light(g));
            println!("Added arcs: {done:?}");
            g.to_dot();
            for u in 0..n {
                for v in 0..n {
                    if g.has_arc(u, v) {
                        println!("t[{u},{v}] {:?}", t[u][v]);
                    }
                }
            }
        }
        

        if backtrack(g, &mut t, &mut done, &mut to_do, &mut added, &mut un_degree) {
            println!("========== END");
            println!("nb extensions: {nb_extensions}");
            return
        }

    }

}




/// ##########################""
/// Strategy: 
/// Start with a triangle uvw (012)
/// - N[uvw]- (called left) is transitive
/// - N[uvw]+ (called right) is transitive
/// - N[uv] is transitive
/// - N[vw] also
/// - N[wu] also
/// - We know that N[uvw]- => N[uvw]+
/// 
/// Try different size for these 5 sets.
/// Try to extend the graph with all possible arcs to make it light and have chromatic number 4
/// 
/// 
pub fn search6(n: usize){
    println!("search 6");
    let mut g = BitwiseAdjacencyMatrix::new(5*n+3);

    g.add_cycle(vec![0,1,2]);

    let left = (3..3+n).collect();
    let right = (3+n..3+2*n).collect();
    let t01 = (3+2*n..3+3*n).collect();
    let t12 = (3+3*n..3+4*n).collect();
    let t20 = (3+4*n..3+5*n).collect();

    g.make_transitive(&left);
    g.make_transitive(&right);
    g.make_transitive(&t01);
    g.make_transitive(&t12);
    g.make_transitive(&t20);

    g.add_arcs(&left, &vec![0,1,2]);
    g.add_arcs(&vec![0,1,2], &right);
    g.add_arcs(&left, &right);

    g.add_arcs( &t01, &vec![0]);
    g.add_arcs(&vec![1], &t01);

    g.add_arcs( &t12, &vec![1]);
    g.add_arcs(&vec![2], &t12);

    g.add_arcs( &t20, &vec![2]);
    g.add_arcs(&vec![0], &t20);


    // add arcs between the exteriors
    // let indices = vec![3,9,12,4,15,10,5,13,16,6,11,14,7,17,8];
    // let indices = vec![9,12,3,15,4,10,5,13,6,16,7,11,8,14,17];
    // for i in 0..indices.len() {
    //     for j in 0..i {
    //         if i-j == 1  {
    //             g.add_arc(indices[i], indices[j]);
    //         } else {
    //             g.add_arc(indices[j], indices[i]);
    //         }
    //     }
    // }



    
    println!("n={}", g.nb_vertices());
    println!("m={}", g.nb_arcs());
    println!("indegrees:");
    g.print_in_degrees();



    light_extend_v2(&mut g);

    return;

    // List all light extensions
    let mut nb_extensions = 0;
    let n = g.nb_vertices();

    let mut todo = vec![];
    for i in 0..n {
        for j in i+1..n {
            if g.has_arc(i, j) == false && g.has_arc(j,i) == false {
                todo.push((i,j));
            }
        }
    }
    println!("nb arcs to choose: {}", todo.len());

    let mut done = vec![];

    loop {
        if let Some((i,j)) = todo.pop(){
            g.add_arc(i, j);
            if is_local_light(&g, i, j) {
                done.push((i,j));
            } else {
                done.push((i,j));
                let mut finito = true;
                while let Some((i,j)) = done.pop(){
                    g.delete_arc(i, j);
                    if i < j {
                        finito = false;
                        todo.push((j,i));
                        break;
                    } else {
                        todo.push((j,i));
                    }
                }
                if finito {
                    println!("{nb_extensions}");
                    return;
                }
            }
        } 
        else {
            // if true || is_light(&g)  {
            nb_extensions += 1;
                let chi =  compute_dichromatric_number(&g) ;
                if chi >= 3 {
                    println!("chi= {chi}");
                    println!("done: {done:?}");

                }
            // }
            let mut finito = true;
            while let Some((i,j)) = done.pop(){
                g.delete_arc(i, j);
                if i < j {
                    finito = false;
                    todo.push((j,i));
                    break;
                } else {
                    todo.push((j,i));
                }
            }
            if finito {
                println!("{nb_extensions}");
                return;
            }
        }
    }
    

}
