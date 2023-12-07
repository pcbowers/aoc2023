use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use rayon::prelude::*;
use std::ops::Range;

type RangeMappings<'a> = IResult<&'a str, Vec<(Range<u64>, Range<u64>)>>;

#[derive(Debug, Eq, PartialEq)]
pub struct SeedMapping {
    pub from: String,
    pub to: String,
    pub maps: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMapping {
    fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
        terminated(
            preceded(tag("seeds: "), separated_list1(space1, complete::u64)),
            line_ending,
        )(input)
    }

    fn parse_range_name(input: &str) -> IResult<&str, (&str, &str)> {
        preceded(
            line_ending,
            terminated(
                separated_pair(alphanumeric1, tag("-to-"), alphanumeric1),
                tag(" map:"),
            ),
        )(input)
    }

    fn parse_range_mapping(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
        terminated(
            map(separated_list1(space1, complete::u64), |nums| {
                (nums[0]..(nums[0] + nums[2]), nums[1]..(nums[1] + nums[2]))
            }),
            line_ending,
        )(input)
    }

    fn parse_range_mappings(input: &str) -> RangeMappings {
        preceded(line_ending, many1(Self::parse_range_mapping))(input)
    }

    pub fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<Self>)> {
        map(
            tuple((
                Self::parse_seeds,
                many1(tuple((Self::parse_range_name, Self::parse_range_mappings))),
            )),
            |(seeds, mappings)| {
                (
                    seeds,
                    mappings
                        .iter()
                        .map(|mapping| Self {
                            from: String::from(mapping.0 .0),
                            to: String::from(mapping.0 .1),
                            maps: mapping.1.to_vec(),
                        })
                        .collect(),
                )
            },
        )(input)
    }
}

pub fn process(input: &str) -> String {
    let (_, (seeds, mappings)) = SeedMapping::parse(input).unwrap();
    let seed_groups = seeds
        .iter()
        .tuples()
        .flat_map(|(&seed, &length)| seed..(seed + length))
        .collect_vec();

    seed_groups
        .par_iter()
        .map(|&seed| {
            mappings.iter().fold(seed, |cur_seed, mapping| {
                for map in mapping.maps.iter() {
                    if cur_seed >= map.1.start && cur_seed < map.1.end {
                        return (cur_seed - map.1.start) + map.0.start;
                    }
                }

                cur_seed
            })
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "46".to_string());
    }
}
