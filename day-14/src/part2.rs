use cached::{proc_macro::cached, Cached, Return};
use grid::Grid;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub fn process(input: &str) -> String {
    let grid = Grid::from_vec(
        input.lines().flat_map(|line| line.chars()).collect(),
        input.lines().next().map_or(0, |row| row.len()),
    );

    let size = (grid.rows(), grid.cols());
    let static_rocks = grid
        .indexed_iter()
        .filter_map(|(index, item)| item.eq(&'#').then_some((index, *item)))
        .collect::<HashMap<_, _>>();
    let mut all_rocks = grid
        .indexed_iter()
        .filter_map(|(index, item)| item.ne(&'.').then_some((index, *item)))
        .collect::<HashMap<_, _>>();

    let mut directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
    .iter()
    .cycle();

    let cycles = 1000000000;
    let iterations = cycles * 4usize;
    let mut repeat_iteration_size = 0;
    let mut first_cached_iteration = 0;
    let mut current_completed_count = 0;
    let mut first_cached_value: Option<HashMap<(usize, usize), char>> = None;
    for index in 1..=iterations {
        let direction = *directions.next().expect("Must always have a direction");
        let new_rocks = shift_rocks(&size, &static_rocks, &direction, &all_rocks);
        all_rocks = new_rocks.value;

        if new_rocks.was_cached && first_cached_value.is_none() {
            first_cached_value = Some(all_rocks.clone());
            first_cached_iteration = index;
        } else if Some(all_rocks.clone()) == first_cached_value {
            repeat_iteration_size = index - first_cached_iteration;
            current_completed_count = index;
            break;
        }
    }

    if repeat_iteration_size > 0 {
        for _ in 1..((iterations - current_completed_count) % repeat_iteration_size) {
            let direction = *directions.next().expect("Must always have a direction");
            let new_rocks = shift_rocks(&size, &static_rocks, &direction, &all_rocks);
            all_rocks = new_rocks.value;
        }
    }

    // dbg!(rocks_to_grid(&all_rocks, &size));

    let result = all_rocks
        .iter()
        .filter(|(_, value)| value == &&'O')
        .map(|(key, _)| size.0 - key.0)
        .sum::<usize>()
        .to_string();

    SHIFT_ROCKS.lock().unwrap().cache_reset();

    result
}

pub fn rocks_to_grid(
    all_rocks: &HashMap<(usize, usize), char>,
    size: &(usize, usize),
) -> Vec<String> {
    (0..size.0)
        .map(|row| {
            (0..size.1)
                .map(move |col| all_rocks.get(&(row, col)).unwrap_or(&'.'))
                .collect::<String>()
        })
        .collect_vec()
}

#[cached(
    key = "(Direction, Vec<String>)",
    convert = r#"{ (*direction, rocks_to_grid(all_rocks, size)) }"#,
    with_cached_flag = true
)]
pub fn shift_rocks(
    size: &(usize, usize),
    static_rocks: &HashMap<(usize, usize), char>,
    direction: &Direction,
    all_rocks: &HashMap<(usize, usize), char>,
) -> Return<HashMap<(usize, usize), char>> {
    let mut new_rocks = static_rocks.clone();

    let outer_range: Box<dyn Iterator<Item = usize>> = match direction {
        Direction::North | Direction::South => Box::new(0..size.1),
        Direction::East | Direction::West => Box::new(0..size.0),
    };

    for outer in outer_range {
        let inner_range: Box<dyn Iterator<Item = usize>> = match direction {
            Direction::North => Box::new(0..size.0),
            Direction::South => Box::new((0..size.0).rev()),
            Direction::East => Box::new((0..size.1).rev()),
            Direction::West => Box::new(0..size.1),
        };

        let mut next_position = match direction {
            Direction::North => (0usize, outer),
            Direction::South => (size.0 - 1, outer),
            Direction::East => (outer, size.1 - 1),
            Direction::West => (outer, 0usize),
        };

        for inner in inner_range {
            let current_point = match direction {
                Direction::North | Direction::South => (inner, outer),
                Direction::East | Direction::West => (outer, inner),
            };

            match all_rocks.get(&current_point) {
                Some('#') => match direction {
                    Direction::North => next_position.0 = inner.saturating_add(1),
                    Direction::South => next_position.0 = inner.saturating_sub(1),
                    Direction::East => next_position.1 = inner.saturating_sub(1),
                    Direction::West => next_position.1 = inner.saturating_add(1),
                },
                Some('O') => {
                    new_rocks.insert(next_position, 'O');
                    match direction {
                        Direction::North => next_position.0 = next_position.0.saturating_add(1),
                        Direction::South => next_position.0 = next_position.0.saturating_sub(1),
                        Direction::East => next_position.1 = next_position.1.saturating_sub(1),
                        Direction::West => next_position.1 = next_position.1.saturating_add(1),
                    }
                }
                _ => {}
            }
        }
    }

    Return::new(new_rocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "64".to_string());
    }
}
