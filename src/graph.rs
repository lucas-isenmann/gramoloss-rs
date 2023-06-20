use std::{collections::{HashMap}, fs::{File, self}, io::Write, fmt, cmp::min};

use rand::{Rng, random};

use crate::{vertex::Vertex, link::Link, math_utils::{det, is_point_in_triangle, is_in_circle}, coord::Coord};



#[derive(Clone,Debug)]
pub struct Graph{
    vertices: HashMap<usize, Vertex>,
    links: HashMap<usize, Link>
}

impl Graph {

    pub fn new()-> Graph{
        return Graph {
            vertices: HashMap::new(),
            links: HashMap::new()
        }
    }

    pub fn to_svg(&self, path: &str){
        let mut file = File::create(path).expect("Unable to create file");

        let mut minx = 0.;
        let mut miny = 0.;
        let mut maxx = 0.;
        let mut maxy = 0.;
        for vertex in self.vertices.values(){
            if vertex.pos.x < minx {
                minx = vertex.pos.x;
            }
            if vertex.pos.y < miny {
                miny = vertex.pos.y;
            }
            if vertex.pos.x > maxx {
                maxx = vertex.pos.x;
            }
            if vertex.pos.y > maxy {
                maxy = vertex.pos.y;
            }
        }
        minx -= 20.;
        miny -= 20.;
        maxx += 20.;
        maxy += 20.;

        let header = format!(r#"<?xml version="1.0" standalone="yes"?>
        <svg
            width="100%"
            height="100%"
            viewBox="{minx} {miny} {maxx} {maxy}"
            preserveAspectRatio="xMidYMid meet"
            xmlns="http://www.w3.org/2000/svg"
            >"#);
        file.write_all(header.as_bytes()).expect("Unable to write to file");

        let stats = format!(r#"<text x="-20" y="5" font-size="5px" fill="white">
        <tspan x="-20" dy="0">vertices={}</tspan>
        <tspan x="-20" dy="1.2em">links={}</tspan>
        </text>"#, self.vertices.len(), self.links.len());
        file.write_all(stats.as_bytes()).expect("Unable to write to file");

        for link in self.links.values(){
            let v1 = self.vertices.get(&link.start_index).unwrap();
            let v2 = self.vertices.get(&link.end_index).unwrap();
            let text = format!(r#"<line
            x1="{}"
            y1="{}"
            x2="{}"
            y2="{}"
            stroke="{}"
            stroke-width="{}"
            />"#,v1.pos.x, v1.pos.y, v2.pos.x, v2.pos.y, "gray", 1 );
            file.write_all(text.as_bytes()).expect("Unable to write to file");
        }
        

        for (index, vertex) in self.vertices.iter(){
            let text = format!( r#"<circle
                cx="{}"
                cy="{}"
                r="{}"
                fill="{}"
              />"#, vertex.pos.x, vertex.pos.y, 3,"white");
              file.write_all(text.as_bytes()).expect("Unable to write to file");
        }

        let footer = "</svg>";
        file.write_all(footer.as_bytes()).expect("Unable to write to file");
    }

    /// Generate a graph with n random vertices in the [0,w]*[0,w] box.
    /// There are no links generated.
    /// w is meant to be positive but I have not tested with negative value.
    pub fn rand_points(n: usize, w: f64) -> Graph {
        let mut g = Graph::new();
        for i in 0..n {
            let v = Vertex::rand_range(w);
            g.vertices.insert(i,v);
        }
        g
    }

    pub fn print(&self){
        println!("#vertices = {}", self.vertices.len());
        println!("#links = {}", self.links.len());
        for link in self.links.values() {
            println!("{} {}", link.start_index, link.end_index);
        }
    }

    pub fn to_graphviz(&self) -> String{
        let mut result = "".to_string();
        result += "graph {\noverlap=true\n";
        for (index,vertex) in self.vertices.iter() {
            result += &format!("{} [shape=circle, pos=\"{},{}!\"]\n", index, vertex.pos.x/7., vertex.pos.y/7.);
        }
        result += "\n";
        result += "}\n";
        result
    }

    /// Adds a link to the Graph. Oriented links are not considererd for the moment.
    /// If the vertices do not exist, the vertices are created. 
    pub fn add_link(&mut self, start_index: usize, end_index: usize){
        for link in self.links.values() {
            if link.start_index == start_index && link.end_index == end_index {
                return
            }
            if link.start_index == end_index && link.end_index == start_index {
                return
            }
        }

        let mut k :usize = 0;
        while self.links.contains_key(&k){
            k += 1;
        }
        let new_link = Link::new(start_index, end_index, None);
        if !self.vertices.contains_key(&start_index) {
            let new_start_vertex = Vertex::new();
            self.vertices.insert(start_index, new_start_vertex);
        }
        if !self.vertices.contains_key(&end_index) {
            let new_end_vertex = Vertex::new();
            self.vertices.insert(end_index, new_end_vertex);
        }
        self.links.insert(k, new_link);
    }

    pub fn degree(&self, vertex_index: usize) -> usize{
        let mut degree = 0;
        for link in self.links.values() {
            if link.start_index == vertex_index || link.end_index == vertex_index {
                degree += 1;
            } 
        }
        degree
    }



    /// Returns an abstract path graph.
    /// m is the number of edges.
    /// So there are m+1 vertices.
    pub fn gen_path(m: usize) -> Graph{
        let mut g = Graph::new();
        for i in 0..m {
            g.add_link(i, i+1)
        }
        g
    }


    pub fn chromatic_number(&self) -> usize {
        let mut k = 1;
        let n = self.vertices.len();

        loop {
            if k >= 5{
                return 0;
            }
            let mut color = Vec::new();
            let mut indices = HashMap::new();
            let mut j:usize = 0;
            for &index in self.vertices.keys(){
                color.push(0);
                indices.insert(index,j);
                j +=1 ;
            }
            loop {
                let mut i = n-1;
                let mut break_loop = false;
                while color[i] == k-1{
                    color[i] = 0;
                    if i == 0 {
                        break_loop = true;
                        break;
                    } else {
                        i -=1 ;
                    }
                    
                }
                if break_loop { // every color was set to k-1
                    break;      // all assignements have been tried
                }
                color[i] +=1;
                // else next color assignement
                // check it
                let mut is_proper_coloring = true;
                for link in self.links.values(){
                    if let Some(&vec_start_index) = indices.get(&link.start_index){
                        if let Some(&vect_end_index) = indices.get(&link.end_index){
                            if color[vec_start_index] == color[vect_end_index]{
                                is_proper_coloring = false;
                                break;
                            }
                        }
                    } 
                }
                if is_proper_coloring{
                    return k;
                }
            }
            k += 1;
        }
        
    }

    pub fn set_euclidian_link_weights(&mut self){
        for (_, link) in self.links.iter_mut(){
            let v1 = self.vertices.get(&link.start_index).unwrap();
            let v2 = self.vertices.get(&link.end_index).unwrap();
            let d = v1.distance_to(&v2);
            link.weight = Some(d);
        }
    }

    /**
     * 
     */
    pub fn floyd_warhall(&self, weighted: bool) -> (HashMap<usize, HashMap<usize, Option<f64>>>,HashMap<usize, HashMap<usize, Option<usize>>>) {
        let mut dist: HashMap<usize, HashMap<usize, Option<f64>>> = HashMap::new();
        let mut next: HashMap<usize, HashMap<usize, Option<usize>>> = HashMap::new();

        for &v_index in self.vertices.keys() {
            let mut v_dist: HashMap<_, _> = HashMap::new();
            v_dist.insert(v_index, Some(0.));

            let mut v_next = HashMap::new();
            v_next.insert(v_index, Some(v_index));

            for &u_index in self.vertices.keys() {
                if v_index != u_index {
                    v_dist.insert(u_index, None);
                    v_next.insert(u_index, None);
                }
            }
            dist.insert(v_index, v_dist);
            next.insert(v_index, v_next);
        }

        for (_,link) in self.links.iter() {
            // TODO: Oriented Case
            let weight = if weighted {
                link.weight
            } else {
                Some(1.)
            };

            dist.get_mut(&link.start_index).unwrap().insert(link.end_index, weight);
            dist.get_mut(&link.end_index).unwrap().insert(link.start_index, weight);

            next.get_mut(&link.start_index).unwrap().insert(link.end_index, Some(link.end_index));
            next.get_mut(&link.end_index).unwrap().insert(link.start_index, Some(link.start_index));
        }

        for k_index in self.vertices.keys() {
            for  i_index in self.vertices.keys() {
                for  j_index in self.vertices.keys() {
                    let direct = dist.get(i_index).unwrap().get(j_index).unwrap();
                    let shortcut_part_1 = dist.get(i_index).unwrap().get(k_index).unwrap();
                    let shortcut_part_2 = dist.get(k_index).unwrap().get(j_index).unwrap();

                    if let Some(d1) = shortcut_part_1 {
                        if let Some(d2) = shortcut_part_2 {
                            let dshort = *d1 + *d2;
                            if let Some(d) = direct{
                                if *d > dshort {
                                    let n = next.get(i_index).unwrap().get(k_index).unwrap().clone();
                                    dist.get_mut(i_index).unwrap().insert(*j_index, Some(dshort));
                                    next.get_mut(i_index).unwrap().insert(*j_index, n );
                                }
                            } else {
                                let n = next.get(i_index).unwrap().get(k_index).unwrap().clone();
                                dist.get_mut(i_index).unwrap().insert(*j_index, Some(dshort));
                                next.get_mut(i_index).unwrap().insert(*j_index, n );
                            }
                        }
                    }
                    
                }
            }
        }

        (dist,next)
    }


    /// Returns the stretch of the graph.
    /// The stretch is defined as the maximal stretch between pairs of vertices.
    /// The stretch of a pair of vertices is defined as the ratio between the euclidian distance in the graph between them and the euclidian distance between them.
    /// 
    /// # Returns
    /// 
    /// Returns None if there is 1 vertex or less.
    /// 
    /// # Examples
    /// 
    ///     let g2 = Graph.fromList([(0,0), (0,10), (10,10)], [(0,1),(1,2)]);
    ///     println!("{}", g2.stretch()); // should be sqrt(2)
    ///
    pub fn stretch(&self) -> (f64,Option<usize>) {
        let mut rng = rand::thread_rng();
        let (distances,next) = self.floyd_warhall(true);
        let mut max_stretch = 0.;
        let mut random_inner_geodetic_vertex: Option<usize> = None;
        for  (index_v1, v1) in self.vertices.iter(){
            for (index_v2, v2) in self.vertices.iter(){
                if index_v1 != index_v2{
                    let v1distances = distances.get(index_v1).unwrap();

                    if let Some(graph_dist) = v1distances.get(index_v2).unwrap(){
                        let stretch = graph_dist / v1.distance_to(v2);
                        if max_stretch < stretch {
                            max_stretch = stretch;

                            // getting a random vertex on the geodetic between v1 and v2
                            let mut inner_vertices = Vec::new();
                            let mut i = index_v1;
                            let mut i_next = next.get(i).unwrap();
                            while let Some(ni) = i_next.get(index_v2).unwrap() {
                                if ni == index_v2 {
                                    break;
                                } else {
                                    inner_vertices.push(*ni);
                                    i = ni;
                                    i_next = next.get(i).unwrap();
                                }
                            }
                            if inner_vertices.len() > 0 {
                                random_inner_geodetic_vertex = Some(inner_vertices[rng.gen_range(0..inner_vertices.len())]);
                            }
                        }
                    }
                }
            }
        }
        (max_stretch, random_inner_geodetic_vertex)
    }


    /// Clears all edges and put them back according to the Delaunay rule.
    /// (v,w) is an edge iff there is no vertex inside any circle going through v and w.
    /// 
    pub fn reset_edges_delaunay(&mut self){
        println!("new");
        self.links.clear();
        let mut mat = 
        vec![vec![1.;4],
        vec![1.;4],
        vec![1.;4],
        vec![1.;4]];
        // let mut triangles_to_add = Vec::new();

        for i1 in 0..self.vertices.len(){
            for i2 in i1+1..self.vertices.len(){
                for i3 in i2+1..self.vertices.len(){
                    let v1: &Vertex = self.vertices.get(&i1).unwrap();
                    let v2: &Vertex = self.vertices.get(&i2).unwrap();
                    let v3: &Vertex = self.vertices.get(&i3).unwrap();
                    
                    let mut is_point_inside = false;
                    mat[0][0] = v1.pos.x;
                    mat[0][1] = v1.pos.y;
                    mat[0][2] = v1.pos.x*v1.pos.x +v1.pos.y*v1.pos.y;

                    mat[1][0] = v2.pos.x;
                    mat[1][1] = v2.pos.y;
                    mat[1][2] = v2.pos.x*v1.pos.x +v2.pos.y*v2.pos.y;

                    mat[2][0] = v3.pos.x;
                    mat[2][1] = v3.pos.y;
                    mat[2][2] = v3.pos.x*v1.pos.x +v3.pos.y*v3.pos.y;

                    // Check if the points are in counterclowise order.
                    if (v2.pos.x - v1.pos.x)*(v3.pos.y-v1.pos.y)-(v3.pos.x -v1.pos.x)*(v2.pos.y-v1.pos.y) <= 0. {
                        // clockwise order
                        for (&i4, v4) in self.vertices.iter(){
                            if i1 != i4 && i2 != i4 && i3 != i4{
                                 mat[3][0] = v4.pos.x;
                                 mat[3][1] = v4.pos.y;
                                 mat[3][2] = v4.pos.x*v1.pos.x +v4.pos.y*v4.pos.y;
                                 
                                 if det(&mat) < 0. {
                                     is_point_inside = true;
                                     break;
                                 }
                             }
                         }
                    } else {
                        // ccw
                        for (&i4, v4) in self.vertices.iter(){
                            if i1 != i4 && i2 != i4 && i3 != i4{
                                mat[3][0] = v4.pos.x;
                                mat[3][1] = v4.pos.y;
                                mat[3][2] = v4.pos.x*v1.pos.x +v4.pos.y*v4.pos.y;
                                 if det(&mat) > 0. {
                                     is_point_inside = true;
                                     break;
                                 }
                             }
                         }
                    }
                    
                    if is_point_inside == false{
                        // triangles_to_add.push( (i1, i2, i3));
                        self.add_link(i1,i2);
                        self.add_link(i1,i3);
                        self.add_link(i2,i3);
                    }
                }
            }
        }

        // for (i,j,k) in triangles_to_add{
        //     self.add_link(i,j);
        //     self.add_link(i,k);
        //     self.add_link(j,k);
        // }

    }


    // Insert(p) {
    //     Find the triangle 4abc containing p
    //     Insert edges pa, pb, and pc into triangulation
    //     SwapTest(ab) // check/fix the surrounding edges
    //     SwapTest(bc)
    //     SwapTest(ca)
    //     }

    pub fn incremental_delaunay_insert(&mut self, iext: usize, triangles: &mut Vec<(usize,usize,usize)>, v_index: usize ){
        let v = self.vertices.get(&v_index).unwrap();
        for i in 0..triangles.len() {
            let triangle = triangles[i];
            let v0 = self.vertices.get(&triangle.0).unwrap();
            let v1 = self.vertices.get(&triangle.1).unwrap();
            let v2 = self.vertices.get(&triangle.2).unwrap();

            if is_point_in_triangle(&v.pos, &v0.pos, &v1.pos, &v2.pos){
                triangles.remove(i);
                self.add_link(v_index, triangle.0 );
                self.add_link(v_index, triangle.1 );
                self.add_link(v_index, triangle.2 );
                triangles.push((v_index, triangle.0, triangle.1));
                triangles.push((v_index, triangle.1, triangle.2));
                triangles.push((v_index, triangle.2, triangle.0));
                self.incremental_delaunay_swap_test(iext,triangles, triangle.0, triangle.1, v_index);
                self.incremental_delaunay_swap_test(iext,triangles, triangle.1, triangle.2, v_index);
                self.incremental_delaunay_swap_test(iext,triangles, triangle.2, triangle.0, v_index);
                break;
            }
        }
        
    }


    //     SwapTest(ab) {
    //     if (ab is an edge on the exterior face) return
    //     Let d be the vertex to the right of edge ab
    //     if (inCircle(b, p, a, d)) { // d violates the incircle test
    //     Flip edge ab // replace ab with pd
    //     SwaptTest(ad) // check/fix the new suspect edges
    //     SwaptTest(db)
    //     }
    //     }

    pub fn incremental_delaunay_swap_test(&mut self, iext: usize, triangles: &mut Vec<(usize,usize,usize)>, a: usize, b: usize, p: usize) {
        if a >= iext && b >= iext { // triangles[0] contains the first triangle inserted
            return
        }
        let posp = self.vertices.get(&p).unwrap().pos;
        let mut d = 0; // 0 is an error
        for triangle in triangles.iter() {
            if triangle.0 == b && triangle.1 == a {
                d = triangle.2;
            } else if triangle.1 == b && triangle.2 == a {
                d = triangle.0
            } else if triangle.2 == b && triangle.0 == a {
                d = triangle.1
            }
        }
        let posd = self.vertices.get(&d).unwrap().pos;
        let pa = self.vertices.get(&a).unwrap().pos;
        let pb = self.vertices.get(&b).unwrap().pos;

        if is_in_circle(&pb, &posp, &pa, &posd) {
            let mut index = 0;
            for (&i, link) in self.links.iter(){
                if (link.start_index == a && link.end_index == b) || (link.start_index == b && link.end_index == a){
                    index = i;
                    break;
                }
            }
            self.links.remove(&index);
            self.add_link(p,d);
            // virer triangles
            for i in 0..triangles.len() {
                let triangle = triangles[i];
                if (triangle.0 == p && triangle.1 == a && triangle.2 == b) || (triangle.0 == a && triangle.1 == b && triangle.2 == p) || (triangle.0 == b && triangle.1 == p && triangle.2 == a)  {
                    triangles.remove(i);
                    break;
                }
            }

            for i in 0..triangles.len() {
                let triangle = triangles[i];
                if (triangle.0 == a && triangle.1 == d && triangle.2 == b) || (triangle.0 == d && triangle.1 == b && triangle.2 == a) || (triangle.0 == b && triangle.1 == a && triangle.2 == d)  {
                    triangles.remove(i);
                    break;
                }
            }

            triangles.push((p,a,d));
            triangles.push((p,d,b));

            self.incremental_delaunay_swap_test(iext, triangles, a,d,p);
            self.incremental_delaunay_swap_test(iext, triangles, d,b,p);


        }


    }


 


    /// 
    /// 
    pub fn reset_edges_delaunay_v3(&mut self){
        self.links.clear();

        let mut i = 0;
        while self.vertices.contains_key(&i){
            i += 1;
        }

        let v0 = Vertex::from_coord(2000.,-500.);
        let v1 = Vertex::from_coord(-1000.,-500.);
        let v2 = Vertex::from_coord(500., 2000.);

        self.vertices.insert(i,v0);
        self.vertices.insert(i+1, v1);
        self.vertices.insert(i+2, v2);


        self.add_link(i,i+1);
        self.add_link(i,i+2);
        self.add_link(i+1,i+2);
        
        let mut triangles = Vec::new();
        // triangles are CLOCKWISE
        triangles.push((i,i+1,i+2));


        let indices: Vec<usize> = self.vertices.keys().cloned().collect();
        for index in indices{
            if index < i {
                self.incremental_delaunay_insert(i, &mut triangles, index);
            }
        }

        self.links.retain(|_,link| link.start_index < i && link.end_index < i );
        self.vertices.retain(|&index,v| index < i);

    }

    pub fn reset_edges_delaunay_old(&mut self){
        self.links.clear();
        let mut triangles_to_add = Vec::new();

        for  (i1, v1) in self.vertices.iter(){
            for (i2, v2) in self.vertices.iter(){
                for (i3, v3) in self.vertices.iter(){
                    if  !( (i1 < i2 && i2 < i3) || (i1 > i2 && i2 > i3) ){
                        continue;
                    }
                    // Check if the points are in counterclowise order.
                    if (v2.pos.x - v1.pos.x)*(v3.pos.y-v1.pos.y)-(v3.pos.x -v1.pos.x)*(v2.pos.y-v1.pos.y) <= 0. {
                        // console.log("not ccw", i1, i2, i3);
                        continue;
                    }
                    // console.log("ccw", i1, i2, i3);

                    let mut is_point_inside = false;
                    for (i4, v4) in self.vertices.iter(){
                       if i1 != i4 && i2 != i4 && i3 != i4{
                            let mat = 
                            vec![vec![v1.pos.x, v1.pos.y, f64::powf(v1.pos.x,2.) + f64::powf(v1.pos.y,2.), 1.],
                            vec![v2.pos.x, v2.pos.y, f64::powf(v2.pos.x,2.) + f64::powf(v2.pos.y,2.), 1.],
                            vec![v3.pos.x, v3.pos.y, f64::powf(v3.pos.x,2.) + f64::powf(v3.pos.y,2.), 1.],
                            vec![v4.pos.x, v4.pos.y, f64::powf(v4.pos.x,2.) + f64::powf(v4.pos.y,2.), 1.]];
                            if det(&mat) > 0. {
                                is_point_inside = true;
                                break;
                            }
                        }
                    }
                    if is_point_inside == false{
                        triangles_to_add.push( (*i1, *i2, *i3));
                        
                    }
                }
            }
        }

        for (i,j,k) in triangles_to_add{
            self.add_link(i,j);
            self.add_link(i,k);
            self.add_link(j,k);
        }

    }

}





pub fn stretch_genetic_maximizer(){

    // parameters
    let pop_size = 200;
    let bucket_size = 20;
    let w = 100.;
    let mut_range = 20.;
    let fitness_power = 10.;

    // lets go
    let mut rng = rand::thread_rng();

    let mut population = Vec::new();
    let mut fitness = vec![0.; pop_size];

    // initialize
    for i in 0..pop_size{
        let mut g = Graph::new();

        // for j in 0..bucket_size{
        //     let mut v = Vertex::new();
        //     let rand_angle = rng.gen_range(0.0..6.18);
        //     v.pos = Coord::new(w/2. + f64::cos(rand_angle)*w/4., w/2. + f64::sin(rand_angle)*w/4.);
        //     v.pos.x += rng.gen_range(-1.0..1.0);
        //     v.pos.y += rng.gen_range(-1.0..1.0);
        //     g.vertices.insert(j,v);
        // }


        for j in 0..bucket_size{
            let mut v = Vertex::new();
            v.pos = Coord::new(rng.gen_range(w*0.25..w*3./4.), rng.gen_range(w*0.25..w*3./4.));
            g.vertices.insert(j,v);
        }

        g.reset_edges_delaunay_v3();
        g.set_euclidian_link_weights();
        population.push(g);
    }

    let mut to_mutate = vec![0;pop_size];

    loop {
        // fitness
        let mut fitness_total = 0.;
        let mut stretch_total = 0.;
        let mut max_stretch = 1.;
        let mut best_i = 0;
        for i in 0..pop_size {
            let (stretch,inner_vertex) = population[i].stretch();
            if let Some(index) = inner_vertex {
                to_mutate[i] = index;
            }
            fitness[i] = f64::powf(stretch, fitness_power);
            fitness_total += fitness[i];

            stretch_total += stretch;
            if stretch > max_stretch{
                best_i = i;
            }
            max_stretch = f64::max(max_stretch, stretch);
        }

        // fs::write("bestgraph.dot", population[best_i].to_graphviz()).unwrap();
        population[best_i].to_svg("test.svg");

        for i in 0..pop_size{
            fitness[i] = fitness[i]/fitness_total;
        }

        println!("{:.3} {:.3}", stretch_total/(pop_size as f64), max_stretch);


        // selection
        let mut new_pop = Vec::new();
        for _ in 0..pop_size{
            let r = rng.gen_range(0.0..1.0);
            let mut s = 0.;
            for j in 0..pop_size {
                if r <= s + fitness[j] {
                    new_pop.push(population[j].clone());
                    break;
                } else {
                    s += fitness[j];
                }
            }
        }
        population.clear();
        population = new_pop;

        // mutation
        for i in 0..pop_size {

            // V1 : un sommet bouge
            // let r = rng.gen_range(0..bucket_size);
            // if let Some(v) = population[i].vertices.get_mut(&r){
            //     let nx = v.pos.x + rng.gen_range(0.0..mut_range) - mut_range/2.;
            //     let ny = v.pos.y + rng.gen_range(0.0..mut_range) - mut_range/2.;
            //     if 0. <= nx && nx <= w && 0. <= ny && ny <= w {
            //         v.pos.x = nx;
            //         v.pos.y = ny;
            //     }
            // }

            // V2 : tous les sommets bouge BOF
            // for (_,v) in population[i].vertices.iter_mut() {
            //     let nx = v.pos.x + rng.gen_range(0.0..mut_range) - mut_range/2.;
            //     let ny = v.pos.y + rng.gen_range(0.0..mut_range) - mut_range/2.;
            //     if 0. <= nx && nx <= w && 0. <= ny && ny <= w {
            //         v.pos.x = nx;
            //         v.pos.y = ny;
            //     }
            // }

            // V3
            let v = population[i].vertices.get_mut(&to_mutate[i]).unwrap();
            let nx = v.pos.x + rng.gen_range(0.0..mut_range) - mut_range/2.;
            let ny = v.pos.y + rng.gen_range(0.0..mut_range) - mut_range/2.;
            if 0. <= nx && nx <= w && 0. <= ny && ny <= w {
                v.pos.x = nx;
                v.pos.y = ny;
            }
            


            population[i].reset_edges_delaunay_v3();
            population[i].set_euclidian_link_weights();
        }
    }
    

}



#[cfg(test)]
mod tests {
    use crate::{graph::Graph, coord::Coord};

    #[test]
    fn chromatic_number() {
        let g = Graph::gen_path(2);
        assert_eq!(g.chromatic_number(), 2);
        let g = Graph::gen_path(3);
        assert_eq!(g.chromatic_number(), 2);
        let g = Graph::gen_path(4);
        assert_eq!(g.chromatic_number(), 2);
    }

    #[test]
    fn stretch(){
        let mut g = Graph::gen_path(2);
        g.vertices.get_mut(&0).unwrap().pos = Coord::new(0.,0.);
        g.vertices.get_mut(&1).unwrap().pos = Coord::new(0.,1.);
        g.vertices.get_mut(&2).unwrap().pos = Coord::new(1.,1.);
        g.set_euclidian_link_weights();
        // if let Some(stretch) = g.stretch(){
        //     assert_eq!(stretch >= 1.414, true);
        // }
         
    }

    #[test]
    fn reset_edges_delaunay(){
        let mut g = Graph::gen_path(2);
        g.vertices.get_mut(&0).unwrap().pos = Coord::new(0.,0.);
        g.vertices.get_mut(&1).unwrap().pos = Coord::new(0.,1.);
        g.vertices.get_mut(&2).unwrap().pos = Coord::new(1.,1.);
        g.reset_edges_delaunay();
        assert_eq!(g.links.len(), 3);

        // for _ in 0..10 {
        //     let mut g = Graph::rand_points(20,100.);
        //     let mut g2 = g.clone();
        //     g.reset_edges_delaunay();
        //     g2.reset_edges_delaunay_old();
        //     assert_eq!(g.links.len(), g2.links.len());
        // }
    }
}
