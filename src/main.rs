use rand::Rng;
use std::env;
use std::fs;
use std::str::FromStr;

struct Individual {
    chromosome: Vec<u8>,
    size: usize,
    fitness: u32
}

struct Population {
    individuals: Vec<Individual>
}

#[derive(Debug)]
struct Transaction {
    items: Vec<u8>,
    utilities: Vec<u32>,
    utility: u32
}

#[derive(Debug)]
struct Database {
    size: usize,
    transactions: Vec<Transaction>
}

impl Individual {
    fn new(size: &usize) -> Individual {
        let mut rng = rand::thread_rng();
        let chromosome: Vec<u8> = (0..*size)
            .map(|_| if rng.gen::<f32>() >= 0.5 { 1 } else { 0 })
            .collect();
        
        Individual { chromosome: chromosome, size: *size, fitness: 0 }
    }

    fn compute_fitness(&mut self, db: &Database) {
        self.fitness = db.transactions.iter()
            .map(|t| {
                let mut tdx = 0;
                for i in 0..self.chromosome.len() {
                    if self.chromosome[i] == 1 {
                        if t.items[i] == 0 {
                            return 0;
                        }
                        tdx += t.utilities[i];
                    }
                }
                tdx
            }).sum();
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

impl Transaction {
    fn from_str(data: &str, size: &usize) -> Transaction {
        let mut items: Vec<u8> = vec![0; *size];
        let mut utilities: Vec<u32> = vec![0; *size];

        let parts: Vec<&str> = data.split(':').collect();
        let utility = u32::from_str(parts[1]).expect("Error parsing the total utility.");
        for (item_index, utility_value) in parts[0].split(' ').zip(parts[2].split(' ')) {
            let index = usize::from_str(item_index).expect("Error parsing the item index.");
            items[index - 1] = 1;
            utilities[index -1] = u32::from_str(utility_value).expect("Error parsing the utility value.");
        }

        Transaction { items, utilities, utility }
    }
}

impl Database {
    fn from_file(path: &str) -> Database {
        let file_content = fs::read_to_string(path).expect("Error opening the input file.");
        let parts: Vec<&str> = file_content.split('\n').collect();

        let size = usize::from_str(parts[0]).expect("Error parsing the total utility.");
        let transactions = parts[1..].into_iter().map(|t| Transaction::from_str(t, &size)).collect();

        Database { size, transactions }
    }
}

fn main() {
    let min_util = 10;
    let d = Database::from_file("input.txt");
    println!("{:?}", d);
}