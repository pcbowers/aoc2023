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

pub fn process(input: &str) -> String {
    let grid = parse(input);

    // print_grid(&grid, GridCell::as_name);
    // print_grid(&grid, GridCell::as_pretty_name);

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

    // print_grid(&grid, GridCell::as_distance);

    visited_cells
        .iter()
        .map(|cell| cell.distance.get())
        .max()
        .expect("There must be a max distance")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example1.txt"));
        assert_eq!(result, "8".to_string());
    }
}
