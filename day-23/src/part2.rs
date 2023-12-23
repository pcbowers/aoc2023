use std::collections::{HashSet, VecDeque};

use grid::Grid;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(isize, isize);

impl Point {
    fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }

    fn is_unvisited(&self, visited: &HashSet<Self>) -> bool {
        !visited.contains(self)
    }

    fn is_valid_path(&self, grid: &Grid<char>) -> bool {
        let Some(point) = grid.get_point(self) else {
            return false;
        };

        match point {
            '.' | '^' | 'v' | '<' | '>' => true,
            '#' => false,
            _ => unreachable!("should be one of these characters"),
        }
    }

    fn neighbors(
        &self,
        path_length: &usize,
        visited: &HashSet<Self>,
        grid: &Grid<char>,
    ) -> Vec<(Self, usize)> {
        [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)]
            .iter()
            .map(|directional_vector| self.add(directional_vector))
            .filter(|neighbor| neighbor.is_unvisited(visited) && neighbor.is_valid_path(grid))
            .map(|neighbor| (neighbor, path_length + 1))
            .collect()
    }
}

impl From<(usize, usize)> for Point {
    fn from((row, col): (usize, usize)) -> Self {
        let row = row as isize;
        let col = col as isize;
        Self(row, col)
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<char>) {
    grid.iter_rows().for_each(|row| {
        let row = row.collect::<String>();
        dbg!(row);
    });
}

trait GetPoint<T> {
    fn get_point(&self, point: &Point) -> Option<&T>;
    fn get_mut_point(&mut self, point: &Point) -> Option<&mut T>;
}

impl<T> GetPoint<T> for Grid<T> {
    fn get_point(&self, point: &Point) -> Option<&T> {
        self.get(point.0, point.1)
    }

    fn get_mut_point(&mut self, point: &Point) -> Option<&mut T> {
        self.get_mut(point.0, point.1)
    }
}

pub fn process(input: &str) -> String {
    let grid = Grid::from(input.lines().map(|l| l.chars().collect()).collect_vec());

    // print_grid(&grid);

    let start = Point::from((0, 1));
    let end = Point::from((grid.rows() - 1, grid.cols() - 2));
    let max_path_length = calculate_max_path(start, end, 0, HashSet::new(), &grid);

    max_path_length.to_string()
}

fn calculate_max_path(
    start: Point,
    end: Point,
    current_length: usize,
    mut visited: HashSet<Point>,
    grid: &Grid<char>,
) -> usize {
    let mut stack = VecDeque::from([(start, current_length)]);
    let mut max_path_length = 0;
    while let Some((point, path_length)) = stack.pop_front() {
        visited.insert(point);

        if point == end {
            if max_path_length < path_length {
                dbg!(format!("Possible Max Path: {path_length}"));
                max_path_length = path_length;
            }

            continue;
        }

        let neighbors = point.neighbors(&path_length, &visited, grid);

        if let Some((first, rest)) = neighbors.split_first() {
            stack.push_back(*first);

            rest.iter().for_each(|(next_point, next_length)| {
                max_path_length = max_path_length.max(calculate_max_path(
                    *next_point,
                    end,
                    *next_length,
                    visited.clone(),
                    grid,
                ));
            });
        }
    }

    max_path_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "154".to_string());
    }
}
