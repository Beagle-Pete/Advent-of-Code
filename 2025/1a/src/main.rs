use std::fs;
use std::error;

fn main() {
    println!("Advent of Code (2025) - Day 1 (12/1)");

    let start_pos = 50;
    // let input_file_path = "data/test_input.txt".to_owned();
    let input_file_path = "data/puzzle_input.txt".to_owned();

    let direction = parse_input(input_file_path).unwrap();
    let mut final_val: Vec<i32> = vec![];
    let mut zero_count = 0;

    let mut point = start_pos;
    for x in &direction {

        let point_test = point + x;
        let mut rem = 0;

        if point_test < 0 {
            
            if point == 0 {
                rem = *x%100;
            } else {
                rem = (x+point)%100;
            }

            point = 100 - rem.abs();
        } else if point_test > 100 {
            rem = point_test%100;
            point = rem.abs();
        } else {
            point = point + x%100;
        }
        
        if point == 100 {
            point = 0;
        }

        if point == 0 {
            zero_count += 1;
        }

        // Panic if a value is <0 or >100
        if point < 0 || point > 99 {
            panic!("Point: {} at direction {}, point_test: {}", point, x, point_test);
        }

        final_val.push(point);
    }

    println!("Zero Count: {}", zero_count);
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
            'R' => direction.push(rest),
            'L' => direction.push(-1*rest),
            _ => panic!("Unrecognized prefix: {}", first_char),
        }
        
    }
    
    Ok(direction)
}