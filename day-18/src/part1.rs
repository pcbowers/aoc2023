use grid::Grid;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{self, anychar, space1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, (char, u64, &str)> {
    tuple((
        terminated(anychar, space1),
        terminated(complete::u64, space1),
        delimited(tag("("), take_until1(")"), tag(")")),
    ))(input)
}

pub fn process(input: &str) -> String {
    // parse the dig plan into memory
    let dig_plan = input
        .lines()
        .filter_map(|line| parse(line).ok())
        .map(|(_, (direction, meters, _))| (direction, meters as isize))
        .collect_vec();

    // Create a path using the dig plan
    let mut dig_path: Vec<(isize, isize)> = Vec::from([(0, 0)]);
    for instruction in dig_plan.iter() {
        let end = *dig_path.last().expect("Must have end path");
        let meters = instruction.1;
        match instruction.0 {
            'U' => (1..=meters).for_each(|up| {
                dig_path.push((end.0 - up, end.1));
            }),
            'D' => (1..=meters).for_each(|down| {
                dig_path.push((end.0 + down, end.1));
            }),
            'L' => (1..=meters).for_each(|left| {
                dig_path.push((end.0, end.1 - left));
            }),
            'R' => (1..=meters).for_each(|right| {
                dig_path.push((end.0, end.1 + right));
            }),
            _ => unreachable!("Must have a direction"),
        }
    }

    // The last one completes the loop and is not needed
    dig_path.pop();

    // Ensure there are no negative rows
    let lowest_row = dig_path.iter().map(|point| point.0).min().unwrap();
    if lowest_row != 0 {
        let difference = 0 - lowest_row;
        dig_path.iter_mut().for_each(|point| point.0 += difference);
    }

    // Ensure there are no negative columns
    let lowest_col = dig_path.iter().map(|point| point.1).min().unwrap();
    if lowest_col != 0 {
        let difference = 0 - lowest_col;
        dig_path.iter_mut().for_each(|point| point.1 += difference);
    }

    // calculate the maximum dimensions
    let dimensions = (
        dig_path.iter().map(|point| point.0).max().unwrap() as usize + 1,
        dig_path.iter().map(|point| point.1).max().unwrap() as usize + 1,
    );

    // create the grid
    let mut grid = Grid::new(dimensions.0, dimensions.1);

    // fill the grid
    grid.fill('.');
    dig_path.iter().for_each(|point| {
        *grid.get_mut(point.0 as usize, point.1 as usize).unwrap() = '#';
    });

    // find an inside cell
    let first_inside_point = grid
        .iter_row(1)
        .enumerate()
        .find(|(col, point)| {
            point == &&'.'
                && grid.get(0, *col).unwrap() == &'#'
                && grid.get(1, col.saturating_sub(1)).unwrap() == &'#'
        })
        .map(|(col, _)| (1_usize, col))
        .unwrap();

    // fill the inside cells
    let mut cells_to_visit = Vec::from([first_inside_point]);
    while let Some((row, col)) = cells_to_visit.pop() {
        *grid.get_mut(row, col).unwrap() = '#';

        if row != 0 && grid.get(row - 1, col) == Some(&'.') {
            cells_to_visit.push((row - 1, col))
        }
        if grid.get(row + 1, col) == Some(&'.') {
            cells_to_visit.push((row + 1, col))
        }
        if col != 0 && grid.get(row, col - 1) == Some(&'.') {
            cells_to_visit.push((row, col - 1))
        }
        if grid.get(row, col + 1) == Some(&'.') {
            cells_to_visit.push((row, col + 1))
        }
    }

    // show the grid
    // grid.iter_rows().for_each(|row| {
    //     let row = row.collect::<String>();
    //     dbg!(row);
    // });

    // count the dug cells
    grid.iter().filter(|cell| cell == &&'#').count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "62".to_string());
    }
}
