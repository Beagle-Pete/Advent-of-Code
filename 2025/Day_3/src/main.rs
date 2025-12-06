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

    fn get_largest_joltage(&self) -> u32 {
        // Find largest digit. Seach from beginning to end-1
        let mut largest_first_digit_ind = 0;
        let mut largest_first_digit = self.batteries.chars().nth(largest_first_digit_ind).unwrap().to_digit(10).unwrap();
        for ii in 0..self.batteries.chars().count()-1 {
            let jolt = self.batteries.chars().nth(ii).unwrap().to_digit(10).unwrap();

            if jolt > largest_first_digit {
                largest_first_digit = jolt;
                largest_first_digit_ind = ii;
            }

            // ToDo: If 9 is found then return early
            // println!("First Digit Search: {}", jolt);
        }

        // Search for second largest digit. Search from (largest first digit + 1) to end
        let largest_second_digit_ind = largest_first_digit_ind+1;
        let mut largest_second_digit = self.batteries.chars().nth(largest_second_digit_ind).unwrap().to_digit(10).unwrap();
        for ii in largest_second_digit_ind..self.batteries.chars().count() {
            let jolt = self.batteries.chars().nth(ii).unwrap().to_digit(10).unwrap();

            if jolt > largest_second_digit {
                largest_second_digit = jolt;
            }
            // println!("Second Digit Seach: {}", jolt);
        }

        let greatest_joltage = format!("{}{}", largest_first_digit, largest_second_digit)
            .parse::<u32>()
            .unwrap();

        // println!("Largest joltage: {} jolts", greatest_joltage);

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

    fn get_joltage_sum(self) -> u32 {

        let mut sum = 0;
        for bank in &self.banks {
            sum += bank.get_largest_joltage();
        }

        sum
    }
}

fn main() {
    println!("Advent of Code (2025) - Day 3 (12/3)");

    let input_file = "data/test_input.txt".to_owned();
    let bank_collection = parse_input_file(input_file).unwrap();
    let bank_collection = BankCollection::new(bank_collection);

    let sum = &bank_collection.get_joltage_sum();
    println!("Part 1 Solution: sum = {}", sum);
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