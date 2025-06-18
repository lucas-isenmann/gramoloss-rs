use crate::BitwiseAdjacencyMatrix;



fn search_optimal_vertex(todo: &Vec<usize>, triangles: &Vec<(usize, usize, usize)>, coloring: &Vec<usize>) -> usize {
    let mut count = vec![0; coloring.len()];
    let mut record = 0;
    let mut vertex = 0;

    for x in todo.iter() {
        let &(u,v,w) = &triangles[*x];
        if coloring[u] == 0 {
            count[u] += 1;
            if count[u] > record {
                record = count[u];
                vertex = u;
            }
        }
        if coloring[v] == 0 {
            count[v] += 1;
            if count[v] > record {
                record = count[v];
                vertex = v;
            }
        }
        if coloring[w] == 0 {
            count[w] += 1;
            if count[w] > record {
                record = count[w];
                vertex = w;
            }
        }
    }

    vertex
}



fn is_conflict(triangle: &(usize, usize, usize), coloring: &Vec<usize>) -> bool {
    coloring[triangle.0] == coloring[triangle.1] &&
    coloring[triangle.0] == coloring[triangle.2] &&
    coloring[triangle.0] > 0
}

fn is_satisfied(triangle: &(usize, usize, usize), coloring: &Vec<usize>) -> bool {
    let mut color1 = 0;

    let &(x,y,z) = triangle;
    if coloring[x] > 0 {
        if color1 > 0 && coloring[x] != color1 {
            return true;
        } else if color1 == 0 {
            color1 = coloring[x];
        }
    }
    if coloring[y] > 0 {
        if color1 > 0 && coloring[y] != color1 {
            return true;
        } else if color1 == 0 {
            color1 = coloring[y];
        }
    }
    if coloring[z] > 0 {
        if color1 > 0 && coloring[z] != color1 {
            return true;
        } else if color1 == 0 {
            color1 = coloring[z];
        }
    }
    false
}



/// Return a coloration: V -> {1,...,c} (or 0 if any choice is ok)
fn clean(todo: &Vec<usize>, triangles: &Vec<(usize, usize, usize)>, coloring: &mut Vec<usize>, color_max: usize, colorings: &mut Vec<Vec<usize>>){
    let mut new_todo = Vec::new();
    
    for &i in todo.iter() {
        if is_conflict(&triangles[i], coloring) {
            return (); // Return empty vector if conflict
        } else if is_satisfied(&triangles[i], coloring) {
            continue;
        }
        new_todo.push(i);
    }

    // Sort new_todo by increasing number of colored vertices
    search_proper_colorings(&new_todo, triangles, coloring, color_max, colorings)
}





fn search_proper_colorings(
    todo: &Vec<usize>,
    triangles: &Vec<(usize, usize, usize)>,
    coloring: &mut Vec<usize>,
    color_max: usize,
    colorings: &mut Vec<Vec<usize>>
)  {
    if todo.is_empty() {
        colorings.push(coloring.clone());
    }

    let x = search_optimal_vertex(todo, triangles, coloring);

    for c in 1..=color_max {
        coloring[x] = c;
        clean(todo, triangles, coloring, color_max, colorings);
        coloring[x] = 0;
    }
}



pub fn list_all_acyclic_colorings(
    g: &BitwiseAdjacencyMatrix,
    color_max: usize,
) -> Vec<Vec<usize>> {
    let n = g.nb_vertices();
    let mut triangles = vec![];
    let mut todo: Vec<usize> = Vec::new();
    let mut coloring = vec![0; n];
    let mut colorings = vec![];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if (i < j && j < k) || (i > j && j > k) {
                    if g.has_arc(i, j) && g.has_arc(j, k) && g.has_arc(k, i) {
                        todo.push(triangles.len());
                        triangles.push( (i,j,k) );
                    }
                }
            }
        }
    }

    search_proper_colorings(&todo, &triangles, &mut coloring, color_max, &mut colorings);
    colorings
}