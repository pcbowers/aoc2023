pub fn process(input: &str) -> String {
    input
        .split(',')
        .map(|item| {
            item.chars()
                .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "1320".to_string());
    }
}
