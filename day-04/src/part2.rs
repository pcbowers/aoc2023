use std::collections::HashMap;

pub fn process(input: &str) -> String {
    let mut ids_to_copies: HashMap<u32, u32> = HashMap::new();

    input.lines().rev().for_each(|line| {
        let (card_and_id, nums) = line.split_once(':').unwrap();
        let (winning_nums, present_nums) = nums.split_once('|').unwrap();
        let card_id = card_and_id
            .split(' ')
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

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
            0 => ids_to_copies.insert(card_id, 1),
            match_count => ids_to_copies.insert(
                card_id,
                ((card_id + 1)..=(card_id + match_count))
                    .filter_map(|id| ids_to_copies.get(&id))
                    .sum::<u32>()
                    + 1,
            ),
        };
    });

    ids_to_copies.values().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "30".to_string());
    }
}
