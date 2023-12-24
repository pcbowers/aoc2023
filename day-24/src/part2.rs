#[derive(Debug, Clone, Copy)]
struct Point(f64, f64, f64);

impl From<(f64, f64, f64)> for Point {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self(x, y, z)
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct Hailstone {
    position: Point,
    velocity: Point,
}

impl From<(Point, Point)> for Hailstone {
    fn from((position, velocity): (Point, Point)) -> Self {
        Self { position, velocity }
    }
}

mod parser {
    use super::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{self, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::{separated_pair, terminated, tuple},
        IResult,
    };

    fn number(input: &str) -> IResult<&str, f64> {
        map(complete::i64, |number| number as f64)(input)
    }

    fn point(input: &str) -> IResult<&str, Point> {
        map(
            tuple((
                terminated(number, tag(", ")),
                terminated(number, tag(", ")),
                number,
            )),
            Point::from,
        )(input)
    }

    fn hailstone(input: &str) -> IResult<&str, Hailstone> {
        map(separated_pair(point, tag(" @ "), point), Hailstone::from)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Hailstone>> {
        separated_list1(line_ending, hailstone)(input)
    }
}

pub fn process(input: &str) -> String {
    let (_, hailstones) = parser::parse(input).expect("should parse");

    dbg!(hailstones);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "47".to_string());
    }
}
