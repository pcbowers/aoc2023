use std::ops::Range;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Point(f64, f64, f64);

impl Point {
    fn is_after(&self, other: &Hailstone) -> bool {
        (self.0 - other.position.0) * other.velocity.0 > 0.0
            && (self.1 - other.position.1) * other.velocity.1 > 0.0
    }

    fn is_in_bounds(&self, bounds: &Range<f64>) -> bool {
        self.0 >= bounds.start
            && self.0 < bounds.end
            && self.1 >= bounds.start
            && self.1 < bounds.end
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self(x, y, z)
    }
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    position: Point,
    velocity: Point,
}

impl Hailstone {
    fn find_intersection(&self, other: &Self) -> Option<Point> {
        let x_diff = other.position.0 - self.position.0;
        let y_diff = other.position.1 - self.position.1;
        let cross_product = self.velocity.0 * other.velocity.1 - self.velocity.1 * other.velocity.0;

        if cross_product == 0.0 {
            None
        } else {
            let t1 = (x_diff * other.velocity.1 - y_diff * other.velocity.0) / cross_product;
            let intersection_x = self.position.0 + (self.velocity.0 * t1);
            let intersection_y = self.position.1 + (self.velocity.1 * t1);
            Some(Point(intersection_x, intersection_y, 0.0))
        }
    }

    fn find_future_intersection_in_bounds(
        &self,
        other: &Self,
        bounds: &Range<f64>,
    ) -> Option<Point> {
        let Some(intersection) = self.find_intersection(other) else {
            return None;
        };

        if intersection.is_in_bounds(bounds)
            && intersection.is_after(self)
            && intersection.is_after(other)
        {
            Some(intersection)
        } else {
            None
        }
    }
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

pub fn process(input: &str, low_bound: isize, high_bound: isize) -> String {
    let (_, hailstones) = parser::parse(input).expect("should parse");
    let bounds = (low_bound as f64)..(high_bound as f64 + 1.0);

    hailstones
        .iter()
        .tuple_combinations()
        .filter_map(|(hailstone1, hailstone2)| {
            hailstone1.find_future_intersection_in_bounds(hailstone2, &bounds)
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"), 7, 27);
        assert_eq!(result, "2".to_string());
    }
}
