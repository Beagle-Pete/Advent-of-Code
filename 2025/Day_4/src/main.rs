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

    fn converge(&self) -> (u64, Vec<u64>, Vec<Vec<Vec<char>>>) {
        let mut grid_history = vec![];
        let mut accessible_rolls_history = vec![];

        let (accessible_rolls, new_grid) = self.get_accessible_rolls();
        grid_history.push(self.grid.clone());
        grid_history.push(new_grid.clone());
        accessible_rolls_history.push(accessible_rolls);

        let mut grid_ii= Grid::new(new_grid.clone());
        let mut iter = 0;
        loop {
            iter += 1;
            let (accessible_rolls_ii, new_grid_ii) = grid_ii.get_accessible_rolls();
            
            grid_history.push(new_grid_ii.clone());
            accessible_rolls_history.push(accessible_rolls_ii);

            grid_ii = Grid::new(new_grid_ii);

            // println!("Iter: {}, Rolls: {}", iter, accessible_rolls_ii);
            if accessible_rolls_ii == 0 || iter == 1000 {
                break
            }
        }

        let mut total_sum = 0;
        for ii in &accessible_rolls_history {
            total_sum += ii;
        }

        (total_sum, accessible_rolls_history, grid_history)
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

    // let input_file = "data/test_input.txt".to_owned();
    let input_file = "data/puzzle_input.txt".to_owned();
    let grid = parse_input_file(input_file).unwrap();
    let (accessible_rolls, _new_grid) = grid.get_accessible_rolls();
    let (total_sum, _accessible_rolls2, _new_grid2) = grid.converge();

    println!("Part 1 Solution: Can remove {} rolls of paper", accessible_rolls);
    println!("Part 2 Solution: Can remove {} rolls of paper in total", total_sum);

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

#[cfg(test)]
mod tests {
    use super::*;

    mod part_1 {
        use super::*;

        #[test]
        fn test_accessible_rolls() {
            let input_file = "data/test_input.txt".to_owned();
            let grid = parse_input_file(input_file).unwrap();
            let (accessible_rolls, _new_grid) = grid.get_accessible_rolls();

            assert_eq!(accessible_rolls, 13);
        }

        #[test]
        fn test_new_grid() {
            let input_file = "data/test_input.txt".to_owned();
            let solution_file = "data/test_input_sol_part1.txt".to_owned();
            let grid_starting = parse_input_file(input_file).unwrap();
            let grid_final = parse_input_file(solution_file).unwrap();
            let (_accessible_rolls, new_grid) = grid_starting.get_accessible_rolls();

            assert_eq!(new_grid, grid_final.grid);
        }
    }

    mod part_2 {
        use super::*;

        #[test]
        fn test_total_sum_of_accessible_rolls() {
            let input_file = "data/test_input.txt".to_owned();
            let grid = parse_input_file(input_file).unwrap();
            let (total_sum, _accessible_rolls, _new_grid) = grid.converge();

            assert_eq!(total_sum, 43);
        }

        #[test]
        fn test_accessible_rolls_history() {
            let input_file = "data/test_input.txt".to_owned();
            let solution_file_2a = "data/test_input_sol_part2a.txt".to_owned();
            let solution_file_2b = "data/test_input_sol_part2b.txt".to_owned();
            let solution_file_2c = "data/test_input_sol_part2c.txt".to_owned();
            let solution_file_2d = "data/test_input_sol_part2d.txt".to_owned();
            let solution_file_2e = "data/test_input_sol_part2e.txt".to_owned();
            let solution_file_2f = "data/test_input_sol_part2f.txt".to_owned();
            let solution_file_2g = "data/test_input_sol_part2g.txt".to_owned();
            let solution_file_2h = "data/test_input_sol_part2h.txt".to_owned();
            let solution_file_2i = "data/test_input_sol_part2i.txt".to_owned();

            let grid_start = parse_input_file(input_file).unwrap();
            let grid_2a = parse_input_file(solution_file_2a).unwrap();
            let grid_2b = parse_input_file(solution_file_2b).unwrap();
            let grid_2c = parse_input_file(solution_file_2c).unwrap();
            let grid_2d = parse_input_file(solution_file_2d).unwrap();
            let grid_2e = parse_input_file(solution_file_2e).unwrap();
            let grid_2f = parse_input_file(solution_file_2f).unwrap();
            let grid_2g = parse_input_file(solution_file_2g).unwrap();
            let grid_2h = parse_input_file(solution_file_2h).unwrap();
            let grid_2i = parse_input_file(solution_file_2i).unwrap();

            let (accessible_rolls_start, _new_grid) = grid_start.get_accessible_rolls();
            let (accessible_rolls_2a, _new_grid_2a) = grid_2a.get_accessible_rolls();
            let (accessible_rolls_2b, _new_grid_2b) = grid_2b.get_accessible_rolls();
            let (accessible_rolls_2c, _new_grid_2c) = grid_2c.get_accessible_rolls();
            let (accessible_rolls_2d, _new_grid_2d) = grid_2d.get_accessible_rolls();
            let (accessible_rolls_2e, _new_grid_2e) = grid_2e.get_accessible_rolls();
            let (accessible_rolls_2f, _new_grid_2f) = grid_2f.get_accessible_rolls();
            let (accessible_rolls_2g, _new_grid_2g) = grid_2g.get_accessible_rolls();
            let (accessible_rolls_2h, _new_grid_2h) = grid_2h.get_accessible_rolls();
            let (accessible_rolls_2i, _new_grid_2i) = grid_2i.get_accessible_rolls();

            assert_eq!(accessible_rolls_start, 13);
            assert_eq!(accessible_rolls_2a, 12);
            assert_eq!(accessible_rolls_2b, 7);
            assert_eq!(accessible_rolls_2c, 5);
            assert_eq!(accessible_rolls_2d, 2);
            assert_eq!(accessible_rolls_2e, 1);
            assert_eq!(accessible_rolls_2f, 1);
            assert_eq!(accessible_rolls_2g, 1);
            assert_eq!(accessible_rolls_2h, 1);
            assert_eq!(accessible_rolls_2i, 0);
        }

        #[test]
        fn test_grid_history() {
            let input_file = "data/test_input.txt".to_owned();
            let solution_file_2a = "data/test_input_sol_part2a.txt".to_owned();
            let solution_file_2b = "data/test_input_sol_part2b.txt".to_owned();
            let solution_file_2c = "data/test_input_sol_part2c.txt".to_owned();
            let solution_file_2d = "data/test_input_sol_part2d.txt".to_owned();
            let solution_file_2e = "data/test_input_sol_part2e.txt".to_owned();
            let solution_file_2f = "data/test_input_sol_part2f.txt".to_owned();
            let solution_file_2g = "data/test_input_sol_part2g.txt".to_owned();
            let solution_file_2h = "data/test_input_sol_part2h.txt".to_owned();
            let solution_file_2i = "data/test_input_sol_part2i.txt".to_owned();

            let grid_start = parse_input_file(input_file).unwrap();
            let grid_2a = parse_input_file(solution_file_2a).unwrap();
            let grid_2b = parse_input_file(solution_file_2b).unwrap();
            let grid_2c = parse_input_file(solution_file_2c).unwrap();
            let grid_2d = parse_input_file(solution_file_2d).unwrap();
            let grid_2e = parse_input_file(solution_file_2e).unwrap();
            let grid_2f = parse_input_file(solution_file_2f).unwrap();
            let grid_2g = parse_input_file(solution_file_2g).unwrap();
            let grid_2h = parse_input_file(solution_file_2h).unwrap();
            let grid_2i = parse_input_file(solution_file_2i).unwrap();

            let (_accessible_rolls, new_grid) = grid_start.get_accessible_rolls();
            let (_accessible_rolls, new_grid_2a) = grid_2a.get_accessible_rolls();
            let (_accessible_rolls, new_grid_2b) = grid_2b.get_accessible_rolls();
            let (_accessible_rolls, new_grid_2c) = grid_2c.get_accessible_rolls();
            let (_accessible_rolls, new_grid_2d) = grid_2d.get_accessible_rolls();
            let (_accessible_rolls, new_grid_2e) = grid_2e.get_accessible_rolls();
            let (_accessible_rolls, new_grid_2f) = grid_2f.get_accessible_rolls();
            let (_accessible_rolls, new_grid_2g) = grid_2g.get_accessible_rolls();
            let (_accessible_rolls, new_grid_2h) = grid_2h.get_accessible_rolls();

            assert_eq!(grid_2a.grid, new_grid);
            assert_eq!(grid_2b.grid, new_grid_2a);
            assert_eq!(grid_2c.grid, new_grid_2b);
            assert_eq!(grid_2d.grid, new_grid_2c);
            assert_eq!(grid_2e.grid, new_grid_2d);
            assert_eq!(grid_2f.grid, new_grid_2e);
            assert_eq!(grid_2g.grid, new_grid_2f);
            assert_eq!(grid_2h.grid, new_grid_2g);
            assert_eq!(grid_2i.grid, new_grid_2h);
        }
    }
}