use crate::bin_adj_matrices::BitwiseAdjacencyMatrix;

pub fn is_dominating(g: &BitwiseAdjacencyMatrix, aset: &Vec<usize>, bset: &Vec<usize>) -> bool {
    for &a in aset {
        for &b in bset {
            if g.has_arc(a, b) == false {
                return false;
            }
        }
    }
    return true;
}

pub fn compute_triangles_poset(g: &BitwiseAdjacencyMatrix){
    let n = g.size();
    let mut triangles = Vec::new();
    let mut t = 0;
    let mut dominate: Vec<Vec<usize>> = Vec::new();

    for i in 0..n {
        for j in i+1..n {
            if g.has_arc(i, j){
                for k in j+1..n {
                    if g.has_arc(j, k) && g.has_arc(k, i){
                        dominate.push( vec![] );
                        for x in 0..t {
                            let (a,b,c) = triangles[x];
                            if is_dominating(g, &vec![a,b,c], &vec![i,j,k]){
                                dominate[x].push(t);
                            }
                            else if is_dominating(g, &vec![i,j,k] , &vec![a,b,c] ) {
                                dominate[t].push(x);
                            }
                        }
                        triangles.push((i,j,k));
                        t += 1;
                    }
                }
            }
        }
    }
    println!("{triangles:?}");
    println!("{dominate:?}");
    for i in 0..n {
        for j in 0..i {
            if g.has_arc(i, j){
                for k in 0..j {
                    if g.has_arc(j, k) && g.has_arc(k, i){
                        dominate.push( vec![] );
                        for x in 0..t {
                            let (a,b,c) = triangles[x];
                            if is_dominating(g, &vec![a,b,c], &vec![i,j,k]){
                                dominate[x].push(t);
                            }
                            else if is_dominating(g, &vec![i,j,k] , &vec![a,b,c] ) {
                                dominate[t].push(x);
                            }
                        }
                        triangles.push((i,j,k));
                        t += 1;
                    }
                }
            }
        }
    }

    println!("{triangles:?}");
    println!("{dominate:?}");
    for i in 0..t {
        println!("{i}: {:?}: {:?} ", triangles[i], dominate[i] );
    }

}