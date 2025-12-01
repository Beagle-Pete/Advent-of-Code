use std::fs;
use std::error;

fn main() {
    println!("Advent of Code (2025) - Day 1 (12/1)");

    let start_pos = 50;
    // let input_file_path = "data/test_input.txt".to_owned();
    let input_file_path = "data/puzzle_input.txt".to_owned();

    let dial_clicks = parse_input(input_file_path).expect("Failed to parse input file");

    let zero_count = stops_at_zero_count(&start_pos, &dial_clicks);

    println!();
    println!("Part 1: ");
    println!("Zero Count: {}", zero_count);

    println!();
    println!("Part 2:");
    println!("Zero Count:");
}

fn parse_input(file_path: String) -> Result<Vec<i32>, Box<dyn error::Error>> {

    let input = fs::read_to_string(file_path);

    let input = match input {
        Ok(val) => val,
        Err(_) => panic!("Could not open file")
    };

    let input: Vec<&str> = input.split("\n").collect();
    let mut direction: Vec<i32> = vec![];

    for x in &input {
        let mut x_chars = x.chars();

        let first_char = x_chars.next();

        let first_char = match first_char {
            Some(val) => val,
            None => continue
        };

        let rest: String = x_chars.collect();
        let rest = rest.parse::<i32>()?;

        match first_char {
            'L' => direction.push(-1*rest),
            'R' => direction.push(rest),
            _ => panic!("Unrecognized prefix: {}", first_char),
        }
        
    }
    
    Ok(direction)
}

fn stops_at_zero_count(start_pos: &i32, dial_clicks: &Vec<i32>) -> i32 {

    let mut pos = start_pos.clone();
    let mut zero_count = 0;
    let mut final_val: Vec<i32> = vec![];

    for dial_click in dial_clicks {

        let new_pos = pos + dial_click;
        let mut rem = 0;

        if new_pos < 0 {            
            rem = (dial_click+pos)%100;
            pos = 100 - rem.abs();

        } else if new_pos > 100 {
            rem = new_pos%100;
            pos = rem.abs();

        } else {
            pos = pos + dial_click%100;
        }
        
        if pos == 100 {
            pos = 0;
        }

        if pos == 0 {
            zero_count += 1;
        }

        // Panic if a value is <0 or >100
        if pos < 0 || pos > 99 {
            panic!("Pos: {} -> Dial Clicks {} -> New Pos: {}", pos, dial_click, new_pos);
        }

        final_val.push(pos);
    }

    zero_count

}