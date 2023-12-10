use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, separated_list1(tag(" "), complete::i32))(input)
}

pub fn process(input: &str) -> String {
    let (_, histories) = parse(input).unwrap();

    histories
        .iter()
        .map(|history| {
            let mut current_history = history.clone();
            let mut first_values = vec![*history.first().unwrap()];
            for _ in 0..history.len() {
                if current_history.iter().all(|&value| value == 0) {
                    break;
                }

                current_history = current_history
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect_vec();

                first_values.push(*current_history.first().unwrap());
            }

            first_values.iter().rev().fold(0, |acc, value| value - acc)
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "2".to_string());
    }
}
