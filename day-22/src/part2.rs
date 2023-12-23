use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

type ID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn distance_to(&self, other: &Self) -> (isize, isize, isize) {
        (
            (self.x - other.x).abs(),
            (self.y - other.y).abs(),
            (self.z - other.z).abs(),
        )
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z
            .cmp(&other.z)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(isize, isize, isize)> for Point {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    id: ID,
    start: Point,
    end: Point,
    supported_by: Vec<ID>,
    supports: Vec<ID>,
}

impl Brick {
    fn lower(&mut self) {
        self.start.z -= 1;
        self.end.z -= 1;
    }

    fn is_supported_by(&mut self, other: &mut Self) -> bool {
        let z_match = self.start.z == (other.end.z + 1);
        let x_match = self.start.x <= other.end.x && self.end.x >= other.start.x;
        let y_match = self.start.y <= other.end.y && self.end.y >= other.start.y;

        if z_match && x_match && y_match {
            other.supports.push(self.id);
            self.supported_by.push(other.id);
            true
        } else {
            false
        }
    }

    fn count_falls(&self, bricks: &HashMap<ID, Self>) -> usize {
        let mut stack = VecDeque::from([self]);
        let mut falling: HashSet<ID> = HashSet::from([self.id]);
        while let Some(curr_brick) = stack.pop_front() {
            for id in curr_brick.supports.iter() {
                if let Some(supported_brick) = bricks.get(id) {
                    if supported_brick
                        .supported_by
                        .iter()
                        .all(|b| falling.contains(b))
                    {
                        stack.push_back(supported_brick);
                        falling.insert(*id);
                    }
                }
            }
        }

        falling.len() - 1
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .cmp(&other.start)
            .then_with(|| self.end.cmp(&other.end))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<((Point, Point), usize)> for Brick {
    fn from(((start, end), id): ((Point, Point), usize)) -> Self {
        assert!(
            matches!(start.distance_to(&end), (0, 0, _) | (0, _, 0) | (_, 0, 0)),
            "A brick must have at least two dimensions that are 1x1"
        );

        Self {
            id,
            start: start.min(end),
            end: end.max(start),
            supported_by: Vec::new(),
            supports: Vec::new(),
        }
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

    fn digit(input: &str) -> IResult<&str, isize> {
        map(complete::i64, |num| num as isize)(input)
    }

    fn point(input: &str) -> IResult<&str, Point> {
        map(
            tuple((
                terminated(digit, tag(",")),
                terminated(digit, tag(",")),
                digit,
            )),
            Point::from,
        )(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Brick>> {
        let mut current_id = 0_usize;
        let mut parser = separated_list1(
            line_ending,
            map(separated_pair(point, tag("~"), point), |points| {
                current_id += 1;
                Brick::from((points, current_id))
            }),
        );

        parser(input)
    }
}

pub fn process(input: &str) -> String {
    let (_, mut bricks) = parser::parse(input).expect("should parse");
    bricks.sort();

    let mut stable_bricks: HashMap<ID, Brick> = HashMap::new();
    for mut brick in bricks.into_iter() {
        loop {
            let mut can_be_lowered = true;

            if brick.start.z == 1 {
                break;
            }

            for (_, stable_brick) in stable_bricks.iter_mut() {
                if brick.is_supported_by(stable_brick) {
                    can_be_lowered = false;
                }
            }

            if can_be_lowered {
                brick.lower();
            } else {
                break;
            }
        }

        stable_bricks.insert(brick.id, brick);
    }

    // dbg!(&stable_bricks);

    stable_bricks
        .values()
        .map(|brick| brick.count_falls(&stable_bricks))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "7".to_string());
    }
}
