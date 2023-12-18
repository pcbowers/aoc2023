use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{self, anychar, space1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, (char, u64, &str)> {
    tuple((
        terminated(anychar, space1),
        terminated(complete::u64, space1),
        delimited(tag("(#"), take_until1(")"), tag(")")),
    ))(input)
}

pub fn process(input: &str) -> String {
    // parse the dig plan into memory
    let dig_plan = input
        .lines()
        .filter_map(|line| parse(line).ok())
        .map(|(_, (_, _, color))| color)
        .map(|color| {
            (
                match isize::from_str_radix(&color[5..], 16).expect("Must parse") {
                    0 => 'R',
                    1 => 'D',
                    2 => 'L',
                    3 => 'U',
                    _ => unreachable!("Must be 0-3"),
                },
                isize::from_str_radix(&color[..5], 16).expect("Must parse"),
            )
        })
        .collect_vec();

    // Create a path using the dig plan
    let mut dig_path: Vec<(isize, isize)> = Vec::from([(0, 0)]);
    for instruction in dig_plan.iter() {
        let end = *dig_path.last().expect("Must have end path");
        match instruction.0 {
            'U' => dig_path.push((end.0 - instruction.1, end.1)),
            'D' => dig_path.push((end.0 + instruction.1, end.1)),
            'L' => dig_path.push((end.0, end.1 - instruction.1)),
            'R' => dig_path.push((end.0, end.1 + instruction.1)),
            _ => unreachable!("Must have a direction"),
        }
    }

    // The last one is the same as the first, and is unnecessary
    dig_path.pop();

    // Calculate perimeter by adding all the distances
    let perimeter = dig_plan.iter().map(|(_, meters)| meters).sum::<isize>();

    // See https://en.wikipedia.org/wiki/Shoelace_formula
    // Calculates Area via Shoelace Formula
    let absolute_area = dig_path
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.1 * b.0 - b.1 * a.0)
        .sum::<isize>()
        .abs()
        / 2;

    // See https://en.wikipedia.org/wiki/Pick%27s_theorem
    // Calculates inner area via re-arranged Pick's Theorem
    let inner_area = absolute_area - perimeter / 2 + 1;

    // Calculate the area based on integer coordinates
    let block_area = inner_area + perimeter;

    block_area.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "952408144115".to_string());
    }
}
