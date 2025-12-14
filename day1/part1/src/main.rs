// AoC 2025 - Day 01 - Part 1
use std::fs::read_to_string;

const PATH_TO_INPUT: &str = "../data/input";
const DIAL_SIZE: i32 = 100;
const DIAL_START_AT: i32 = 50;

fn main() {
    let lines = read_lines(PATH_TO_INPUT);
    let hit = calc_hit(&lines, DIAL_SIZE, DIAL_START_AT);
    println!("HIT:{hit}");
}

fn calc_hit(lines: &[String], dial_size: i32, dial_start: i32) -> u32 {
    let mut curr = dial_start;
    let mut hit = 0;
    let mut line_num = 0;

    for line in lines {
        line_num += 1;

        assert!(
            line.len() >= 2,
            "Line '{line_num}' does not conform to format 'L|R[0-9]+'",
        );

        let dir = &line[0..1];
        let count = line[1..].parse::<i32>().unwrap();

        // Move 1 to the left.
        if dir == "L" {
            curr -= count;
        }
        // Move 1 to the left.
        else {
            curr += count;
        }

        // Wrap
        curr %= dial_size;
        if curr < 0 {
            curr += dial_size;
        }

        // Counting 0 hits
        if curr == 0 {
            hit += 1;
        }
    }

    hit
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = read_lines(PATH_TO_INPUT);
        let result = calc_hit(&lines, DIAL_SIZE, DIAL_START_AT);
        assert_eq!(result, 1118);
    }
}
