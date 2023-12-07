pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (_, nums) = line.split_once(':').unwrap();
            let (winning_nums, present_nums) = nums.split_once('|').unwrap();

            let winning_nums: Vec<u32> = winning_nums
                .split(' ')
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();
            let present_nums: Vec<u32> = present_nums
                .split(' ')
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();

            match winning_nums
                .iter()
                .filter(|n| present_nums.contains(n))
                .count() as u32
            {
                0 => 0,
                match_count => (2_u32).pow(match_count - 1),
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "13".to_string());
    }
}
