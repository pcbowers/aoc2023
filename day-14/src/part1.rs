use grid::Grid;
use itertools::Itertools;

pub fn process(input: &str) -> String {
    let grid = Grid::from_vec(
        input.lines().flat_map(|line| line.chars()).collect(),
        input.lines().next().map_or(0, |row| row.len()),
    );

    let mut new_grid = Grid::new(grid.cols(), grid.rows());

    grid.iter_cols().enumerate().for_each(|(index, col)| {
        let mut new_column = Vec::from([('X', 1)]);
        col.group_by(|key| *key)
            .into_iter()
            .map(|(group, items)| (*group, items.count()))
            .collect_vec()
            .into_iter()
            .rev()
            .fold(&mut new_column, |column, group| {
                let last_index = column.len() - 1;
                let last_column = column[last_index];

                if last_column.0 == 'X' {
                    column.push(group);
                    return column;
                }

                if last_column.0 == 'O' && group.0 == 'O' {
                    column[last_index] = ('O', last_column.1 + group.1);
                    return column;
                }

                if last_column.0 == 'O' && group.0 == '.' {
                    column.push(group);
                    column.swap(last_index, last_index + 1);
                } else {
                    column.push(group);
                }

                column
            });

        new_grid
            .iter_col_mut(index)
            .zip(new_column[1..].iter().flat_map(|a| [a.0].repeat(a.1)))
            .for_each(|(grid_element, new_value)| *grid_element = new_value);
    });

    new_grid
        .iter_rows()
        .enumerate()
        .map(|(index, row)| row.filter(|char| char == &&'O').count() * (index + 1))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "136".to_string());
    }
}
