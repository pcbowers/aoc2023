use itertools::{izip, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    multi::separated_list1,
    sequence::preceded,
    sequence::{terminated, tuple},
    IResult,
};
use roots::{find_roots_quadratic, Roots};

#[derive(Debug, Eq, PartialEq)]
struct TimeDistance {
    pub time: u64,
    pub distance: u64,
}

impl TimeDistance {
    pub fn create_pairs(times: Vec<u64>, distances: Vec<u64>) -> Vec<Self> {
        izip!(times, distances)
            .map(|(time, distance)| Self { time, distance })
            .collect_vec()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
}

impl Direction {
    pub fn calculate_inner_bound(&self, bound: f64) -> u64 {
        if bound.fract() != 0.0f64 {
            if Direction::Up.eq(self) {
                bound.ceil() as u64
            } else {
                bound.floor() as u64
            }
        } else if Direction::Up.eq(self) {
            (bound + 1f64) as u64
        } else {
            (bound - 1f64) as u64
        }
    }
}

pub fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    tuple((
        terminated(
            preceded(
                terminated(tag("Time:"), space1),
                separated_list1(space1, nom::character::complete::u64),
            ),
            line_ending,
        ),
        preceded(
            terminated(tag("Distance:"), space1),
            separated_list1(space1, nom::character::complete::u64),
        ),
    ))(input)
}

pub fn process(input: &str) -> String {
    let (_, (times, distances)) = parse(input).unwrap();
    let time_distance_pairs = TimeDistance::create_pairs(times, distances);
    time_distance_pairs
        .iter()
        .filter_map(|time_distance| {
            let roots = find_roots_quadratic(
                -1f64,
                time_distance.time as f64,
                -(time_distance.distance as f64),
            );

            match roots {
                Roots::Two(bounds) => {
                    let first = Direction::Up.calculate_inner_bound(bounds[0]);
                    let last = Direction::Down.calculate_inner_bound(bounds[1]);
                    Some(last - first + 1u64)
                }
                _ => None,
            }
        })
        .product::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "288".to_string());
    }
}
