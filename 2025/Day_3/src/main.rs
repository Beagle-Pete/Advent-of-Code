use std::fs;
use std::error;

#[derive(Debug)]
struct Bank {
    batteries: String
}

impl Bank {
    fn new(batteries: String) -> Self {
        Self {
            batteries
        }
    }

    fn get_largest_joltage(&self) -> u64 {

        0
    }
}

#[derive(Debug)]
struct BankCollection {
    banks: Vec<Bank>
}

impl BankCollection {
    fn new(banks: Vec<Bank>) -> Self {
        Self {
            banks
        }
    }

    fn get_joltage_sum(self) -> u64 {

        0
    }
}

fn main() {
    println!("Hello, world!");

    let input_file = "data/test_input.txt".to_owned();
    let bank_collection = parse_input_file(input_file).unwrap();
    let bank_collection = BankCollection::new(bank_collection);

    println!("{:?}", bank_collection);
}

fn parse_input_file(file_path: String) -> Result<Vec<Bank>, Box<dyn error::Error>> {
    let input = fs::read_to_string(file_path)?;

    let input: Vec<Bank> = input.split("\n")
        .map(|line| Bank::new(line.to_string()))
        .collect();

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_largest_joltage() {
        let bank1 = Bank::new("987654321111111".to_owned());
        let bank2 = Bank::new("811111111111119".to_owned());
        let bank3 = Bank::new("234234234234278".to_owned());
        let bank4 = Bank::new("818181911112111".to_owned());

        assert_eq!(bank1.get_largest_joltage(), 98);
        assert_eq!(bank2.get_largest_joltage(), 89);
        assert_eq!(bank3.get_largest_joltage(), 78);
        assert_eq!(bank4.get_largest_joltage(), 92);
    }

    #[test]
    fn test_joltage_sum() {

        let input_file = "data/test_input.txt".to_owned();
        let bank_collection = parse_input_file(input_file).unwrap();
        let bank_collection = BankCollection::new(bank_collection);

        assert_eq!(bank_collection.get_joltage_sum(), 357);
    }
}