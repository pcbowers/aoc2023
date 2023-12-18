use grid::Grid;
use itertools::Itertools;
use pathfinding::prelude::astar;

type Point = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    point: Point,
    direction: Direction,
    straight_for: usize,
}

impl Vector {
    pub fn derive(&self, point: Point, direction: Direction, straight_for: usize) -> Self {
        Self {
            point: (self.point.0 + point.0, self.point.1 + point.1),
            direction,
            straight_for,
        }
    }

    pub fn successors(grid: &Grid<usize>, _: Point) -> impl Fn(&Self) -> Vec<(Self, usize)> + '_ {
        use Direction::*;

        move |vector| {
            let mut possible_vectors: Vec<(Self, usize)> = Vec::new();

            if vector.straight_for < 10 {
                let straight_for = vector.straight_for + 1;
                let straight_vector = match vector.direction {
                    North => vector.derive((-1, 0), North, straight_for),
                    South => vector.derive((1, 0), South, straight_for),
                    East => vector.derive((0, 1), East, straight_for),
                    West => vector.derive((0, -1), West, straight_for),
                };

                if let Some(weight) = grid.get(
                    straight_vector.point.0 as usize,
                    straight_vector.point.1 as usize,
                ) {
                    possible_vectors.push((straight_vector, *weight))
                }
            }

            if vector.straight_for >= 4 {
                let right_vector = match vector.direction {
                    North => vector.derive((0, 1), East, 1),
                    South => vector.derive((0, -1), West, 1),
                    East => vector.derive((1, 0), South, 1),
                    West => vector.derive((-1, 0), North, 1),
                };

                if let Some(weight) =
                    grid.get(right_vector.point.0 as usize, right_vector.point.1 as usize)
                {
                    possible_vectors.push((right_vector, *weight))
                }

                let left_vector = match vector.direction {
                    North => vector.derive((0, -1), West, 1),
                    South => vector.derive((0, 1), East, 1),
                    East => vector.derive((-1, 0), North, 1),
                    West => vector.derive((1, 0), South, 1),
                };

                if let Some(weight) =
                    grid.get(left_vector.point.0 as usize, left_vector.point.1 as usize)
                {
                    possible_vectors.push((left_vector, *weight))
                }
            }

            possible_vectors
        }
    }

    pub fn heuristic(_: &Grid<usize>, end_point: Point) -> impl Fn(&Self) -> usize {
        move |vector| vector.point.0.abs_diff(end_point.0) + vector.point.1.abs_diff(end_point.1)
    }

    pub fn success(_: &Grid<usize>, end_point: Point) -> impl Fn(&Self) -> bool {
        move |vector| vector.point == end_point && vector.straight_for >= 4
    }
}

pub fn process(input: &str) -> String {
    let grid = Grid::from_vec(
        input
            .lines()
            .flat_map(|line| {
                line.chars()
                    .filter_map(|heat_loss| heat_loss.to_digit(10))
                    .map(|heat_loss| heat_loss as usize)
            })
            .collect(),
        input.lines().next().map_or(0, |row| row.len()),
    );

    let end_point = (grid.rows() as isize - 1, grid.cols() as isize - 1);

    // See https://www.redblobgames.com/pathfinding/a-star/introduction.html
    // This is how I discovered and understood using the astar algorithm
    if let Some((_, min_heat)) = astar(
        &Vector {
            point: (0, 0),
            direction: Direction::East,
            straight_for: 0,
        },
        Vector::successors(&grid, end_point),
        Vector::heuristic(&grid, end_point),
        Vector::success(&grid, end_point),
    ) {
        // print_grid(&grid, &path);
        min_heat.to_string()
    } else {
        "".to_string()
    }
}

pub fn print_grid(grid: &Grid<usize>, path: &[Vector]) {
    use Direction::*;
    let mut new_grid = Grid::from_vec(
        grid.clone()
            .into_vec()
            .iter()
            .map(|item| item.to_string())
            .collect_vec(),
        grid.cols(),
    );

    path.iter().for_each(|vector| {
        if let Some(block) = new_grid.get_mut(vector.point.0 as usize, vector.point.1 as usize) {
            *block = match (vector.point, vector.direction) {
                ((0, 0), _) => block.to_string(),
                (_, North) => "^".to_string(),
                (_, South) => "v".to_string(),
                (_, East) => ">".to_string(),
                (_, West) => "<".to_string(),
            }
        }
    });

    dbg!(new_grid
        .iter_rows()
        .map(|mut row| row.join(""))
        .collect_vec());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "94".to_string());
    }

    #[test]
    fn test_process2() {
        let result = process(include_str!("../data/example2.txt"));
        assert_eq!(result, "71".to_string());
    }
}
