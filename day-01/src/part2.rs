use std::collections::HashMap;

pub fn process(input: &str) -> String {
    let str_to_num = HashMap::from([
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ]);

    input
        .lines()
        .map(|line| {
            let first_key = (0..line.len())
                .find_map(|i| str_to_num.keys().find(|key| line[i..].starts_with(*key)))
                .unwrap();

            let last_key = (0..line.len())
                .rev()
                .find_map(|i| str_to_num.keys().find(|key| line[i..].starts_with(*key)))
                .unwrap();

            str_to_num[first_key] * 10 + str_to_num[last_key]
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example2.txt"));
        assert_eq!(result, "281".to_string());
    }
}
