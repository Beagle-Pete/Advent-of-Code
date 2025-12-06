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

    fn get_largest_joltage(&self, digits: usize) -> u64 {
        let bank_size = self.batteries.chars().count();

        let mut largest_digits = "".to_owned();

        let mut prev_largest_digit_ind: usize = 0;
        let mut start_ind = 1;
        let mut prev_largest_digit = self.batteries.chars().nth(prev_largest_digit_ind).unwrap().to_digit(10).unwrap();
        for ii in 0..digits {

            for jj in start_ind..=(bank_size - digits + ii) {
                let jolt = self.batteries.chars().nth(jj).unwrap().to_digit(10).unwrap();

                if jolt > prev_largest_digit {
                    prev_largest_digit = jolt;
                    prev_largest_digit_ind = jj;
                }
            }
            // ToDo: If 9 is found then return early
            // ToDo: If remaining index to search is equal to digits left then return remaining string and return early

            largest_digits = format!("{}{}", largest_digits, prev_largest_digit);

            start_ind = prev_largest_digit_ind+1;
            prev_largest_digit = 0;
        }

        let greatest_joltage = largest_digits.parse::<u64>().unwrap();

        greatest_joltage
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

    fn get_joltage_sum(&self, digits: usize) -> u64 {

        let mut sum = 0;
        for bank in &self.banks {
            sum += bank.get_largest_joltage(digits);
        }

        sum
    }
}

fn main() {
    println!("Advent of Code (2025) - Day 3 (12/3)");

    // let input_file = "data/test_input.txt".to_owned();
    let input_file = "data/puzzle_input.txt".to_owned();
    let bank_collection = parse_input_file(input_file).unwrap();
    let bank_collection = BankCollection::new(bank_collection);

    let sum_part1 = &bank_collection.get_joltage_sum(2);
    let sum_part2 = &bank_collection.get_joltage_sum(12);
    println!("Part 1 Solution: sum = {}", sum_part1);
    println!("Part 2 Solution: sum = {}", sum_part2);
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

    mod part_1 {
        use super::*;

        #[test]
        fn test_bank_largest_joltage() {
            let bank1 = Bank::new("987654321111111".to_owned());
            let bank2 = Bank::new("811111111111119".to_owned());
            let bank3 = Bank::new("234234234234278".to_owned());
            let bank4 = Bank::new("818181911112111".to_owned());

            assert_eq!(bank1.get_largest_joltage(2), 98);
            assert_eq!(bank2.get_largest_joltage(2), 89);
            assert_eq!(bank3.get_largest_joltage(2), 78);
            assert_eq!(bank4.get_largest_joltage(2), 92);
        }

        #[test]
        fn test_joltage_sum() {

            let input_file = "data/test_input.txt".to_owned();
            let bank_collection = parse_input_file(input_file).unwrap();
            let bank_collection = BankCollection::new(bank_collection);
            let digits = 2;

            assert_eq!(bank_collection.get_joltage_sum(digits), 357);
        }
    }

    mod part_2 {
        use super::*;

        #[test]
        fn test_bank_largest_joltage() {
            let bank1 = Bank::new("987654321111111".to_owned());
            let bank2 = Bank::new("811111111111119".to_owned());
            let bank3 = Bank::new("234234234234278".to_owned());
            let bank4 = Bank::new("818181911112111".to_owned());

            assert_eq!(bank1.get_largest_joltage(12), 987654321111);
            assert_eq!(bank2.get_largest_joltage(12), 811111111119);
            assert_eq!(bank3.get_largest_joltage(12), 434234234278);
            assert_eq!(bank4.get_largest_joltage(12), 888911112111);
        }

        #[test]
        fn test_joltage_sum() {

            let input_file = "data/test_input.txt".to_owned();
            let bank_collection = parse_input_file(input_file).unwrap();
            let bank_collection = BankCollection::new(bank_collection);
            let digits = 12;

            assert_eq!(bank_collection.get_joltage_sum(digits), 3121910778619);
        }
    }
}