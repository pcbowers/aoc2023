use cached::proc_macro::cached;
use grid::Grid;
use itertools::Itertools;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(isize, isize);

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }

    fn neighbors(&self, grid: &Grid<char>) -> BTreeSet<Point> {
        [Point(-1, 0), Point(1, 0), Point(0, 1), Point(0, -1)]
            .iter()
            .map(|point| self.add(point))
            .filter(|point| matches!(grid.get(point.0, point.1), Some('.') | Some('S')))
            .collect()
    }
}

impl From<(usize, usize)> for Point {
    fn from((row, col): (usize, usize)) -> Self {
        Point(row as isize, col as isize)
    }
}

pub fn process(input: &str, steps: usize) -> String {
    let grid = Grid::from(input.lines().map(|l| l.chars().collect()).collect_vec());

    let (starting_point, _) = grid
        .indexed_iter()
        .find(|(_, &cell)| cell == 'S')
        .expect("should have a starting point");

    // dbg!("Original Grid");
    // grid.iter_rows().for_each(|row| {
    //     let row = row.collect::<String>();
    //     dbg!(row);
    // });

    let possible_plots = garden_plots(steps, Point::from(starting_point), &grid);

    // dbg!("Visited Grid");
    // grid.iter_rows().enumerate().for_each(|(row, line)| {
    //     let row = line
    //         .enumerate()
    //         .map(|(col, &cell)| {
    //             if possible_plots.contains(&Point::from((row, col))) && cell != 'S' {
    //                 'O'
    //             } else {
    //                 cell
    //             }
    //         })
    //         .collect::<String>();
    //     dbg!(row);
    // });

    possible_plots.len().to_string()
}

#[cached(key = "(usize, Point)", convert = r#"{ (steps_left, point) }"#)]
fn garden_plots(steps_left: usize, point: Point, grid: &Grid<char>) -> BTreeSet<Point> {
    let neighbors = point.neighbors(grid);

    if steps_left == 1 {
        return neighbors;
    }

    neighbors
        .iter()
        .fold(BTreeSet::new(), |mut points, neighbor| {
            points.extend(garden_plots(steps_left - 1, *neighbor, grid));
            points
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"), 10);
        assert_eq!(result, "16".to_string());
    }
}
