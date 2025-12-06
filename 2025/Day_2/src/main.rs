use std::fs;
use std::error;

fn main() {
    println!("Advent of Code (2025) - Day 2 (12/2)");

    // let input_file_path = "data/test_input.txt".to_owned();
    let input_file_path = "data/puzzle_input.txt".to_owned();

    let input_file = parse_input(input_file_path).unwrap();

    let ranges_part1 = RangeCollection::new(input_file.clone());
    let ranges_part2 = RangeCollection::new(input_file.clone());
    println!("Part 1 Solution: sum = {}", ranges_part1.get_invalid_ids(Part::Part1));
    println!("Part 2 Solution: sum = {}", ranges_part2.get_invalid_ids(Part::Part2));
}

enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone)]
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

    fn get_invalid_ids(self, part: &Part) -> (u64, Vec<String>) {
        let ids = self.start..=self.end;
        let mut invalid_ids: Vec<String> = vec!{};

        for id in ids {
            let invalid_id = match part {
                Part::Part1 => self.test_for_pattern(id),
                Part::Part2 => self.test_for_pattern2(id)
            };

            match invalid_id {
                Some(id) => invalid_ids.push(id.to_string()),
                None => continue
            }
        }

        let mut sum = 0;
        for id in &invalid_ids {
            let id = id.parse::<u64>().unwrap();
            sum += id;
        }

        (sum, invalid_ids)
    }

    fn test_for_pattern(&self, id: u64) -> Option<u64> {
        let id_str = id.to_string();

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

    fn test_for_pattern2(&self, id: u64) -> Option<u64> {
        let id_str = id.to_string();

        for div in 2..=id_str.len() {

            // If not divisible continue
            if id_str.len() % div != 0 {
                continue
            }        
            
            let mut parts = vec![];
            for ii in 1..=div {
                let ind_start = (ii-1) * (id_str.len()/div);
                let ind_end = (ii) * (id_str.len()/div);
                let part = &id_str[ind_start..ind_end];
                parts.push(part);
            }
            
            let mut pattern_match_found = true;
            for ii in 0..parts.len()-1 {
                if parts[ii] != parts[ii+1] {                
                    pattern_match_found = false;
                    break
                }
            }

            if !pattern_match_found {
                continue
            }

            return Some(id);
        }

        None
    }
}

struct RangeCollection {
    ranges: Vec<Range>,
}

impl RangeCollection {
    fn new(ranges: Vec<Range>) -> Self {
        Self {
            ranges,
        }
    }

    fn get_invalid_ids(self, part: Part) -> u64 {

        let mut total_sum = 0;
        for range in self.ranges {
            let (sum, _invalid_ids) = range.get_invalid_ids(&part);
            total_sum += sum;
        }

        total_sum
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_ids_part1() {
        let part = Part::Part1;
        let range1 = Range::new(11, 22).get_invalid_ids(&part);
        let range2 = Range::new(95, 115).get_invalid_ids(&part);
        let range3 = Range::new(998, 1012).get_invalid_ids(&part);
        let range4 = Range::new(1188511880, 1188511890).get_invalid_ids(&part);
        let range5 = Range::new(222220, 222224).get_invalid_ids(&part);
        let range6 = Range::new(1698522, 1698528).get_invalid_ids(&part);
        let range7 = Range::new(446443, 446449).get_invalid_ids(&part);
        let range8 = Range::new(38593856, 38593862).get_invalid_ids(&part);
        let range9 = Range::new(565653, 565659).get_invalid_ids(&part);
        let range10 = Range::new(824824821, 824824827).get_invalid_ids(&part);
        let range11 = Range::new(2121212118, 2121212124).get_invalid_ids(&part);

        assert_eq!(range1.1, ["11", "22"]);
        assert_eq!(range1.1, ["11", "22"]);
        assert_eq!(range2.1, ["99"]);
        assert_eq!(range3.1, ["1010"]);
        assert_eq!(range4.1, ["1188511885"]);
        assert_eq!(range5.1, ["222222"]);
        assert_eq!(range6.1, vec![] as Vec<String>);
        assert_eq!(range7.1, ["446446"]);
        assert_eq!(range8.1, ["38593859"]);
        assert!(range9.1.is_empty());
        assert!(range10.1.is_empty());
        assert!(range11.1.is_empty());
    }

    #[test]
    fn test_invalid_ids_part2() {
        let part = Part::Part2;
        let range1 = Range::new(11, 22).get_invalid_ids(&part);
        let range2 = Range::new(95, 115).get_invalid_ids(&part);
        let range3 = Range::new(998, 1012).get_invalid_ids(&part);
        let range4 = Range::new(1188511880, 1188511890).get_invalid_ids(&part);
        let range5 = Range::new(222220, 222224).get_invalid_ids(&part);
        let range6 = Range::new(1698522, 1698528).get_invalid_ids(&part);
        let range7 = Range::new(446443, 446449).get_invalid_ids(&part);
        let range8 = Range::new(38593856, 38593862).get_invalid_ids(&part);
        let range9 = Range::new(565653, 565659).get_invalid_ids(&part);
        let range10 = Range::new(824824821, 824824827).get_invalid_ids(&part);
        let range11 = Range::new(2121212118, 2121212124).get_invalid_ids(&part);

        assert_eq!(range1.1, ["11", "22"]);
        assert_eq!(range2.1, ["99", "111"]);
        assert_eq!(range3.1, ["999", "1010"]);
        assert_eq!(range4.1, ["1188511885"]);
        assert_eq!(range5.1, ["222222"]);
        assert_eq!(range6.1, vec![] as Vec<String>);
        assert_eq!(range7.1, ["446446"]);
        assert_eq!(range8.1, ["38593859"]);
        assert_eq!(range9.1, ["565656"]);
        assert_eq!(range10.1, ["824824824"]);
        assert_eq!(range11.1, ["2121212121"]);
    }

    #[test]
    fn sum_of_test_input_part_1() {
        let input_file_path = "data/test_input.txt".to_owned();
        let input_file = parse_input(input_file_path).unwrap();

        let ranges_part1 = RangeCollection::new(input_file.clone());
        let sum = ranges_part1.get_invalid_ids(Part::Part1);
        assert_eq!(sum, 1227775554);
    }

    #[test]
    fn sum_of_test_input_part_2() {
        let input_file_path = "data/test_input.txt".to_owned();
        let input_file = parse_input(input_file_path).unwrap();

        let ranges_part1 = RangeCollection::new(input_file.clone());
        let sum = ranges_part1.get_invalid_ids(Part::Part2);
        assert_eq!(sum, 4174379265);
    }
}