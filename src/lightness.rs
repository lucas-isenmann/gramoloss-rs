use crate::matrix_graph::print_adj;


pub fn is_light(adj: &Vec<Vec<bool>>) -> bool {
    let n = adj.len();
    for u in 0..n {
        for v in 0..n {
            if adj[u][v] {
                for a in 0..n {
                    if adj[a][u] && adj[v][a] {
                        for b in 0.. n {
                            if adj[b][u] && adj[v][b] && adj[a][b] {
                                for c in 0..n{
                                    if adj[c][u] && adj[v][c] && adj[b][c] && adj[c][a] {
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




pub fn is_local_light(matrix: &Vec<Vec<bool>>, u: usize, v: usize) -> bool {
    let n = matrix.len();

    // Type 1 conflict with uv (u->v is an heavy arc)
    let mut vertices = vec![];
    for x in 0..n {
        if matrix[x][u] && matrix[v][x] {
            vertices.push(x);
        }
    }

    for &a in &vertices {
        for &b in &vertices {
            if matrix[a][b] {
                for &c in &vertices {
                    if matrix[b][c] && matrix[c][a] {
                        // console.log("type 1")
                        return false;
                    }
                }
            }
        }
    }

    // Type 2: uv is in the triangle (search for w the third vertex of the triangle)
    for w in 0..n {
        if matrix[v][w] && matrix[w][u] {
            let mut dominated = vec![];
            for a in 0..n {
                if matrix[v][a] && matrix[u][a] && matrix[w][a] {
                    dominated.push(a);
                }
            }
            for b in 0..n {
                if matrix[b][v] && matrix[b][u] && matrix[b][w] {
                    for &a in &dominated {
                        if matrix[a][b] {
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
        if matrix[v][w] && matrix[u][w] {
            for x in 0..n {
                if matrix[w][x] && matrix[u][x] && matrix[x][v] {
                    for y in 0..n {
                        if matrix[v][y] && matrix[x][y] && matrix[w][y] && matrix[y][u] {
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
        if matrix[u][w] && matrix[w][v] {
            for x in 0..n {
                if matrix[w][x] && matrix[x][v] && matrix[x][u] {
                    for y in 0..n {
                        if matrix[v][y] && matrix[y][u] && matrix[y][w] && matrix[y][x] {
                            return false
                        }
                    }
                }
            }
        }
    }

    true
}






pub fn is_light_critic(m: &Vec<Vec<bool>>) -> bool {
    let n = m.len();

    let mut adj = vec![vec![false; n+1]; n+1];
    for i in 0..n {
        for j in 0..n {
            adj[i][j] = m[i][j];
        }
    }

    let u: usize = n;


    let mut todo = vec![];
    let mut done = vec![];

    for v in 0..n {
        todo.push((v, u));
    }

    loop {
        while let Some((i,j)) = todo.pop(){

            println!("impl {i} {j}");
            adj[i][j] = true; // Add arc ij

            if is_light(&adj){
                done.push((i,j));
                continue;
            } else {
                println!("del {i} {j}");
                adj[i][j] = false; // Cancel arc ij
                if i < j {
                    println!("impl {j} {i}");
                    adj[j][i] = true; // Add arc ji if i < j

                    if is_light(&adj) {
                        done.push((j,i));
                    } else {
                        adj[j][i] = false;
                        // Backtrack
                        let mut finito = true;
                        todo.push((i,j));
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
                        if finito {
                            return true;
                        }
                    }

                    
                } else {
                    // Backtrack
                    let mut finito = true;
                    todo.push((i,j));
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
                    if finito {
                        return true;
                    }
                }
            }
        }
        // End of branch
        // Tournament is light
        let mut has_twin = false;
        for x in 0..n {
            let mut is_twin = true;
            for y in 0..n {
                if y != x && y != u && adj[x][y] != adj[u][y] {
                    is_twin = false;
                    break;
                }
            }
            if is_twin == true {
                has_twin = true;
                break;
            }
        }

        let mut in_degree = 0;
        for x in 0..n{
            if adj[x][u]{
                in_degree += 1;
            }
        }

        if has_twin == false && in_degree != 0 && in_degree != n {
            print_adj(&adj);
            return false;
        }

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
            return true;
        }

    }

//  let finito = false;
//  while (finito == false){
//      const r = todo.pop();
//      if (typeof r != "undefined"){
//          const [x,y] = r;
//          const addedArc = h.addLink(x, y, ORIENTATION.DIRECTED);
//          if (typeof addedArc != "undefined"){
//              if (h.isTournamentLight() == false){
//                  h.deleteLink(addedArc.index);
//                  if ( y > x){
//                      // Rembobiner
//                      while (true){
//                          const r2 = done.pop();
//                          if (typeof r2 == "undefined"){
//                              // Finito
//                              finito = true;
//                              break;
//                          } else {
//                              const [a,b, arcId] = r2;
//                              h.deleteLink(arcId);
                             
//                              if (a < b ){
//                                  todo.push([b,a]);
//                                  break;
//                              } else {
//                                  todo.push([a,b]);
//                              }
//                          }
//                      }
//                  }
//              } else {
//                  done.push([x,y, addedArc.index]);
//              }
//          }
//      } else {
//          // On a tout mis et c'est light
//          // Check si c'est un twin
//          let is_twin = true;
//          for (const [id, v] of g.vertices){
             
//          }

//          // Sinon return true

//          // Rembobiner
//          while (true){
//              const r2 = done.pop();
//              if (typeof r2 == "undefined"){
//                  // Finito
//                  finito = true;
//                  break;
//              } else {
//                  const [a,b, arcId] = r2;
//                  h.deleteLink(arcId);
                 
//                  if (a < b ){
//                      todo.push([b,a]);
//                      break;
//                  } else {
//                      todo.push([a,b]);
//                  }
//              }
//          }

//      }
//  }
}

       

