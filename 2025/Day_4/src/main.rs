use std::fs;
use std::error;

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self {
            grid
        }
    }

    fn get_accessible_rolls(&self) -> (u64, Vec<Vec<char>>) {

        let mut new_grid = self.grid.clone();
        let mut accessible_rolls = 0;

        for ii in 0..self.grid.len() {
            let current_row = self.grid[ii].clone();
            let prev_row;
            let next_row;

            if ii == 0 {
                prev_row = vec!['.'; current_row.len()];
                next_row = self.grid[ii+1].clone();
            } else if ii == self.grid.len()-1 {
                prev_row = self.grid[ii-1].clone();
                next_row = vec!['.'; current_row.len()];
            } else {
                prev_row = self.grid[ii-1].clone();
                next_row = self.grid[ii+1].clone();
            }

            for jj in 0..current_row.len() {
                // Numpad positioning
                // 789
                // 456
                // 123
                // -> [789456123]
                // Position 5 is not counted

                // Col jj-1
                let mut pos7 = '.';
                let mut pos4 = '.';
                let mut pos1 = '.';

                // Col jj
                let pos8 = prev_row[jj];
                let pos5 = current_row[jj];
                let pos2 = next_row[jj];

                // Col jj+1
                let mut pos9 = '.';
                let mut pos6 = '.';
                let mut pos3 = '.';
                
                if jj != 0 {
                    pos7 = prev_row[jj-1];
                    pos4 = current_row[jj-1];
                    pos1 = next_row[jj-1];
                }
                
                if jj != current_row.len()-1 {
                    pos9 = prev_row[jj+1];
                    pos6 = current_row[jj+1];
                    pos3 = next_row[jj+1];
                }

                let adjacent_space = vec![pos7,pos8,pos9,pos4,pos6,pos1,pos2,pos3];

                let mut sum = 0;

                adjacent_space.iter().for_each(|el| {
                    if el == &'@' {
                        sum +=1
                    }
                });

                if pos5 == 'x' {
                    new_grid[ii][jj] = '.';
                }

                if pos5 == '@' && sum < 4 {
                    accessible_rolls += 1;
                    new_grid[ii][jj] = 'x';
                }

                // println!("Row: {}, Col: {} -> {}", ii, jj, sum);
                // println!("{:?}", adjacent_space);
            }
        }

        (accessible_rolls, new_grid)
    }
}

fn main() {
    println!("Advent of Code (2025) - Day 4 (12/4): Printing Department\n");

    let input_file = "data/test_input.txt".to_owned();
    // let input_file = "data/puzzle_input.txt".to_owned();
    let grid = parse_input_file(input_file).unwrap();
    let (accessible_rolls, new_grid) = grid.get_accessible_rolls();

    // Example of how to re-consume answer and converge on final solution. Build on this
    let grid2 = Grid::new(new_grid);
    let (accessible_rolls2, new_grid2) = grid2.get_accessible_rolls();

    println!("Part 1 Solution: Can remove {} rolls of paper", accessible_rolls);

}

fn parse_input_file(file_path: String) -> Result<Grid, Box<dyn error::Error>> {
    let input = fs::read_to_string(file_path)?;

    let input: Vec<Vec<char>> = input.split("\n")
        .map(|line| {
            line.to_string()
            .chars()
            .collect()            
        })
        .collect();

    Ok(Grid::new(input))

}