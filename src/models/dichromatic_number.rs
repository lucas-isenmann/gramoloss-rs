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



pub fn acyclic_coloring_matrix(
    adj: &Vec<Vec<bool>>,
    color_max: usize,
) -> Vec<usize> {
    let n = adj.len();
    let mut triangles = vec![];
    let mut todo: Vec<usize> = Vec::new();
    let mut coloring = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if (i < j && j < k) || (i > j && j > k) {
                    if adj[i][j] && adj[j][k] && adj[k][i] {
                        todo.push(triangles.len());
                        triangles.push( vec![i,j,k]);
                    }
                }
            }
        }
    }

    search_proper_coloring(&todo, &triangles, &mut coloring, color_max)
}



pub fn compute_dichromatric_number_matrix(adj: &Vec<Vec<bool>>) -> usize {
    let n = adj.len();
    for i in 0..n {
        if acyclic_coloring_matrix(&adj, i).len() > 0{
            return i
        }
    }
    return n
}



pub fn to_dot(adj: &Vec<Vec<bool>>){
    let n = adj.len();
    println!("digraph G {{");
    for i in 0..n {
        for j in 0..n {
            if adj[i][j] {
                println!("{i} -> {j};")
            }
        }
    }
    println!("}}");

}