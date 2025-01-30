

pub struct BitwiseAdjacencyMatrix {
    data: Vec<usize>,
    in_degree: Vec<usize>,
    out_degree: Vec<usize>,
    un_degree: Vec<usize>,
    powers_of_2: Vec<usize>,
}


impl BitwiseAdjacencyMatrix {
    fn new(n: usize) -> Self {
        let data = vec![0; n * n];
        let powers_of_2 = (0..n).map(|i| 1 << i).collect::<Vec<usize>>();

        BitwiseAdjacencyMatrix { data, 
            in_degree: vec![0; n], 
            out_degree: vec![0; n], 
            un_degree: vec![n-1; n], 
            powers_of_2 }
    }

    fn add_arc(&mut self, i: usize, j: usize) {
        self.data[i] |= self.powers_of_2[j];
        self.in_degree[j] += 1;
        self.out_degree[i] += 1;
        self.un_degree[j] -= 1;
        self.un_degree[i] -= 1;
    }

    fn delete_arc(&mut self, i: usize, j: usize) {
        self.data[i] &= !self.powers_of_2[j];
        self.in_degree[j] -= 1;
        self.out_degree[i] -= 1;
        self.un_degree[j] += 1;
        self.un_degree[i] += 1;
    }

    fn has_arc(&self, i: usize, j: usize) -> bool {
        (self.data[i] & self.powers_of_2[j]) != 0
    }

    fn in_degree(&self, i: usize) -> usize {
        self.in_degree[i]
    }



}
