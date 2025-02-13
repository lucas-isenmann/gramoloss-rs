use std::{collections::HashMap, fs, path::Path};



pub struct BitwiseAdjacencyMatrix {
    n: usize,
    data: Vec<usize>,
    in_degree: Vec<usize>,
    out_degree: Vec<usize>,
    un_degree: Vec<usize>,
    powers_of_2: Vec<usize>,
}


impl BitwiseAdjacencyMatrix {
    pub fn new(n: usize) -> Self {
        let data = vec![0; n * n];
        let powers_of_2 = (0..n).map(|i| 1 << i).collect::<Vec<usize>>();

        BitwiseAdjacencyMatrix { n,
            data, 
            in_degree: vec![0; n], 
            out_degree: vec![0; n], 
            un_degree: vec![n-1; n], 
            powers_of_2 }
    }

    pub fn from_adj_matrix(adj: &Vec<Vec<bool>>) -> Self {
        let n = adj.len();
        let mut g = BitwiseAdjacencyMatrix::new(n);
        for i in 0..n{
            for j in 0..n {
                if adj[i][j] {
                    g.add_arc(i, j);
                }
            }
        }
        g
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn add_arc(&mut self, i: usize, j: usize) {
        self.data[i] |= self.powers_of_2[j];
        self.in_degree[j] += 1;
        self.out_degree[i] += 1;
        self.un_degree[j] -= 1;
        self.un_degree[i] -= 1;
    }

    pub fn delete_arc(&mut self, i: usize, j: usize) {
        self.data[i] &= !self.powers_of_2[j];
        self.in_degree[j] -= 1;
        self.out_degree[i] -= 1;
        self.un_degree[j] += 1;
        self.un_degree[i] += 1;
    }

    pub fn has_arc(&self, i: usize, j: usize) -> bool {
        (self.data[i] & self.powers_of_2[j]) != 0
    }

    pub fn in_degree(&self, i: usize) -> usize {
        self.in_degree[i]
    }

    pub fn print_in_degrees(&self) {
        let mut list = (0..self.n).map( | i| self.in_degree(i)).collect::<Vec<usize>>();
        println!("{:?}", list  );
        list.sort();
        println!("{:?}", list)
    
    }

    pub fn are_twin(&self, i: usize, j: usize) -> bool {
        (self.data[i] & !self.powers_of_2[j]) == (self.data[j]  & !self.powers_of_2[i] )
    }

    pub fn has_twin(&self) -> bool {
        for i in 0..self.n {
            if self.un_degree[i] == 0{
                for j in 0..i{
                    if self.un_degree[j] == 0 && self.are_twin(i, j) {
                        return true
                    }
                }
            }
            
        }
        false
    }

    pub fn print(&self) {
        let n = self.n;
        
        for i in 0..n {
            for j in 0..n {
                let p = 1 << j;
                let bit = if (self.data[i] & p) != 0 { "1" } else { "." };
                print!("{}", bit);
            }
            println!();
        }
    }

    pub fn to_dot(&self){
        let n = self.n;
        println!("digraph G {{");
        for i in 0..n {
            for j in 0..n {
                if self.has_arc(i, j) {
                    println!("{i} -> {j};")
                }
            }
        }
        println!("}}");
    
    }

    pub fn from_dot_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(path)?;

        let mut arcs = vec![];
        let mut n = 0;
        let mut indices = HashMap::new();

        
        // Split into lines and process each edge
        for line in contents.lines() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            // Remove trailing semicolon
            let line = line.trim_end_matches(';');
            
            // Split on arrow operator
            if let Some(pos) = line.find(" -> ") {
                let source = line[..pos].to_string();
                let target = line[pos + 4..].to_string();

                if indices.contains_key(&source) == false {
                    indices.insert(source.clone(), n);
                    n += 1;
                }
                if indices.contains_key(&target) == false {
                    indices.insert(target.clone(), n);
                    n += 1;
                }
                
                // Add nodes and edge
                arcs.push((source, target));
            }
        }


        let mut g = BitwiseAdjacencyMatrix::new(n);

        for (s,t) in arcs {
            let u: usize = s.parse().unwrap();
            let v: usize = t.parse().unwrap();
            g.add_arc(u, v);
        }


        // for (s, t) in arcs {
        //     if let Some(&u)  = indices.get(&s) {
        //         if let Some(&v) = indices.get(&t){
        //             g.add_arc(u, v);
        //         }
        //     }

        // }
        
        Ok(g)
    }

}




pub fn is_light(g: &BitwiseAdjacencyMatrix) -> bool {
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
                                        return false
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    true
}



pub fn is_local_light(g: &BitwiseAdjacencyMatrix, u: usize, v: usize) -> bool {
    let n = g.n;

    // Type 1 conflict with uv (u->v is an heavy arc)
    let mut vertices = vec![];
    for x in 0..n {
        if g.has_arc(x,u) && g.has_arc(v,x) {
            vertices.push(x);
        }
    }

    for &a in &vertices {
        for &b in &vertices {
            if g.has_arc(a,b) {
                for &c in &vertices {
                    if g.has_arc(b,c) && g.has_arc(c,a) {
                        // console.log("type 1")
                        return false;
                    }
                }
            }
        }
    }

    // Type 2: uv is in the triangle (search for w the third vertex of the triangle)
    for w in 0..n {
        if g.has_arc(v,w) && g.has_arc(w,u) {
            let mut dominated = vec![];
            for a in 0..n {
                if g.has_arc(v,a) && g.has_arc(u,a) && g.has_arc(w,a) {
                    dominated.push(a);
                }
            }
            for b in 0..n {
                if g.has_arc(b,v) && g.has_arc(b,u) && g.has_arc(b,w) {
                    for &a in &dominated {
                        if g.has_arc(a,b) {
                            // console.log("type 2")
                            return false;
                        }
                    }
                }
            }
        }
    }

    // Type 3: v is in the triangle and u is endvertex of the searched heavy arc
    for w in 0..n {
        if g.has_arc(v,w) && g.has_arc(u,w) {
            for x in 0..n {
                if g.has_arc(w,x) && g.has_arc(u,x) && g.has_arc(x,v) {
                    for y in 0..n {
                        if g.has_arc(v,y) && g.has_arc(x,y) && g.has_arc(w,y) && g.has_arc(y,u) {
                            // console.log("type 3")
                            return false;
                        }
                    }
                }
            }
        }
    }

    // Type 4: u is in the triangle and v is the start vertex of the searched heavy arc
    for w in 0..n {
        if g.has_arc(u,w) && g.has_arc(w,v) {
            for x in 0..n {
                if g.has_arc(w,x) && g.has_arc(x,v) && g.has_arc(x,u) {
                    for y in 0..n {
                        if g.has_arc(v,y) && g.has_arc(y,u) && g.has_arc(y,w) && g.has_arc(y,x) {
                            return false
                        }
                    }
                }
            }
        }
    }

    true
}






fn check_0_is_min(g: &BitwiseAdjacencyMatrix) -> bool {
    for k in 0..g.n-1 {
        if g.in_degree[g.n-1] > g.in_degree[k] + g.un_degree[k]{
            return false
        }
    }
    true
}



pub fn search3(n: usize){
    println!("search 3");
    let mut g = BitwiseAdjacencyMatrix::new(n);

    // We suppose we have an hamiltonian cycle
    for  i in 0..n {
        g.add_arc(i, (i+1)%n);
    }

    g.print_in_degrees();


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
            g.add_arc(i, j);

            if  check_0_is_min(&g) && g.has_twin() == false && is_local_light(&g, i, j) {
                done.push((i,j));
            } else {
                g.delete_arc(i, j);

                if i < j {
                    todo.push((j,i));
                } else {
                    // Backtrack
                    let mut finito = true;
                    todo.push((j,i));
                    while let Some((x,y)) = done.pop(){
                        g.delete_arc(x, y);
                        todo.push((y,x));

                        if x < y {
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

        // Light tournament found
        // let mut is_twin = false;
        // for i in 0..n {
        //     for j in 0..i{
        //         if g.are_twin(i, j) {
        //             is_twin = true;
        //             break;
        //         }
        //     }
        //     if is_twin {
        //         break;
        //     }
        // }

        // if is_twin == false {
            let chi = compute_dichromatric_number(&g);

            if chi >= 3 {
                println!("---");
                println!("chi={chi} {done:?}");
                g.print_in_degrees();
                //to_dot(&adj);
                g.to_dot();
                c += 1;
            }
            


        // }

            
        
        // println!("light found");
        // println!("done= {done:?}");
        // println!("indeg= {:?}", in_degrees_sequence(&adj));
        // println!("{}", dichromatic_number::compute_dichromatric_number(&adj));

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










// -----------------------------------------------------------
// DiChromatic Number






fn search_optimal_vertex(todo: &Vec<usize>, triangles: &Vec<Vec<usize>>, coloring: &Vec<usize>) -> usize {
    let mut count = vec![0; coloring.len()];
    let mut record = 0;
    let mut vertex = 0;

    for x in todo.iter() {
        let triangle = &triangles[*x];
        for &v in triangle {
            if coloring[v] == 0 {
                count[v] += 1;
                if count[v] > record {
                    record = count[v];
                    vertex = v;
                }
            }
        }
    }

    vertex
}



fn is_conflict(triangle: &Vec<usize>, coloring: &Vec<usize>) -> bool {
    coloring[triangle[0]] == coloring[triangle[1]] &&
    coloring[triangle[0]] == coloring[triangle[2]] &&
    coloring[triangle[0]] > 0
}

fn is_satisfied(triangle: &Vec<usize>, coloring: &Vec<usize>) -> bool {
    let mut color1 = 0;

    for &x in triangle.iter() {
        if coloring[x] > 0 {
            if color1 > 0 && coloring[x] != color1 {
                return true;
            } else if color1 == 0 {
                color1 = coloring[x];
            }
        }
    }
    false
}

fn nb_colored(triangle: &Vec<usize>, coloring: &Vec<usize>) -> usize {
    triangle.iter().filter(|&&x| coloring[x] > 0).count()
}



/// Return a coloration: V -> {1,...,c}
fn clean(todo: &Vec<usize>, triangles: &Vec<Vec<usize>>, coloring: &mut Vec<usize>, color_max: usize) -> Vec<usize> {
    let mut new_todo = Vec::new();
    
    for &i in todo.iter() {
        if is_conflict(&triangles[i], coloring) {
            return Vec::new(); // Return empty vector if conflict
        } else if is_satisfied(&triangles[i], coloring) {
            continue;
        }
        new_todo.push(i);
    }

    // Sort new_todo by increasing number of colored vertices
    search_proper_coloring(&new_todo, triangles, coloring, color_max)
}





fn search_proper_coloring(
    todo: &Vec<usize>,
    triangles: &Vec<Vec<usize>>,
    coloring: &mut Vec<usize>,
    color_max: usize,
) -> Vec<usize> {
    if todo.is_empty() {
        return coloring.to_vec();
    }

    let x = search_optimal_vertex(todo, triangles, coloring);

    for c in 1..=color_max {
        coloring[x] = c;
        let r = clean(todo, triangles, coloring, color_max);
        if !r.is_empty() {
            return r;
        }
        coloring[x] = 0;
    }

    Vec::new()
}



pub fn acyclic_coloring(
    g: &BitwiseAdjacencyMatrix,
    color_max: usize,
) -> Vec<usize> {
    let n = g.n;
    let mut triangles = vec![];
    let mut todo: Vec<usize> = Vec::new();
    let mut coloring = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if (i < j && j < k) || (i > j && j > k) {
                    if g.has_arc(i, j) && g.has_arc(j, k) && g.has_arc(k, i) {
                        todo.push(triangles.len());
                        triangles.push( vec![i,j,k]);
                    }
                }
            }
        }
    }

    search_proper_coloring(&todo, &triangles, &mut coloring, color_max)
}



pub fn compute_dichromatric_number(g: &BitwiseAdjacencyMatrix) -> usize {
    for i in 0..g.n {
        if acyclic_coloring(&g, i).len() > 0{
            return i
        }
    }
    return g.n
}

