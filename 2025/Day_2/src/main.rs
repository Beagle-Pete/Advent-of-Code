use std::fs;
use std::error;

fn main() {
    println!("Advent of Code (2025) - Day 2 (12/2)");

    // let input_file_path = "data/test_input.txt".to_owned();
    let input_file_path = "data/puzzle_input.txt".to_owned();

    let input_file = parse_input(input_file_path).unwrap();

    let mut invalid_ids: Vec<String> = vec!{};

    for input in &input_file {
        let ids = input.start..=input.end;

        for id in ids {
            let id = id.to_string();

            // If id has odd number of digits continue
            if id.len() % 2 == 1 {
                continue
            }

            let ind_halve = id.len() / 2;

            let part1 = &id[0..=ind_halve-1];
            let part2 = &id[ind_halve..];
            
            if part1==part2 {
                invalid_ids.push(id);
            }
        }
    }

    // Sum of invalid IDs
    let mut sum = 0;
    for id in &invalid_ids {
        let id = id.parse::<u64>().unwrap();
        sum += id;
    }
    
    println!("Sum of invalid IDs: {:#?}", sum);

}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self {
            start,
            end,
        }
    }
}

fn parse_input(file_path: String) -> Result<Vec<Range>, Box<dyn error::Error>> {

    let input = fs::read_to_string(file_path)?;

    let input: Vec<Range> = input.split(",")
        .map(|el| {
            let mut el_split = el.split("-");
            let start = el_split.next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let end = el_split.next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            Range::new(start, end)
        })
        .collect();

    Ok(input)
}