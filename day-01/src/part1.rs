pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let first_digit = line.chars().find_map(|ch| ch.to_digit(10)).unwrap();
            let last_digit = line.chars().rev().find_map(|ch| ch.to_digit(10)).unwrap();
            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example1.txt"));
        assert_eq!(result, "142".to_string());
    }
}
