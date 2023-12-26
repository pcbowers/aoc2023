use itertools::Itertools;
use std::{f64::EPSILON, ops::Range};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn is_after(&self, other: &Hailstone) -> bool {
        (self.x - other.position.x) * other.velocity.x > 0.0
            && (self.y - other.position.y) * other.velocity.y > 0.0
    }

    fn is_in_bounds(&self, bounds: &Range<f64>) -> bool {
        self.x >= bounds.start
            && self.x < bounds.end
            && self.y >= bounds.start
            && self.y < bounds.end
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from((x, y, _): (f64, f64, f64)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    position: Point,
    velocity: Point,
}

impl Hailstone {
    /*
    Let's talk math for a bit.
    Here's the general formula for an intersection based on initial position and constant velocities:

    (x1, y1) + t * (vx1, vy1) = (x2, y2) + t * (vx2, vy2)

    This can be expanded like so:

    (x1 + t * vx1, y1 + t * vy1) = (x2 + t * vx2, y2 + t * vy2)

    We can separate this into two equations, one for x and one for y:

    x1 + t * vx1 = x2 + t * vx2
    y1 + t * vy1 = y2 + t * vy2

    Using either of these, we can solve for t:

    t = (x2 - x1) / (vx1 - vx2)
    t = (y2 - y1) / (vy1 - vy2)

    Perfect! Once you get t just solve for x and y and you get your intersection!

    One problem though: what about parallel lines? A line is parallel when the following is true:

    vx1 - vx2 = 0
    vy1 - vy2 = 0

    Since this is possible, we have a problem: you can't divide by 0.
    This means we have to check that both aren't 0.
    If one is 0 and the other isn't, we have to use the non-0 component to solve for t.

    This is rather annoying: we end up duplicating logic for each component to manage this.
    Thankfully, the cross product can be used to help us here. For 2D space, this holds true:

    given two vectors (a,b) and (c,d)
    cross_product = a * d - b * c
    cross_product = vx1 * vy2 - vy1 * vx2

    If both components are parallel, the cross product is 0, now just one check!
    Here's the cool part though, we can also calculate t using the cross product.
    Let's start with some of the earlier formulas:

    t = (x2 - x1) / (vx1 - vx2)
    t = (y2 - y1) / (vy1 - vy2)

    Let's simplify:

    t = x_diff / (vx1 - vx2)
    t = y_diff / (vy1 - vy2)

    And we'll modify them slightly:

    t = x_diff * vy2 / ((vx1 - vx2) * vy2)
    t = y_diff * vx2 / ((vy1 - vy2) * vx2)

    We can combine like so:

    t = (x_diff * vy2 - y_diff * vx2) / (((vx1 - vx2) * vy2) - ((vy1 - vy2) * vx2))

    Distribute the denominator like so:

    t = (x_diff * vy2 - y_diff * vx2) / (vx1 * vy2 - vx2 * vy2 - vy1 * vx2 + vy2 * vx2)
    t = (x_diff * vy2 - y_diff * vx2) / (vx1 * vy2 - vy1 * vx2)
    t = (x_diff * vy2 - y_diff * vx2) / cross_product

    And there we have it! We've got the cross product in the denominator.
    This means we don't need to duplicate logic anymore
    */
    fn find_intersection(&self, other: &Self) -> Option<Point> {
        let x_diff = other.position.x - self.position.x;
        let y_diff = other.position.y - self.position.y;
        let cross_product = self.velocity.x * other.velocity.y - self.velocity.y * other.velocity.x;

        // Take into account floating point precision instead of checking for 0
        if cross_product.abs() < EPSILON {
            None
        } else {
            let t = (x_diff * other.velocity.y - y_diff * other.velocity.x) / cross_product;
            let x = self.position.x + (self.velocity.x * t);
            let y = self.position.y + (self.velocity.y * t);
            Some(Point { x, y })
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
