use std::fs;
use std::error;

fn main() {
    println!("Advent of Code (2025) - Day 2 (12/2)");

    // let input_file_path = "data/test_input.txt".to_owned();
    let input_file_path = "data/puzzle_input.txt".to_owned();

    let input_file = parse_input(input_file_path).unwrap();

    let sum = get_invalid_ids(&input_file);    
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

fn get_invalid_ids(input_file: &Vec<Range>) -> u64 {

    let mut invalid_ids: Vec<String> = vec!{};

    for input in input_file {
        let ids = input.start..=input.end;

        for id in ids {
            let invalid_id = test_for_pattern(id);

            match invalid_id {
                Some(id) => invalid_ids.push(id.to_string()),
                None => continue
            }
        }
    }

    // Sum of invalid IDs
    let mut sum = 0;
    for id in &invalid_ids {
        let id = id.parse::<u64>().unwrap();
        sum += id;
    }

    sum
}

fn test_for_pattern(id: u64) -> Option<u64> {
    let id_str = id.to_string();
    println!("{}", id_str);

    // If id has odd number of digits continue
    if id_str.len() % 2 == 1 {
        return None;
    }

    let ind_halve = id_str.len() / 2;

    let part1 = &id_str[0..=ind_halve-1];
    let part2 = &id_str[ind_halve..];
    
    if part1!=part2 {
        return None;
    }

    Some(id)
}