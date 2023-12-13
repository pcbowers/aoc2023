use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn parse(input: &str) -> IResult<&str, (&str, Vec<u64>)> {
    separated_pair(
        take_until(" "),
        space1,
        separated_list1(tag(","), complete::u64),
    )(input)
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| parse(line).ok())
        .map(|(_, (springs, damaged_groups))| {
            let new_springs = [springs, springs, springs, springs, springs].join("?");
            dbg!(&new_springs);
            (new_springs, damaged_groups.repeat(5))
        })
        .map(|(springs, damaged_groups)| {
            itertools::repeat_n(['#', '.'], springs.chars().filter(|&c| c == '?').count())
                .multi_cartesian_product()
                .par_bridge()
                .map(|product| {
                    let mut permutation = product.into_iter();
                    springs
                        .chars()
                        .map(|c| match c {
                            '?' => permutation.next().unwrap(),
                            value => value,
                        })
                        .join("")
                })
                .filter(|permutation| {
                    permutation
                        .chars()
                        .group_by(|c| c == &'#')
                        .into_iter()
                        .filter_map(|(is_damaged, section)| {
                            is_damaged.then_some(section.count() as u64)
                        })
                        .collect_vec()
                        == *damaged_groups
                })
                .count()
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
        assert_eq!(result, "525152".to_string());
    }
}
