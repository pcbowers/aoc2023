use grid::Grid;
use itertools::Itertools;

pub fn process(input: &str) -> String {
    input
        .split("\n\n")
        .filter_map(|pattern| {
            let grid = Grid::from_vec(
                pattern.lines().flat_map(|line| line.chars()).collect(),
                pattern.lines().next().map_or(0, |row| row.len()),
            );

            let rows = grid.iter_rows().map(|row| row.collect_vec()).collect_vec();
            let horizontal_reflection = rows
                .iter()
                .enumerate()
                .tuple_windows()
                .filter(|((_, a), (_, b))| a == b)
                .find_map(|((a, _), (b, _))| {
                    // compare char vectors
                    let a_lines = rows[0..=a].iter().rev();
                    let b_lines = rows[b..].iter();

                    a_lines
                        .zip(b_lines)
                        .all(|values| values.0 == values.1)
                        .then_some(a + 1)
                });

            if let Some(rows_above_horizontal_reflection) = horizontal_reflection {
                return Some(rows_above_horizontal_reflection * 100);
            }

            let cols = grid.iter_cols().map(|col| col.collect_vec()).collect_vec();
            let vertical_reflection = cols
                .iter()
                .enumerate()
                .tuple_windows()
                .filter(|((_, a), (_, b))| a == b)
                .find_map(|((a, _), (b, _))| {
                    // compare char vectors
                    let a_lines = cols[0..=a].iter().rev();
                    let b_lines = cols[b..].iter();

                    a_lines
                        .zip(b_lines)
                        .all(|values| values.0 == values.1)
                        .then_some(a + 1)
                });

            vertical_reflection
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "405".to_string());
    }
}
