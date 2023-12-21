use grid::Grid;
use itertools::Itertools;
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Plot(isize, isize);

impl Plot {
    fn add(&self, other: &Plot) -> Plot {
        Plot(self.0 + other.0, self.1 + other.1)
    }

    fn neighbors(&self, grid: &Grid<char>) -> BTreeSet<Plot> {
        [Plot(-1, 0), Plot(1, 0), Plot(0, 1), Plot(0, -1)]
            .iter()
            .map(|plot| self.add(plot))
            .filter(|plot| matches!(grid.get(plot.0, plot.1), Some('.') | Some('S')))
            .collect()
    }
}

impl From<(usize, usize)> for Plot {
    fn from((row, col): (usize, usize)) -> Self {
        Plot(row as isize, col as isize)
    }
}

fn print_grid(grid: &Grid<char>) {
    grid.iter_rows().for_each(|row| {
        let row = row.collect::<String>();
        dbg!(row);
    });
}

fn place_plots_on_grid(grid: &mut Grid<char>, possible_plots: &HashSet<Plot>) {
    possible_plots.iter().for_each(|plot| {
        if let Some(cell) = grid.get_mut(plot.0, plot.1) {
            *cell = if *cell == 'S' { 'S' } else { 'O' };
        }
    });
}

pub fn process(input: &str, steps: usize) -> String {
    let mut grid = Grid::from(input.lines().map(|l| l.chars().collect()).collect_vec());

    let (starting_point, _) = grid
        .indexed_iter()
        .find(|(_, &cell)| cell == 'S')
        .expect("should have a starting point");

    dbg!("Original Grid");
    print_grid(&grid);

    let mut possible_plots = HashSet::from([Plot::from(starting_point)]);
    for _ in 0..steps {
        possible_plots = possible_plots
            .iter()
            .flat_map(|plot| plot.neighbors(&grid))
            .collect();
    }

    dbg!("Visited Grid");
    place_plots_on_grid(&mut grid, &possible_plots);
    print_grid(&grid);

    possible_plots.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"), 6);
        assert_eq!(result, "16".to_string());
    }
}
