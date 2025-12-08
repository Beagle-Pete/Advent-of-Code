use std::fs;
use std::error;

struct Ingredients {
    fresh: Vec<(u64, u64)>,
    available: Vec<u64>,
}

impl Ingredients {
    fn new(fresh: Vec<(u64, u64)>, available: Vec<u64>) -> Self {
        Self {
            fresh,
            available,
        }
    }

    fn fresh_count(&self) -> u64 {

        let mut sum = 0;

        for ingredient in &self.available {
            for fresh_ingredient in &self.fresh {

                if ingredient >= &fresh_ingredient.0 && ingredient <= &fresh_ingredient.1 {
                    sum += 1;
                    break;
                }
                
            }
        }
        
        sum
    }

    fn get_unique_fresh_sum(&self) -> u64 {

        let mut new_fresh = vec![];

        new_fresh.push(self.fresh[0]);

        for ii in 1..self.fresh.len() {
            let new_fresh_last_ind = new_fresh.len() - 1;
            let (start_prev, end_prev) = new_fresh[new_fresh_last_ind];
            let (start_current, end_current) = self.fresh[ii];

            let start_current_is_in_prev = start_current >= start_prev && start_current <= end_prev;

            if start_current_is_in_prev && end_current >= end_prev {
                new_fresh[new_fresh_last_ind].1 = end_current;
                continue
            } else if start_current_is_in_prev && start_current < end_prev {
                continue
            }

            new_fresh.push(self.fresh[ii]);
            
        }

        for ii in 1..new_fresh.len() {
            if new_fresh[ii].0 <= new_fresh[ii-1].1 {
                println!("ii-1: {} -> {}", new_fresh[ii-1].0, new_fresh[ii-1].1);
                println!("ii: {} -> {}\n", new_fresh[ii].0, new_fresh[ii].1);
            }
        }

        let mut sum = 0;
        for n in new_fresh {
            sum += n.1 - n.0 + 1;
        }

        sum
    }
}

fn main() {
    println!("Advent of Code (2025) - Day 5 (12/5): Cafeteria\n");

    // let input_file = "data/test_input.txt".to_owned();
    let input_file = "data/puzzle_input.txt".to_owned();

    let ingredients = parse_input_file(input_file).unwrap();

    let sum = ingredients.fresh_count();
    let sum2 = ingredients.get_unique_fresh_sum();

    println!("Part 1 Solution: {} of the available ingredients are fresh", sum);
    println!("Part 2 Solution: {} unique fresh ingredient IDs", sum2);
}


fn parse_input_file(path: String) -> Result<Ingredients, Box<dyn error::Error>> {
    let input = fs::read_to_string(path).unwrap();

    let input: Vec<String> = input.split("\n\n")
        .map(|el| el.to_string())
        .collect();
    
    let mut fresh: Vec<(u64, u64)> = input[0].split("\n")
        .map(|line| {
            let mut fresh_range = line.split("-");

            let range_start = fresh_range.next()
                .unwrap()
                .parse::<u64>()
                .unwrap();

            let range_end = fresh_range.next()
                .unwrap()
                .parse::<u64>()
                .unwrap();

            (range_start, range_end)
        })
        .collect();

    fresh.sort_by(|a, b| (a.0).cmp(&b.0));

    let available_ingredients: Vec<u64> = input[1].split("\n")
        .map(|el| el.parse::<u64>().unwrap())
        .collect();

    Ok(Ingredients::new(fresh, available_ingredients))

    
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part_1 {
        use super::*;

        #[test]
        fn fresh_ingredients_should_be_3() {

            let input_file = "data/test_input.txt".to_owned();
            let ingredients = parse_input_file(input_file).unwrap();

            assert_eq!(ingredients.fresh_count(), 3);
        }
    }

    mod part_2 {
        use super::*;

        #[test]
        fn unique_fresh_ingredients_should_be_14() {

            let input_file = "data/test_input.txt".to_owned();
            let ingredients = parse_input_file(input_file).unwrap();

            assert_eq!(ingredients.get_unique_fresh_sum(), 14);
        }
    }
}