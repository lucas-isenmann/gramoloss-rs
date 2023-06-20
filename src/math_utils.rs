use crate::coord::Coord;

/// Solve equation t u + t'v = c where u, v and c are 2d vectors
/// return false if there is no solution
pub fn solve_linear_equation_2d( u: &Coord, v: &Coord, c: &Coord) -> Option<(f64,f64)>{
    let det = u.x * v.y - u.y * v.x;
    if det == 0. {
        return None;
    }
    let t1 = (c.x * v.y - c.y * v.x )/det;
    let t2 = (c.y * u.x - c.x * u.y )/det;
    Some((t1,t2))
}


/// Given a point and triangle defined by its three corners q1, q2 and q3
/// Returns true iff point is in the triangle
pub fn is_point_in_triangle(point: &Coord, q1: &Coord, q2: &Coord, q3: &Coord) -> bool{   
    if let Some(sol) = solve_linear_equation_2d(&q1.sub(&q3), &q2.sub(&q3), &point.sub(&q3)){
        let r = sol.0;
        let s = sol.1;
        0. <= r && 0. <= s && 0. <= 1.-r-s 
    } else {
        false
    }
}

/// Returns true if the d is in the circle of a,b,c
pub fn is_in_circle(a: &Coord, b: &Coord, c: &Coord, d: &Coord) -> bool{
    let sign = 
    if (b.x - a.x)*(c.y-a.y)-(c.x -a.x)*(b.y-a.y) <= 0. {
        -1. // a b c are in clockwise order
    } else {
        1. // counter clockwise order
    };

    let mat = 
    vec![vec![a.x, a.y, a.x*a.x + a.y*a.y, 1.],
    vec![b.x, b.y, b.x*b.x + b.y*b.y, 1.],
    vec![c.x, c.y, c.x*c.x + c.y*c.y, 1.],
    vec![d.x, d.y, d.x*d.x + d.y*d.y, 1.]];

    sign*det(&mat) > 0.
}





/// Compute the determinant of the matrix.
/// Certainly buf if the matrix is empty or matrix is not square.
pub fn det(matrix: &Vec<Vec<f64>>) -> f64{
    let n = matrix.len();
    if n == 1{
        return matrix[0][0];
    }
    let mut sum = 0.;
    for i in 0..n {

        let mut sub_matrix = Vec::new();
        // slice the first column and the ith row
        for k in 1..n {
            sub_matrix.push(Vec::new());
            for j in 0..n {
                if j != i {
                    sub_matrix[k-1].push(matrix[k][j])
                }
            }
        }
        
        let sign: f64 = if i % 2 == 0 {
            1.
        } else {
            -1.
        };
        sum += sign * matrix[0][i] * det(&sub_matrix);
    }
    sum
}



#[cfg(test)]
mod tests {
    use crate::math_utils::det;


    #[test]
    fn test_det() {
        let matrix = vec![ vec![1.,1.,1.], vec![1.,0.,1.], vec![1.,1.,0.]];
        assert_eq!(det(&matrix), 1.)
    }
}