use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
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

    seeds
        .iter()
        .tuples()
        .map(|(&seed, &length)| seed..(seed + length))
        // Each seed range will be broken down into smaller and smaller ranges until it represents location ranges
        .flat_map(|seed_range| {
            mappings.iter().fold(vec![seed_range], |ranges, mapping| {
                ranges.iter().fold(Vec::new(), |mut new_ranges, range| {
                    // We'll use this later when filling in the gaps
                    let mut curr_start = range.start;

                    mapping
                        .maps
                        .iter()
                        // Replace each mapping with the range that overlaps, if it exists
                        .filter_map(|map| {
                            let start = map.1.start.clamp(range.start, range.end);
                            let end = map.1.end.clamp(range.start, range.end);

                            if start != end {
                                // Save the new range slice
                                new_ranges.push(Range {
                                    start: start + map.0.start - map.1.start,
                                    end: end + map.0.end - map.1.end,
                                });
                                // Return the original range instead of the new one so that we can calculate the gaps
                                Some(start..end)
                            } else {
                                None
                            }
                        })
                        // Sort the original ranges so we can just look at the start of each range
                        .sorted_by(|a, b| a.start.partial_cmp(&b.start).unwrap())
                        // Loop over the range slices, filling in the gaps as necessary
                        .for_each(|range_slice| {
                            if curr_start < range_slice.start {
                                new_ranges.push(curr_start..range_slice.start);
                            }
                            curr_start = range_slice.end;
                        });

                    // Ensure the last gap is filled if the last range slice was smaller than the original range
                    if curr_start < range.end {
                        new_ranges.push(curr_start..range.end);
                    }

                    new_ranges
                })
            })
        })
        // Extracts the lowest value from each range, the start value
        .map(|range| range.start)
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
