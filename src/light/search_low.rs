use crate::{compute_dichromatric_number, is_light, is_local_light, BitwiseAdjacencyMatrix};



pub fn light_extend(g: &mut BitwiseAdjacencyMatrix){

    let n = g.nb_vertices();

    let mut t = vec![vec![vec![];n];n];
    for i in 0..n{
        for j in 0..n {
            if g.has_arc(i, j) {
                // i -> j -> u -> i
                for u in 0..n {
                    if g.has_arc(j,u) && g.has_arc(u, i){

                        let mut insertion_index = None;
                        let mut is_ok = true;
                        for (index, &x) in t[i][j].iter().enumerate() {
                            if insertion_index == None {
                                if g.has_arc(u, x) {
                                    insertion_index = Some(index);
                                }
                            } else if g.has_arc(x, u) {
                                println!("not light {i} {j} with {u}");
                                is_ok = false;
                                break;
                            }
                        }
                        if is_ok {
                            if let Some(index) = insertion_index {
                                t[i][j].insert(index, u);
                            } else {
                                t[i][j].push(u);
                            }
                        }
                    }
                }
            }
        }
    }



    for i in 0..n {
        // Compute N+(i) and N-(i) in [0,i[


        // Iterate over all possibilities for R
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


    g.print_in_degrees();


    println!("m={}", g.nb_arcs());

    let n = 3+5*n;

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
                    return;
                }
            }
        } 
        else {
            // if true || is_light(&g)  {
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
                return;
            }
        }
    }
    

}
