// AoC 2025 - Day 02 - Part 1

const PATH_TO_INPUT: &str = "../data/input";

fn main() {
    let line = std::fs::read_to_string(PATH_TO_INPUT).unwrap();
    let hit_sum = calc_fakes(&line);

    println!("Hit Sum:{hit_sum}");
}

fn calc_fakes(line: &str) -> u64 {
    println!("Day 2 - Part 1 {line}");

    // Split line into ranges ("1-2,3-4,5-6" => ["1-2","3-4","5-6"])
    let ranges = line.split(',');

    let mut hit_sum = 0;

    // Loop through all ranges
    for range in ranges {
        // Split range into "from" and "to" (strings)
        let range_split: Vec<&str> = range.split('-').collect();
        assert!(
            range_split.len() == 2,
            "Range '{range}' does not conform to expected format '[0-9]+-[0-9]+'"
        );

        let from_s = range_split[0].trim();
        let to_s = range_split[1].trim();

        // Parse range to integers
        let from = from_s.parse::<u64>().unwrap();
        let to = to_s.parse::<u64>().unwrap();
        assert!(from < to, "Range goes backwards ('{range}')");

        // Loop through range values (from -> to)
        for i in from..=to {
            let i_str = i.to_string();

            // Only include numbers with an even digit count
            // I.e.:
            // - 22 => Yes
            // - 222 => No
            // - 2222 => Yes
            if i_str.len() % 2 != 0 {
                continue;
            }

            // Compare the first half and the last half, and increment the sum if they're equal
            // I.e.:
            // 22 => Yes
            // 12 => Yes
            // 2244 => No
            // 2222 => Yes
            let strlen = i_str.len() / 2; // Since we already check for even digit count, this will always return a whole number
            let half_1 = &i_str[0..strlen];
            let half_2 = &i_str[strlen..];
            if half_1 == half_2 {
                hit_sum += i;
            }
        }
    }

    hit_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_hit_test() {
        let line = std::fs::read_to_string(PATH_TO_INPUT).unwrap();
        let hit_count = calc_fakes(&line);
        assert_eq!(24_747_430_309, hit_count);
    }
}
