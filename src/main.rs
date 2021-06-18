struct Individual {
    chromosome: Vec<u8>,
    size: usize
}

impl Individual {
    fn new(size: &usize) -> Individual {
        let chromosome: Vec<u8> = (0..size)
            .map(|_| 0)
            .collect();
        
        Individual { chromosome, size }
    }
}

fn main() {
    println!("Hello, world!");
}
