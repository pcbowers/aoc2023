use std::{cell::Cell, collections::LinkedList};

use grid::Grid;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GridCell {
    pub name: char,
    pub row: usize,
    pub col: usize,
    pub distance: Cell<isize>,
}

impl GridCell {
    pub fn as_name(&self) -> String {
        self.name.to_string()
    }

    pub fn as_pretty_name(&self) -> String {
        match self.name {
            '|' => '│',
            '-' => '─',
            'L' => '└',
            'J' => '┘',
            '7' => '┐',
            'F' => '┌',
            '.' => ' ',
            'S' => '⚐',
            _ => unreachable!("Should only match these characters"),
        }
        .to_string()
    }

    pub fn as_distance(&self) -> String {
        format!(
            "{:3}",
            match self.distance.get() {
                distance if distance >= 0 => distance.to_string(),
                _ => " ".to_string(),
            }
        )
    }
}

impl GridCell {
    fn new(name: char, row: usize, col: usize) -> Self {
        GridCell {
            name,
            row,
            col,
            distance: Cell::default(),
        }
    }
}

pub fn parse(input: &str) -> Grid<GridCell> {
    Grid::from_vec(
        input
            .lines()
            .enumerate()
            .flat_map(|(row, chars)| {
                chars
                    .chars()
                    .enumerate()
                    .map(move |(col, cell)| GridCell::new(cell, row, col))
            })
            .collect_vec(),
        input.lines().next().unwrap().chars().count(),
    )
}

pub fn print_grid(grid: &Grid<GridCell>, grid_cell_converter: fn(&GridCell) -> String) {
    grid.iter_rows().for_each(|row| {
        println!("{}", row.map(grid_cell_converter).join(""));
    });
}

pub fn print_big_grid(big_grid: &Grid<char>) {
    big_grid.iter_rows().for_each(|row| {
        println!("{}", row.copied().join(""));
    })
}

pub fn get_the_loop(grid: &Grid<GridCell>) -> LinkedList<&GridCell> {
    let mut visited_cells: LinkedList<&GridCell> = LinkedList::new();
    let mut cells_to_visit = LinkedList::from([grid
        .iter()
        .find(|cell| cell.name == 'S')
        .expect("Must be a starting cell")]);

    while let Some(cell) = cells_to_visit.pop_front() {
        let up = match cell.name {
            'S' | '|' | 'L' | 'J' => {
                if cell.row == 0 {
                    None
                } else {
                    grid.get(cell.row - 1, cell.col)
                }
            }
            _ => None,
        };
        if let Some(next_cell) = up {
            match next_cell.name {
                '|' | '7' | 'F' => {
                    if next_cell.distance.get() == isize::default() {
                        next_cell.distance.set(cell.distance.get() + 1);
                        cells_to_visit.push_back(next_cell);
                    }
                }
                _ => (),
            }
        }

        let down = match cell.name {
            'S' | '|' | '7' | 'F' => grid.get(cell.row + 1, cell.col),
            _ => None,
        };
        if let Some(next_cell) = down {
            match next_cell.name {
                '|' | 'L' | 'J' => {
                    if next_cell.distance.get() == isize::default() {
                        next_cell.distance.set(cell.distance.get() + 1);
                        cells_to_visit.push_back(next_cell);
                    }
                }
                _ => (),
            }
        }

        let left = match cell.name {
            'S' | '-' | '7' | 'J' => {
                if cell.col == 0 {
                    None
                } else {
                    grid.get(cell.row, cell.col - 1)
                }
            }
            _ => None,
        };
        if let Some(next_cell) = left {
            match next_cell.name {
                '-' | 'L' | 'F' => {
                    if next_cell.distance.get() == isize::default() {
                        next_cell.distance.set(cell.distance.get() + 1);
                        cells_to_visit.push_back(next_cell);
                    }
                }
                _ => (),
            }
        }

        let right = match cell.name {
            'S' | '-' | 'L' | 'F' => grid.get(cell.row, cell.col + 1),
            _ => None,
        };
        if let Some(next_cell) = right {
            match next_cell.name {
                '-' | '7' | 'J' => {
                    if next_cell.distance.get() == isize::default() {
                        next_cell.distance.set(cell.distance.get() + 1);
                        cells_to_visit.push_back(next_cell);
                    }
                }
                _ => (),
            }
        }

        visited_cells.push_back(cell);
    }

    visited_cells
}

pub fn calculate_name(start: &GridCell, the_loop: &LinkedList<&GridCell>) -> char {
    if start.name != 'S' {
        return start.name;
    }

    let neighbors = the_loop
        .iter()
        .filter(|cell| cell.distance.get() == 1)
        .collect_vec();

    let top = neighbors
        .iter()
        .find(|cell| start.col == cell.col && start.row != 0 && start.row - 1 == cell.row);

    let down = neighbors
        .iter()
        .find(|cell| start.col == cell.col && start.row + 1 == cell.row);

    let left = neighbors
        .iter()
        .find(|cell| start.col != 0 && start.col - 1 == cell.col && start.row == cell.row);

    let right = neighbors
        .iter()
        .find(|cell| start.col + 1 == cell.col && start.row == cell.row);

    match (top, down, left, right) {
        (Some(_), Some(_), None, None) => '|',
        (None, None, Some(_), Some(_)) => '-',
        (Some(_), None, None, Some(_)) => 'L',
        (Some(_), None, Some(_), None) => 'J',
        (None, Some(_), Some(_), None) => '7',
        (None, Some(_), None, Some(_)) => 'F',
        _ => unreachable!("Must be a valid pipe character"),
    }
}

pub fn process(input: &str) -> String {
    let grid = parse(input);

    // print_grid(&grid, GridCell::as_pretty_name);

    let loop_char = '#';
    let outside_char = '.';
    let flood_char = 'o';

    let mut big_grid = Grid::new(grid.rows() * 3, grid.cols() * 3);
    big_grid.fill(outside_char);

    let the_loop = get_the_loop(&grid);

    the_loop.iter().for_each(|cell| {
        let row = cell.row * 3;
        let col = cell.col * 3;
        match calculate_name(cell, &the_loop) {
            '|' => {
                *big_grid.get_mut(row, col + 1).unwrap() = loop_char;
                *big_grid.get_mut(row + 1, col + 1).unwrap() = loop_char;
                *big_grid.get_mut(row + 2, col + 1).unwrap() = loop_char;
            }
            '-' => {
                *big_grid.get_mut(row + 1, col).unwrap() = loop_char;
                *big_grid.get_mut(row + 1, col + 1).unwrap() = loop_char;
                *big_grid.get_mut(row + 1, col + 2).unwrap() = loop_char;
            }
            'L' => {
                *big_grid.get_mut(row, col + 1).unwrap() = loop_char;
                *big_grid.get_mut(row + 1, col + 1).unwrap() = loop_char;
                *big_grid.get_mut(row + 1, col + 2).unwrap() = loop_char;
            }
            'J' => {
                *big_grid.get_mut(row, col + 1).unwrap() = loop_char;
                *big_grid.get_mut(row + 1, col).unwrap() = loop_char;
                *big_grid.get_mut(row + 1, col + 1).unwrap() = loop_char;
            }
            '7' => {
                *big_grid.get_mut(row + 1, col).unwrap() = loop_char;
                *big_grid.get_mut(row + 1, col + 1).unwrap() = loop_char;
                *big_grid.get_mut(row + 2, col + 1).unwrap() = loop_char;
            }
            'F' => {
                *big_grid.get_mut(row + 1, col + 1).unwrap() = loop_char;
                *big_grid.get_mut(row + 1, col + 2).unwrap() = loop_char;
                *big_grid.get_mut(row + 2, col + 1).unwrap() = loop_char;
            }
            _ => unreachable!("Must match one of these characters"),
        };
    });

    let mut cells_to_visit = LinkedList::from([(0, 0)]);

    while let Some((row, col)) = cells_to_visit.pop_back() {
        if let Some(cell) = big_grid.get_mut(row, col) {
            *cell = flood_char;
        }

        if row != 0 {
            if let Some(cell) = big_grid.get(row - 1, col) {
                if cell == &outside_char {
                    cells_to_visit.push_back((row - 1, col))
                }
            }
        }

        if let Some(cell) = big_grid.get(row + 1, col) {
            if cell == &outside_char {
                cells_to_visit.push_back((row + 1, col))
            }
        }

        if col != 0 {
            if let Some(cell) = big_grid.get(row, col - 1) {
                if cell == &outside_char {
                    cells_to_visit.push_back((row, col - 1))
                }
            }
        }

        if let Some(cell) = big_grid.get(row, col + 1) {
            if cell == &outside_char {
                cells_to_visit.push_back((row, col + 1))
            }
        }
    }

    // print_big_grid(&big_grid);

    big_grid
        .iter_rows()
        .skip(1)
        .step_by(3)
        .flat_map(|row| row.skip(1).step_by(3).filter(|&cell| cell == &outside_char))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let result = process(include_str!("../data/example2.1.txt"));
        assert_eq!(result, "4".to_string());
    }

    #[test]
    fn test_process2() {
        let result = process(include_str!("../data/example2.2.txt"));
        assert_eq!(result, "4".to_string());
    }

    #[test]
    fn test_process3() {
        let result = process(include_str!("../data/example2.3.txt"));
        assert_eq!(result, "8".to_string());
    }

    #[test]
    fn test_process4() {
        let result = process(include_str!("../data/example2.4.txt"));
        assert_eq!(result, "10".to_string());
    }
}
