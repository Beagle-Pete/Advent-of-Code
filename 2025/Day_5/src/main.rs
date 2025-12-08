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

                if ingredient > &fresh_ingredient.0 && ingredient < &fresh_ingredient.1 {
                    sum += 1;
                    break;
                }
                // println!("{} -> {}", fresh_ingredient.0, fresh_ingredient.1);
            }
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

    println!("Part 1 Solution: {} of the available ingredients are fresh", sum);

    // println!("{:?}", ingredients.fresh)
}


fn parse_input_file(path: String) -> Result<Ingredients, Box<dyn error::Error>> {
    let input = fs::read_to_string(path).unwrap();

    let input: Vec<String> = input.split("\n\n")
        .map(|el| el.to_string())
        .collect();
    
    let fresh = input[0].split("\n")
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
}