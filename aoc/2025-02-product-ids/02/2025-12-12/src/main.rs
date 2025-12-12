use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });

    let product_ranges = split_into_ranges(&content);
    let mut total: u64 = 0;

    for (start, end) in &product_ranges {
        println!("Range: {}-{}", start, end);
        for id in *start..=*end {
            if has_pattern(id) {
                // println!("Product ID {} matches pattern", id);
                total += id;
            }
        }
    }

    println!("Total sum of matching product IDs: {}", total);
}

fn split_into_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|product_range| {
            let parts: Vec<&str> = product_range.split('-').collect();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();
            (start, end)
        })
        .collect()
}

fn has_pattern(value: u64) -> bool {
    let s = value.to_string();
    if s.len() < 2 {
        return false;
    }

    if s.split_at(s.len() / 2)
        .1
        .chars()
        .zip(s.chars())
        .all(|(a, b)| a == b)
    {
        return true;
    }
    for i in 3..s.len() {
        if s.split_at(s.len() / i)
            .1
            .chars()
            .zip(s.chars())
            .all(|(a, b)| a == b)
        {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_ranges() {
        let input = "1-3,4-6,7-10";
        let expected = vec![(1, 3), (4, 6), (7, 10)];
        assert_eq!(split_into_ranges(input), expected);
    }

    #[test]
    fn test_has_pattern_single_digits() {
        assert_eq!(has_pattern(0), false);
        assert_eq!(has_pattern(1), false);
        assert_eq!(has_pattern(9), false);
    }

    #[test]
    fn test_has_pattern_two_digits() {
        assert_eq!(has_pattern(11), true);
        assert_eq!(has_pattern(12), false);
        assert_eq!(has_pattern(22), true);
        assert_eq!(has_pattern(23), false);
        assert_eq!(has_pattern(98), false);
        assert_eq!(has_pattern(99), true);
    }

    #[test]
    fn test_has_pattern_odd_length() {
        assert_eq!(has_pattern(111), true);
        assert_eq!(has_pattern(121), false);
        assert_eq!(has_pattern(123), false);

        assert_eq!(has_pattern(12345), false);
        assert_eq!(has_pattern(11234), false);
        assert_eq!(has_pattern(12321), false);

        assert_eq!(has_pattern(1111117), false);
        assert_eq!(has_pattern(1111111), true);

        assert_eq!(has_pattern(123123123), true);
        assert_eq!(has_pattern(123123124), false);
    }

    #[test]
    fn test_has_pattern_even_length() {
        assert_eq!(has_pattern(123456), false);
        assert_eq!(has_pattern(121121), true);
        assert_eq!(has_pattern(121212), true);
        assert_eq!(has_pattern(123123), true);
        assert_eq!(has_pattern(123124), false);

        assert_eq!(has_pattern(1010), true);
        assert_eq!(has_pattern(1011), false);

        assert_eq!(has_pattern(1188511885), true);
        assert_eq!(has_pattern(1188511882), false);
    }
}
