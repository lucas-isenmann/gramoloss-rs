


pub fn print_adj(adj: &Vec<Vec<bool>>){
    for row in adj {
        for &cell in row {
            if cell {
                print!("1");
            } else {
                print!(".")
            }
        }
        println!();
    }
}