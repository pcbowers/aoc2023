use itertools::Itertools;

trait Transpose {
    fn transpose(&self) -> Self;
}

impl<T: Clone> Transpose for Vec<Vec<T>> {
    fn transpose(&self) -> Self {
        let row_length = self.first().map_or(0, |line| line.len());
        let mut row_iterations: Vec<_> = self.clone().into_iter().map(|n| n.into_iter()).collect();
        (0..row_length)
            .map(|_| row_iterations.iter_mut().filter_map(|n| n.next()).collect())
            .collect()
    }
}

pub fn process(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let galaxies = grid.iter().enumerate().flat_map(|(row, line)| {
        line.iter()
            .enumerate()
            .filter_map(move |(col, pixel)| pixel.eq(&'#').then_some((row as isize, col as isize)))
    });

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter_map(|(row, line)| line.iter().all(|&c| c == '.').then_some(row))
        .collect_vec();

    let empty_cols = grid
        .transpose()
        .iter()
        .enumerate()
        .filter_map(|(col, line)| line.iter().all(|&c| c == '.').then_some(col))
        .collect_vec();

    galaxies
        .tuple_combinations()
        .map(|(a, b)| {
            let row_extra = empty_rows
                .iter()
                .filter_map(|row| {
                    (a.0.min(b.0)..a.0.max(b.0))
                        .contains(&(*row as isize))
                        .then_some(1000000 - 1)
                })
                .sum::<isize>();

            let col_extra = empty_cols
                .iter()
                .filter_map(|col| {
                    (a.1.min(b.1)..a.1.max(b.1))
                        .contains(&(*col as isize))
                        .then_some(1000000 - 1)
                })
                .sum::<isize>();

            (a.0 - b.0).abs() + (a.1 - b.1).abs() + row_extra + col_extra
        })
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "82000210".to_string());
    }
}
