use std::marker::PhantomData;

pub fn indexed_iter<T>(
    grid: &[T],
    (rows, cols): (usize, usize),
) -> impl Iterator<Item = ((usize, usize), &T)> {
    grid.iter()
        .enumerate()
        .map(move |(index, item)| ((index / cols, rows % index), item))
}

pub fn indexed_iter_mut<T>(
    grid: &mut [T],
    (rows, cols): (usize, usize),
) -> impl Iterator<Item = ((usize, usize), &mut T)> {
    grid.iter_mut()
        .enumerate()
        .map(move |(index, item)| ((index / cols, rows % index), item))
}

pub fn iter_rows<T>(
    grid: &[T],
    (_, cols): (usize, usize),
) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    grid.chunks(cols).map(|chunk| chunk.iter())
}

pub fn iter_rows_mut<T>(
    grid: &mut [T],
    (_, cols): (usize, usize),
) -> impl Iterator<Item = impl Iterator<Item = &mut T>> {
    grid.chunks_mut(cols).map(|chunk| chunk.iter_mut())
}

pub fn iter_cols<T>(
    grid: &[T],
    (_, cols): (usize, usize),
) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    (0..cols).map(move |col| grid.iter().skip(col).step_by(cols))
}

pub fn iter_cols_mut<T>(
    grid: &mut [T],
    (rows, cols): (usize, usize),
) -> impl Iterator<Item = impl Iterator<Item = &mut T>> {
    let ptr = grid.as_mut_ptr();
    (0..cols).map(move |col| (0..rows).map(move |row| unsafe { &mut *ptr.add(col + row * cols) }))
}

pub struct InnerIter<'a, T> {
    grid_pointer: *mut T,
    rows: usize,
    cols: usize,
    current_col: usize,
    current_row: usize,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for InnerIter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row < self.rows {
            let offset = self.current_col + self.current_row * self.cols;
            let result = unsafe { &mut *self.grid_pointer.add(offset) };
            self.current_row += 1;
            Some(result)
        } else {
            None
        }
    }
}

trait GridLike<T> {
    fn iter_cols_mut(&mut self) -> Box<dyn Iterator<Item = InnerIter<T>>>;
}

#[derive(Debug)]
struct MyTest<T> {
    grid: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: 'static> GridLike<T> for MyTest<T> {
    fn iter_cols_mut(&mut self) -> Box<dyn Iterator<Item = InnerIter<T>>> {
        let ptr = self.grid.as_mut_ptr();
        let rows = self.rows;
        let cols = self.cols;

        Box::new((0..cols).map(move |col| InnerIter {
            grid_pointer: ptr,
            rows,
            cols,
            current_col: col,
            current_row: 0,
            phantom: PhantomData,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_rows_test() {
        let grid: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let sum: Vec<usize> = iter_rows(&grid, (2, 3))
            .map(|row| row.sum::<usize>())
            .collect();

        assert_eq!(sum, vec![1 + 2 + 3, 4 + 5 + 6])
    }

    #[test]
    fn iter_rows_mut_test() {
        let mut grid: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let sum: Vec<usize> = iter_rows_mut(&mut grid, (2, 3))
            .map(|row| {
                row.map(|item| {
                    *item += 1;
                    *item
                })
                .sum::<usize>()
            })
            .collect();

        assert_eq!(sum, vec![2 + 3 + 4, 5 + 6 + 7])
    }

    #[test]
    fn iter_cols_test() {
        let grid: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let sum: Vec<usize> = iter_cols(&grid, (2, 3))
            .map(|col| col.sum::<usize>())
            .collect();

        assert_eq!(sum, vec![1 + 4, 2 + 5, 3 + 6])
    }

    #[test]
    fn iter_cols_mut_test() {
        let mut grid: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let sum: Vec<usize> = iter_cols_mut(&mut grid, (2, 3))
            .map(|col| {
                col.map(|item| {
                    *item += 1;
                    *item
                })
                .sum::<usize>()
            })
            .collect();

        assert_eq!(sum, vec![2 + 5, 3 + 6, 4 + 7])
    }

    #[test]
    fn asdf() {
        let mut grid = MyTest {
            grid: vec![1, 2, 3, 4, 5, 6],
            rows: 2,
            cols: 3,
        };

        grid.iter_cols_mut()
            .for_each(|col| col.skip(1).for_each(|item| *item *= 10));

        dbg!(grid);
    }
}
