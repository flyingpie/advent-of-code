// AoC 2025 - Day 02 - Part 2

const PATH_TO_INPUT: &str = "../data/input";

fn main() {
    let line = std::fs::read_to_string(PATH_TO_INPUT).unwrap();
    let hit_sum = calc_fakes(&line);

    println!("Hit Sum:{hit_sum}");
}

fn calc_fakes(line: &str) -> u64 {
    let re = fancy_regex::Regex::new("^([0-9]+)\\1+$").unwrap();

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

            // Check whether the number consists of a repeating group
            // "22" => Yes
            // "24" => No
            // "2222" => Yes
            // "2244" => No
            // "232323" => Yes
            let is_match = re.is_match(&i_str).unwrap();
            if is_match {
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
        assert_eq!(30_962_646_823, hit_count);
    }
}
