use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::alphanumeric1,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
struct Point<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

pub fn parse_point(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((
        terminated(alphanumeric1, tag(" = (")),
        terminated(alphanumeric1, tag(", ")),
        terminated(alphanumeric1, tag(")")),
    ))(input)
}

pub fn process(input: &str) -> String {
    let mut lines = input.lines();
    let mut instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|ch| match ch {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .cycle();

    let mut points: HashMap<&str, Point> = HashMap::new();

    lines
        .filter_map(|line| parse_point(line).ok())
        .for_each(|(_, names)| {
            points.insert(
                names.0,
                Point {
                    name: names.0,
                    left: names.1,
                    right: names.2,
                },
            );
        });

    let mut steps: u64 = 0;
    let mut curr_step = "AAA";

    loop {
        if let Some(point) = points.get(curr_step) {
            if point.name == "ZZZ" {
                break;
            }

            steps += 1;
            match instructions.next() {
                Some(Direction::Left) => curr_step = point.left,
                Some(Direction::Right) => curr_step = point.right,
                None => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }

    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example1.txt"));
        assert_eq!(result, "6".to_string());
    }
}
