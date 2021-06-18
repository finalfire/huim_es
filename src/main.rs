struct Individual {
    chromosome: Vec<u8>,
    size: usize
}

struct Population {
    individuals: Vec<Individual>
}

impl Individual {
    fn new(size: &usize) -> Individual {
        let chromosome: Vec<u8> = (0..*size)
            .map(|_| 0)
            .collect();
        
        Individual {
            chromosome: chromosome,
            size: *size
        }
    }
}

impl Population {
    fn new(n: &usize, size: &usize) -> Population {
        let individuals: Vec<Individual> = (0..*n)
            .map(|_| Individual::new(&size))
            .collect();

        Population { individuals }
    }
}

fn main() {
    let population = Population::new(&10, &5);
    for individual in population.individuals {
        println!("{:?}", individual.chromosome);
    }
}
