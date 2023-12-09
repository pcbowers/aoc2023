use itertools::Itertools;
use nom::{
    character::complete::{self, alphanumeric1, line_ending, space1},
    combinator::map,
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};
use std::collections::HashMap;

pub fn parse(input: &str) -> IResult<&str, Vec<(Vec<u32>, u32)>> {
    let card_to_num: std::collections::HashMap<char, u32> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 1),
        ('Q', 12),
        ('K', 14),
        ('A', 15),
    ]);

    let mut parser = many1(tuple((
        terminated(
            map(alphanumeric1, |cards: &str| {
                cards
                    .chars()
                    .map(|char| *card_to_num.get(&char).unwrap())
                    .collect_vec()
            }),
            space1,
        ),
        terminated(complete::u32, line_ending),
    )));

    parser(input)
}

pub fn process(input: &str) -> String {
    let (_, hands) = parse(input).unwrap();
    hands
        .iter()
        .map(|hand| {
            let mut counts = hand.0.iter().counts();
            let jack_count = counts.remove(&1).unwrap_or(0);
            let mut hand_type = counts.values().sorted().collect_vec();

            let new_highest_count: usize;
            if let Some(highest_count) = hand_type.last_mut() {
                new_highest_count = *highest_count + jack_count;
                *highest_count = &new_highest_count;
            }

            match hand_type[..] {
                [] => (6, hand),
                [5] => (6, hand),
                [1, 4] => (5, hand),
                [2, 3] => (4, hand),
                [1, 1, 3] => (3, hand),
                [1, 2, 2] => (2, hand),
                [1, 1, 1, 2] => (1, hand),
                [1, 1, 1, 1, 1] => (0, hand),
                _ => unreachable!(),
            }
        })
        .sorted_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(b.1)))
        .enumerate()
        .map(|(index, hand)| hand.1 .1 * (index as u32 + 1))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "5905".to_string());
    }
}
