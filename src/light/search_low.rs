use crate::BitwiseAdjacencyMatrix;




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
pub fn search5(n: usize){
    println!("search 5");
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

    g.add_arcs(&left, &right);

    g.add_arcs( &t01, &vec![0]);
    g.add_arcs(&vec![1], &t01);

    g.add_arcs( &t12, &vec![1]);
    g.add_arcs(&vec![2], &t12);

    g.add_arcs( &t20, &vec![2]);
    g.add_arcs(&vec![0], &t20);


    g.print_in_degrees();



    

}
