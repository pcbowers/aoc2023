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
            let mut next_value = 0;
            let mut current_history = history.clone();
            for _ in 0..history.len() {
                if current_history.iter().all(|&value| value == 0) {
                    break;
                }

                next_value += current_history.last().unwrap();

                current_history = current_history
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect_vec();
            }

            next_value
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
        assert_eq!(result, "114".to_string());
    }
}
