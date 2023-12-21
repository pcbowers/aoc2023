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

    dbg!(starting_point);

    (steps + 1).pow(2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let result = process(include_str!("../data/example.txt"), 6);
        assert_eq!(result, "16".to_string());
    }

    #[test]
    fn test_process2() {
        let result = process(include_str!("../data/example.txt"), 10);
        assert_eq!(result, "50".to_string());
    }

    #[test]
    fn test_process3() {
        let result = process(include_str!("../data/example.txt"), 50);
        assert_eq!(result, "1594".to_string());
    }

    #[test]
    fn test_process4() {
        let result = process(include_str!("../data/example.txt"), 100);
        assert_eq!(result, "6536".to_string());
    }

    #[test]
    fn test_process5() {
        let result = process(include_str!("../data/example.txt"), 500);
        assert_eq!(result, "167004".to_string());
    }

    #[test]
    fn test_process6() {
        let result = process(include_str!("../data/example.txt"), 1000);
        assert_eq!(result, "668697".to_string());
    }

    #[test]
    fn test_process7() {
        let result = process(include_str!("../data/example.txt"), 5000);
        assert_eq!(result, "16733044".to_string());
    }
}
