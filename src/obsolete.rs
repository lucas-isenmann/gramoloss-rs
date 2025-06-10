
// This tournament is light and the rainbow coloring on 012 and 345 cannot be extended to 678
// The considered coloring is 03 in red, 14 in blue, 45 in green
fn check_hereditary_subtournament2(){
    let mut g = BitwiseAdjacencyMatrix::new(9);
    g.add_cycle(vec![0,1,2]);
    g.add_cycle(vec![3,4,5]);
    g.add_cycle(vec![6,7,8]);

    g.add_cycle(vec![1,4,7]);
    g.add_cycle(vec![1,4,8]);
    g.add_cycle(vec![2,5,7]);
    g.add_cycle(vec![2,5,8]);

    g.add_arc(0, 3);
    g.add_arc(3, 6);
    g.add_arc(0, 6);

    // println!("{}", bin_adj_matrices::compute_dichromatric_number(&g));

    // g.print_in_degrees();

    list_all_light_extension(&mut g);
}











pub fn search4(n: usize){
    println!("search v4 : {n} vertices");
    let mut g = BitwiseAdjacencyMatrix::new(n);

    // We suppose we have an hamiltonian cycle
    for  i in 0..n {
        g.add_arc(i, (i+1)%n);
    }

    

    
    g.print_in_degrees();


    let mut todo = vec![];
    let mut order = vec![vec![false; n]; n];
    for  i in 2..n {
        for j in 0..i-1 {
            if j == 0 && i == n-1 {
                continue;
            }
            todo.push((j,i));
            order[j][i] = true;
            order[i][j] = false;

            if (i+j*n)%2 == 0{
                order[i][j] = !order[i][j];
                order[j][i] = !order[j][i];
            }
        }
    }

    let mut c = 0;



    let mut done = vec![];
    loop {
        while let Some((x,y)) = todo.pop(){
            let mut i = x;
            let mut j = y;
            if order[i][j] {
                i = y;
                j = x;
            }

            g.add_arc(i, j);

            let m = if i > j { i } else { j };

            let mut light = true;
            // Deductions
            // let mut deductions = vec![];
            let mut outvertices = vec![];

            for k in 0..m-1 {
                if g.has_arc(j, k) && g.has_arc(k, i) {
                    outvertices.clear();
                    for outv in 0..m-1 {
                        if g.has_arc(i,outv) && g.has_arc(j, outv) && g.has_arc(k, outv) {
                            outvertices.push(outv);                            
                        }
                    }
                    for inv in 0..m-1{
                        if g.has_arc(inv, i) && g.has_arc(inv, j) && g.has_arc(inv, k){
                            for &outv in outvertices.iter() {
                                if g.has_arc(outv, inv) {
                                    light = false;
                                    break;
                                }
                            }
                        }
                        if light == false {
                            break;
                        }
                    }
                }
                if light == false {
                    break;
                }
            }

            

            if light && check_0_is_min(&g) && g.has_twin() == false { // && is_local_light(&g, i, j) {
                done.push((x,y));
            } else {
                g.delete_arc(i, j);

                if x < y {
                    todo.push((y,x));
                } else {
                    // Backtrack
                    let mut finito = true;
                    todo.push((y,x));
                    while let Some((u,v)) = done.pop(){
                        g.delete_arc(u, v);
                        todo.push((v,u));

                        if u < v {
                            finito = false;
                            break;
                        }
                    }
                    if finito {
                        println!("Number of light tournaments with Chi >= 3 and without twins: {c}");
                        return;
                    }
                }
            }
        }

        let chi = compute_dichromatric_number(&g);

        if chi >= 3 && is_light(&g) {
            println!("---");
            println!("chi={chi} {done:?}");
            g.print_in_degrees();
            //to_dot(&adj);
            g.to_dot();
            c += 1;
        }

            

        // Backtrack
        let mut finito = true;
        while let Some((x,y)) = done.pop(){
            
            g.delete_arc(x, y);
            todo.push((y,x));

            if x < y {
                finito = false;
                break;
            }
        }
        if finito{
            println!("Number of light tournaments with Chi >= 3 and without twins: {c}");
            return;
        }
    }
    

}























pub fn get_light_conflict(g: &BitwiseAdjacencyMatrix) -> Option<(usize, usize, usize, usize, usize)> {
    let n = g.n;
    for u in 0..n {
        for v in 0..n {
            if g.has_arc(u, v ) {
                for a in 0..n {
                    if g.has_arc(a, u) && g.has_arc(v,a) {
                        for b in 0.. n {
                            if g.has_arc(b, u) &&  g.has_arc(v, b) && g.has_arc(a, b)  {
                                for c in 0..n{
                                    if g.has_arc(c, u) && g.has_arc(v, c) && g.has_arc(b, c) && g.has_arc(c, a) {
                                        return Some((u,v,a,b,c));
                                        // println!("light conflict: {u}-> {v}, ({a} -> {b} -> {c})");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}







pub fn search_by_deletion(n: usize){
    let mut rng = rand::rng();


    let mut squares = vec![false; n];
    for i in 0..n{
        squares[(i*i)%n] = true;
    }
    let mut adj = vec![];
    for i in 1..(n/2+1) {
        adj.push(squares[i])
    }
    println!("{squares:?}");
    println!("{adj:?}");


    let m = circulant_tournament(adj);
    println!("{}", m.len());


    loop {
        let mut g = BitwiseAdjacencyMatrix::new(n);
        let mut vertices = vec![0;n];
        for i in 0..n {
            vertices[i] = i;
        }

        for i in 0..n {
            for j in 0..n{
                if m[i][j] {
                    g.add_arc(i, j);
                }
            }
        }

        while let Some((u,v,a,b,c)) = get_light_conflict(&g) {
            // println!("{u} {v} {a} {b} {c}");
            let l = vec![u,v,a,b,c];
            let r = rng.random_range(0..l.len());
            let x = l[r];

            vertices.remove(x);

            g = BitwiseAdjacencyMatrix::new(vertices.len());
            for i in 0..vertices.len() {
                for j in 0..vertices.len() {
                    if m[vertices[i]][vertices[j]] {
                        g.add_arc(i, j);
                    }
                }
            }

        }

        let dichro = compute_dichromatric_number(&g);
        if dichro >= 3 && vertices.len() >= 10 {
            println!("n= {}", vertices.len());
            println!("chi= {}", dichro);
        }
    }

}






















pub fn check_all_light_extension(g: &mut BitwiseAdjacencyMatrix) {
    let n = g.n;
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
            done.push((i,j));
        } 
        else {
            if g.has_twin() == false && is_light(g) && g.is_strongly_connected()  {
                let chi =  compute_dichromatric_number(&g) ;
                if chi >= 3 {
                    println!("chi= {chi}")
                }
                // println!("light {done:?}");
            }
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
                return;
            }
        }
    }
}




/**
 Search a light tournament with n vertices 
 - strongly connected
 - has no twin
 with the triangle decomposition

 */
pub fn search_v5(n: usize){

    let mut g = BitwiseAdjacencyMatrix::new(n);
    let mut c = 0;

    g.add_cycle(vec![0,1,2]);


    // k vertices are 0 -> x -> 1
    for k in 1..(n-3) {

        for x in 3..3+k {
            g.add_arc(0, x);
            g.add_arc(x, 1);
        }
        for x in 3..3+k {
            for y in 3..x {
                g.add_arc(y, x);
            }
        }

        // j vertices are 1 -> x -> 2
        for j in 0..=k{
            if j+k+3 > n {
                break;
            }

            for x in 3+k..3+k+j {
                g.add_arc(1, x);
                g.add_arc(x, 2);
            }
            for x in 3+k..3+k+j {
                for y in 3+k..x {
                    g.add_arc(y, x);
                }
            }

            // i vertices are 2 -> x -> 0
            for i in 0..=j {
                if i+j+k+3 > n {
                    break;
                }

                for x in 3+k+j..3+k+j+i {
                    g.add_arc(2, x);
                    g.add_arc(x, 0);
                }
                for x in 3+k+j..3+k+j+i {
                    for y in 3+k+j..x {
                        g.add_arc(y, x);
                    }
                }

                //
                for l in 0..n {
                    if 3+i+j+k+l > n {
                        break;
                    }
                    for x in 3+i+j+k..3+i+j+k+l {
                        g.add_arc(x, 0);
                        g.add_arc(x, 1);
                        g.add_arc(x, 2);
                    }
                    
                    // The remaining vertices are dominated by T
                    let m = n - (3+i+j+k+l);

                    // Add the dominated vertices by T
                    for x in 3+i+j+k+l..3+i+j+k+l+m {
                        g.add_arc(0, x);
                        g.add_arc(1, x);
                        g.add_arc(2, x);
                    }
                    for x in 3+i+j+k+l..3+i+j+k+l+m {
                        for y in 3+i+j+k..3+i+j+k+l {
                            g.add_arc(y, x);
                        }
                    }

                    // Main
                    println!("{c} config: {i} {j} {k} {l} {m}");
                    c += 1;
                    check_all_light_extension(&mut g);

                    for x in 3+i+j+k+l..3+i+j+k+l+m {
                        for y in 3+i+j+k..3+i+j+k+l {
                            g.delete_arc(y, x);
                        }
                    }

                    for x in 3+i+j+k+l..3+i+j+k+l+m {
                        g.delete_arc(0, x);
                        g.delete_arc(1, x);
                        g.delete_arc(2, x);
                    }
                    //


                    for x in 3+i+j+k..3+i+j+k+l {
                        g.delete_arc(x, 0);
                        g.delete_arc(x, 1);
                        g.delete_arc(x, 2);
                    }

                }


                for x in 3+k+j..3+k+j+i {
                    for y in 3+k+j..x {
                        g.delete_arc(y, x);
                    }
                }
                for x in 3+k+j..3+k+j+i {
                    g.delete_arc(2, x);
                    g.delete_arc(x, 0);
                }
            }

            for x in 3+k..3+k+j {
                for y in 3+k..x {
                    g.delete_arc(y, x);
                }
            }
            for x in 3+k..3+k+j {
                g.delete_arc(1, x);
                g.delete_arc(x, 2);
            }
        }

        // Cancel k vertices
        for x in 3..3+k {
            for y in 3..x {
                g.delete_arc(y, x);
            }
        }
        for x in 3..3+k {
            g.delete_arc(0, x);
            g.delete_arc(x, 1);
        }
    }



}

