use glam::DVec3;
use z3::{
    ast::{Ast, Int},
    Config, Context, SatResult, Solver,
};

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    position: DVec3,
    velocity: DVec3,
}

impl From<(DVec3, DVec3)> for Hailstone {
    fn from((position, velocity): (DVec3, DVec3)) -> Self {
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

    fn point(input: &str) -> IResult<&str, DVec3> {
        map(
            tuple((
                terminated(number, tag(", ")),
                terminated(number, tag(", ")),
                number,
            )),
            DVec3::from,
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

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let initial_rock_x = Int::new_const(&ctx, "x");
    let initial_rock_y = Int::new_const(&ctx, "y");
    let initial_rock_z = Int::new_const(&ctx, "z");
    let velocity_rock_x = Int::new_const(&ctx, "vx");
    let velocity_rock_y = Int::new_const(&ctx, "vy");
    let velocity_rock_z = Int::new_const(&ctx, "vz");

    for (index, hailstone) in hailstones.into_iter().enumerate() {
        let t = Int::new_const(&ctx, format!("t{index}"));

        let hailstone_x = hailstone.position.x as i64 + &t * hailstone.velocity.x as i64;
        let hailstone_y = hailstone.position.y as i64 + &t * hailstone.velocity.y as i64;
        let hailstone_z = hailstone.position.z as i64 + &t * hailstone.velocity.z as i64;
        let rock_x = &initial_rock_x + &t * &velocity_rock_x;
        let rock_y = &initial_rock_y + &t * &velocity_rock_y;
        let rock_z = &initial_rock_z + &t * &velocity_rock_z;

        solver.assert(&t.ge(&Int::from_i64(&ctx, 0)));
        solver.assert(&hailstone_x._eq(&rock_x));
        solver.assert(&hailstone_y._eq(&rock_y));
        solver.assert(&hailstone_z._eq(&rock_z));
    }

    let SatResult::Sat = solver.check() else {
        unreachable!("Unsolvable!");
    };

    let model = solver.get_model().unwrap();

    let rock = Hailstone {
        position: DVec3::new(
            model
                .get_const_interp(&initial_rock_x)
                .and_then(|ast| ast.as_i64().map(|n| n as f64))
                .unwrap(),
            model
                .get_const_interp(&initial_rock_y)
                .and_then(|ast| ast.as_i64().map(|n| n as f64))
                .unwrap(),
            model
                .get_const_interp(&initial_rock_z)
                .and_then(|ast| ast.as_i64().map(|n| n as f64))
                .unwrap(),
        ),
        velocity: DVec3::new(
            model
                .get_const_interp(&velocity_rock_x)
                .and_then(|ast| ast.as_i64().map(|n| n as f64))
                .unwrap(),
            model
                .get_const_interp(&velocity_rock_y)
                .and_then(|ast| ast.as_i64().map(|n| n as f64))
                .unwrap(),
            model
                .get_const_interp(&velocity_rock_z)
                .and_then(|ast| ast.as_i64().map(|n| n as f64))
                .unwrap(),
        ),
    };

    // dbg!(&rock);

    (rock
        .position
        .to_array()
        .into_iter()
        .map(|n| n as isize)
        .sum::<isize>())
    .to_string()
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
