use grid::Grid;
use std::collections::BTreeMap;

type Point = (isize, isize);

trait GetChecked<T> {
    fn get_checked(&self, point: Point) -> Option<&T>;
    fn get_checked_mut(&mut self, point: Point) -> Option<&mut T>;
}

impl<T> GetChecked<T> for Grid<T> {
    fn get_checked(&self, point: Point) -> Option<&T> {
        self.get(point.0, point.1)
    }

    fn get_checked_mut(&mut self, point: Point) -> Option<&mut T> {
        self.get_mut(point.0, point.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vector {
    point: Point,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn advance(&self, vector: &Vector, path: &BTreeMap<Point, Vec<Direction>>) -> Vector {
        let new_vector = match self {
            Self::North => Vector {
                point: (vector.point.0 - 1, vector.point.1),
                direction: *self,
            },
            Self::South => Vector {
                point: (vector.point.0 + 1, vector.point.1),
                direction: *self,
            },
            Self::East => Vector {
                point: (vector.point.0, vector.point.1 + 1),
                direction: *self,
            },
            Self::West => Vector {
                point: (vector.point.0, vector.point.1 - 1),
                direction: *self,
            },
        };

        if let Some(tiles) = path.get(&new_vector.point) {
            if tiles.contains(&new_vector.direction) {
                return Vector {
                    point: (-1, -1),
                    direction: *self,
                };
            }
        }

        new_vector
    }
}

pub fn process(input: &str) -> String {
    let grid = Grid::from_vec(
        input.lines().flat_map(|line| line.chars()).collect(),
        input.lines().next().map_or(0, |row| row.len()),
    );

    let mut path = BTreeMap::new();

    follow_beam(
        Vector {
            point: (0, 0),
            direction: Direction::East,
        },
        &mut path,
        &grid,
    )
    .to_string()

    // print_grid(&path, &grid);
}

pub fn follow_beam(
    vector: Vector,
    path: &mut BTreeMap<Point, Vec<Direction>>,
    grid: &Grid<char>,
) -> usize {
    use Direction::*;

    if grid.get_checked(vector.point).is_none() {
        return 0;
    }

    let mut new_tile = 1;
    if path.get(&vector.point).is_some() {
        new_tile = 0;
    }

    path.entry(vector.point)
        .and_modify(|tiles| tiles.push(vector.direction))
        .or_insert(vec![vector.direction]);

    let beam_size = match grid.get_checked(vector.point) {
        Some('.') => follow_beam(vector.direction.advance(&vector, path), path, grid),
        Some('/') => match vector.direction {
            North => follow_beam(Direction::advance(&East, &vector, path), path, grid),
            South => follow_beam(Direction::advance(&West, &vector, path), path, grid),
            East => follow_beam(Direction::advance(&North, &vector, path), path, grid),
            West => follow_beam(Direction::advance(&South, &vector, path), path, grid),
        },
        Some('\\') => match vector.direction {
            North => follow_beam(Direction::advance(&West, &vector, path), path, grid),
            South => follow_beam(Direction::advance(&East, &vector, path), path, grid),
            East => follow_beam(Direction::advance(&South, &vector, path), path, grid),
            West => follow_beam(Direction::advance(&North, &vector, path), path, grid),
        },
        Some('-') => match vector.direction {
            North | South => {
                follow_beam(Direction::advance(&East, &vector, path), path, grid)
                    + follow_beam(Direction::advance(&West, &vector, path), path, grid)
            }
            East => follow_beam(Direction::advance(&East, &vector, path), path, grid),
            West => follow_beam(Direction::advance(&West, &vector, path), path, grid),
        },
        Some('|') => match vector.direction {
            North => follow_beam(Direction::advance(&North, &vector, path), path, grid),
            South => follow_beam(Direction::advance(&South, &vector, path), path, grid),
            East | West => {
                follow_beam(Direction::advance(&North, &vector, path), path, grid)
                    + follow_beam(Direction::advance(&South, &vector, path), path, grid)
            }
        },
        _ => unreachable!(),
    };

    beam_size + new_tile
}

pub fn print_grid(path: &BTreeMap<Point, Vec<Vector>>, grid: &Grid<char>) {
    dbg!(path.keys().count());
    grid.iter_rows().enumerate().for_each(|(row, row_iter)| {
        let line = row_iter
            .enumerate()
            .map(|(col, _)| {
                path.get(&(row as isize, col as isize))
                    .map(|_| '#')
                    .unwrap_or('.')
            })
            .collect::<String>();
        dbg!(line);
    })
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
