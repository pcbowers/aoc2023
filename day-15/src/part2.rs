use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::digit1,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::BTreeMap;

pub fn parse(input: &str) -> IResult<&str, (&str, Option<u64>)> {
    tuple((
        alt((take_until1("="), take_until1("-"))),
        alt((
            preceded(
                tag("="),
                map(digit1, |digit: &str| {
                    Some(digit.parse::<u64>().expect("Must be a number"))
                }),
            ),
            preceded(tag("-"), map(tag(""), |_| None as Option<u64>)),
        )),
    ))(input)
}

pub fn process(input: &str) -> String {
    let mut hash_map: BTreeMap<u64, IndexMap<&str, u64>> = BTreeMap::new();

    input.split(',').for_each(|item| {
        let (_, (label, possible_focal_length)) = parse(item).expect("Must match the parser");
        let hash = label
            .chars()
            .fold(0, |acc, c| ((acc + c as u64) * 17) % 256);

        match possible_focal_length {
            Some(focal_length) => hash_map
                .entry(hash)
                .and_modify(|values| {
                    values.insert(label, focal_length);
                })
                .or_insert(IndexMap::from([(label, focal_length)])),
            None => hash_map
                .entry(hash)
                .and_modify(|values| {
                    values.shift_remove(label);
                })
                .or_default(),
        };
    });

    hash_map
        .iter()
        .map(|(box_num, the_box)| {
            the_box
                .iter()
                .enumerate()
                .map(move |(slot_num, (_, focal_length))| {
                    (box_num + 1) * (slot_num as u64 + 1) * focal_length
                })
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "145".to_string());
    }
}
