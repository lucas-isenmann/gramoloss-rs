use rand::{random_bool, random_range};

use crate::BitwiseAdjacencyMatrix;

fn lol(g: &mut BitwiseAdjacencyMatrix, v: &mut Vec<Vec<Vec<usize>>>, i: usize, j: usize, i2: usize, j2: usize){
    for k in 0..3{
        for k2 in 0..3 {
            g.add_arc(v[i][j][k], v[i2][j2][k2]);
        }
    }
}


fn make_incomparable(g: &mut BitwiseAdjacencyMatrix, v: &mut Vec<Vec<Vec<usize>>>, i: usize, j: usize, i2: usize, j2: usize){
    let a = random_range(0..3);
    let b = (a+1)%3;
    let c = (a+2)%3;

    let a2 = v[i2][j2][a];
    let b2 = v[i2][j2][b];
    let c2 = v[i2][j2][c];

    let a1 = v[i][j][0];
    let b1 = v[i][j][1];
    let c1 = v[i][j][2];

    
    if random_bool(0.5){
        lol2(g, a1, b1, c1, a2, b2, c2);
    } else {
        lol2(g, a2, b2, c2, a1, b1, c1);
    }
    
}


pub fn lol2(g: &mut BitwiseAdjacencyMatrix, a1: usize, b1: usize, c1: usize,
a2: usize, b2: usize, c2: usize){
    g.add_arc(a1, a2);
    g.add_arc(a1, b2);
    g.add_arc(c2, a1);

    g.add_arc(b1, a2);
    g.add_arc(b2, b1);
    g.add_arc(b1, c2);

    g.add_arc( a2, c1);
    g.add_arc(c1, b2);
    g.add_arc(c1, c2);
}


fn make_incomparable2(g: &mut BitwiseAdjacencyMatrix, v: &mut Vec<Vec<Vec<usize>>>, matrix: &Vec<Vec<usize>>,  i: usize, j: usize, i2: usize, j2: usize){
    
    let a2 = v[i2][j2][0];
    let b2 = v[i2][j2][1];
    let c2 = v[i2][j2][2];

    let a1 = v[i][j][0];
    let b1 = v[i][j][1];
    let c1 = v[i][j][2];
    let vtriangles = vec![a1, b1, c1, a2, b2, c2];

    
    let r = random_range(0..matrix.len());


    for i in 0..9 {
        g.add_arc(vtriangles[matrix[r][2*i]], vtriangles[matrix[r][2*i+1]]);
    }
    
}



pub fn gen_grid_poset(g: &mut BitwiseAdjacencyMatrix, n: usize, m : usize, matrix: &Vec<Vec<usize>>){
    let s = 3*n*m;
    // let mut g = BitwiseAdjacencyMatrix::new(s);
    g.reset();

    let mut v: Vec<Vec<Vec<usize>>> = vec![vec![vec![0;3];m];n];

    

    for i in 0..n {
        for j in 0..m {
            for k in 0..3 {
                v[i][j][k] = k+3*(m*i+j);
                // println!("{i} {j} {k} {}", v[i][j][k]);
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            g.add_arc(v[i][j][0], v[i][j][1]);
            g.add_arc(v[i][j][1], v[i][j][2]);
            g.add_arc(v[i][j][2], v[i][j][0]);

            for i2 in i..n {
                if i2 == i {
                    for j2 in (j+1)..m {
                        // make_incomparable(&mut g, &mut v, i, j, i2, j2);
                        make_incomparable2(g, &mut v, &matrix, i, j, i2, j2);
                    }
                } else {

                    // (i,j) => (i+1,j) et c'est tout
                    lol( g, &mut v, i,j, i2, j );
                    for j2 in 1..m {
                        make_incomparable2( g, &mut v, &matrix, i,j, i2, (j+j2)%m );
                    }

                    // (i,j) => (i+1,j) et (i+1,j+1)
                    // for j2 in 0..(i2-i) {
                    //     lol( g, &mut v, i,j, i2, (j+j2)%m );
                    // }
                    // for j2 in (i2-i)..m{
                    //     // make_incomparable(&mut g, &mut v, i, j, i2, (j+j2)%m);
                    //     make_incomparable2( g, &mut v, &matrix, i, j, i2, (j+j2)%m);
                    // }
                }
                
            }

            
        }
    }
    





    for id in 0..s{
        for id2 in 0..s{
            if id < id2 {
                if g.has_arc(id, id2) == false && g.has_arc(id2, id) == false {
                    println!("{id} {id2}");
                    if random_bool(0.5) {
                        g.add_arc(id, id2);
                    } else {
                        g.add_arc(id2, id);
                    }
                }
            }
        }
    }


    



    // g
}