// AoC 2025 - Day 03 - Part 1

const PATH_TO_INPUT: &str = "../data/input";

fn main() {
    println!("Hello day 3!");

    let lines: Vec<String> = std::fs::read_to_string(PATH_TO_INPUT)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let joltage = calc_joltage(&lines);

    println!("Joltage:{joltage}"); // 17109
}

fn calc_joltage(lines: &[String]) -> u32 {
    let mut joltage:u32 = 0;

    for line in lines {
        // Parse characters as integers
        let digits = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        // Find the largest number, excluding the last digit (since we need a 2-digit number)
        let d0 = digits.iter().take(digits.len() - 1).max().unwrap();
        // Find the position of the first number
        let d0_pos = digits.iter().position(|d| d == d0).unwrap();

        // Find the largest number _after_ the previous one (which includes the final digit)
        let d1 = digits.iter().skip(d0_pos + 1).max().unwrap();

        // Find the position of the second number
        let d1_pos = digits
            .iter()
            .skip(d0_pos + 1)
            .position(|d| d == d1)
            .unwrap();

        // Concat the numbers as string, and parse back to int
        let res = format!("{d0}{d1}").parse::<u32>().unwrap();
        joltage += res;

        println!("{line} {res}");
        println!("{}^{}^", " ".repeat(d0_pos), " ".repeat(d1_pos));
    }

    joltage
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_joltage_test() {
        let lines: Vec<String> = std::fs::read_to_string(PATH_TO_INPUT)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        let joltage = calc_joltage(&lines);

        assert_eq!(17109, joltage);
    }
}
