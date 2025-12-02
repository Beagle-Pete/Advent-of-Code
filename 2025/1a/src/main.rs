use std::fs;
use std::error;
use std::cmp;

fn main() {
    println!("Advent of Code (2025) - Day 1 (12/1)");

    let start_pos = 50;
    // let input_file_path = "data/test_input.txt".to_owned();
    let input_file_path = "data/puzzle_input.txt".to_owned();

    let dial_clicks = parse_input(input_file_path).expect("Failed to parse input file");

    let zero_count = stops_at_zero_count(&start_pos, &dial_clicks);
    let zero_count2 = stops_or_passes_zero_count(&start_pos, &dial_clicks);

    println!();
    println!("Part 1: ");
    println!("Stop at Zero Count: {}", zero_count);

    println!();
    println!("Part 2:");
    println!("Stop or Pass by Zero Count: {}", zero_count2);
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

        let mut new_pos = pos + dial_click;
        let rem = new_pos % 100;

        if new_pos < 0 {
            new_pos = 100 + rem;
        } else if new_pos > 100 {
            new_pos = rem;
        }
        
        if new_pos == 100 {
            new_pos = 0;
        }

        if new_pos == 0 {
            zero_count += 1;
        }

        // Panic if a new position is <0 or >100
        if new_pos < 0 || new_pos > 99 {
            panic!("Pos: {} -> Dial Clicks {} -> New Pos: {}", pos, dial_click, new_pos);
        }

        pos = new_pos;
        final_val.push(pos);
    }

    zero_count

}

fn stops_or_passes_zero_count(start_pos: &i32, dial_clicks: &Vec<i32>) -> i32 {

    let mut pos = start_pos.clone();
    let mut zero_count = 0;
    let mut pass_by_zero_count = 0;
    let mut final_val: Vec<i32> = vec![];

    for dial_click in dial_clicks {

        let mut new_pos = pos + dial_click;
        let rem = new_pos % 100;
        // let pass_zero = if rem == 0 {
        //     (new_pos / 100) - 1
        // } else {
        //     new_pos / 100
        // };
        let mut pass_zero = 0;

        if new_pos < 0 {
            if rem == 0 {
                pass_zero = (new_pos.abs() / 100);
                if pos == 0 {
                    pass_zero = (new_pos.abs() / 100);
                }
                
                pass_by_zero_count += pass_zero;
            } else {
                pass_zero = (new_pos.abs() / 100) + 1;
                if pos == 0 {
                    pass_zero = (new_pos.abs() / 100);
                }
                pass_by_zero_count += pass_zero;
            }
            new_pos = 100 + rem;
        } else if new_pos > 100 {
            if rem == 0 {
                pass_zero += (new_pos.abs() / 100) - 1;
                pass_by_zero_count += pass_zero;
            } else {
                pass_zero = (new_pos.abs() / 100);
                pass_by_zero_count += pass_zero;
            }
            new_pos = rem;
            // pass_by_zero_count += cmp::max(pass_zero - 1, 0);
        }
        
        if new_pos == 100 {
            new_pos = 0;
        }

        if new_pos == 0 {
            zero_count += 1;
        }

        // Panic if a new position is <0 or >100
        if new_pos < 0 || new_pos > 99 {
            panic!("Pos: {} -> Dial Clicks {} -> New Pos: {}", pos, dial_click, new_pos);
        }

        // println!("Pass Zero Count: {}", pass_zero);
        println!("Start: {} -> Dial: {} -> Pos: {} -> Pass 0: {}", pos, dial_click, pos + dial_click, pass_zero);

        pos = new_pos;
        final_val.push(pos);
    }

    println!("{:?}", dial_clicks);
    println!("{:?}", final_val);
    println!("Zero Count: {}", zero_count);
    println!("Pass by Zero Count: {}", pass_by_zero_count);
    zero_count+pass_by_zero_count

}