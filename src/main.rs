use rand::Rng;
use std::env;
use std::str::FromStr;

struct Individual {
    chromosome: Vec<u8>,
    size: usize
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

struct Database {
    transactions: Vec<Transaction>
}

impl Individual {
    fn new(size: &usize) -> Individual {
        let mut rng = rand::thread_rng();
        let chromosome: Vec<u8> = (0..*size)
            .map(|_| if rng.gen::<f32>() >= 0.5 { 1 } else { 0 })
            .collect();
        
        Individual { chromosome: chromosome, size: *size }
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
        
    }
}

fn main() {
    let min_util = 10;
    let t = Transaction::from_str(&"2 3 4:9:2 2 5", &4);
    println!("{:?}", t);
}