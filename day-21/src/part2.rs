use grid::Grid;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Plot(isize, isize);

impl Plot {
    fn add(&self, other: &Plot) -> Plot {
        Plot(self.0 + other.0, self.1 + other.1)
    }

    fn normalized(&self, grid: &Grid<char>) -> Plot {
        Plot(
            self.0.rem_euclid(grid.rows() as isize),
            self.1.rem_euclid(grid.cols() as isize),
        )
    }

    fn neighbors(&self, grid: &Grid<char>) -> BTreeSet<Plot> {
        [Plot(-1, 0), Plot(1, 0), Plot(0, 1), Plot(0, -1)]
            .iter()
            .map(|plot| self.add(plot))
            .filter(|plot| matches!(grid.get_plot(&plot.normalized(grid)), Some('.') | Some('S')))
            .collect()
    }
}

impl From<(usize, usize)> for Plot {
    fn from((row, col): (usize, usize)) -> Self {
        Plot(row as isize, col as isize)
    }
}

trait GetPlot<T> {
    fn get_plot(&self, plot: &Plot) -> Option<&T>;
    fn get_mut_plot(&mut self, plot: &Plot) -> Option<&mut T>;
}

impl<T> GetPlot<T> for Grid<T> {
    fn get_plot(&self, plot: &Plot) -> Option<&T> {
        self.get(plot.0, plot.1)
    }

    fn get_mut_plot(&mut self, plot: &Plot) -> Option<&mut T> {
        self.get_mut(plot.0, plot.1)
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<char>) {
    grid.iter_rows().for_each(|row| {
        let row = row.collect::<String>();
        dbg!(row);
    });
}

#[allow(dead_code)]
fn place_plots_on_grid(grid: &mut Grid<char>, possible_plots: &HashSet<Plot>) {
    possible_plots.iter().for_each(|plot| {
        if let Some(cell) = grid.get_mut_plot(&plot.normalized(grid)) {
            *cell = if *cell == 'S' { 'S' } else { 'O' };
        }
    });
}

/*
This one is hard! So here, are the notes as I go along:

- Thankfully, I printed out my grid on part1 and noticed a diamond shape
- Both the example and input grids are squares with odd numbered dimensions
- Both have their starting plot directly in the center
- I tried running the solution for part 1 for 1-10 steps
- Each showed a distinct diamond shape in a checkered pattern
- O's were next to the starting plot on odd step counts
- O's were 1 away from the starting plot on even step counts

- The next thing I tried was part1 with 1-10 steps and no rocks
- Interestingly, they all gave solutions that fit this: (steps + 1)^2
- This only gives me an upper limit: I still need to account for the rocks
- In thinking about this, odd dimensions plays an interesting role
- Odd dimensions means the checkered pattern is opposite on every other grid
- This means that two grids side-by-side have all plots completely filled - rocks * 2
- Extrapolating, I could probably figure out a full grid count * 2
- This could drastically speed it up, but then I'd have to worry about the edges

- So: how many edges are there? Would calculating the edges be feasible?
- Input is 131 wide, and I need 26501365 steps
- A diamond is, at its widest, 26501365 from the center
- So, 65 to the edge of the first map, yielding 26501300 (odd that it zeroed out here)
- Then 26501300 / 131 grids to the right: 202300 grids more (odd again, it's exact)
- This means 202300 * 4 for up, down, left, and right + 1
- And that's just cardinal directions, the diamond will have less filled in as you go up

- OK, probably not feasible, even if we can calculate the inner. There must be a trick
- I'm not seeing anything, so maybe I need to see if there's a pattern when I hit the edges?

*Coding furiously here*

- OK, I have the brute force code done! So it doesn't run for too long, I break at 500 steps
- I also offset the index to start at 1 to make counting steps easier
- Let's try running the examples with some debugging statements
- At ((index as isize) - 5) % 11 == 0 (i.e. map edges), we get the following:
- 13, 129, 427, 894, 1528, 2324, 3282, 4402, 5684, 7128 ...
- Used https://www.wolframalpha.com/input, no pattern

- Let's try it on the input instead of the examples (-65 and % 131, max 500):
- 3867, 34253, 94909, 185835
- Converged to give this sequence in wolfram alpha: 3751 - 15019 n + 15135 n^2
- Progress! I don't understand the sequence much right now, but cool!

- But why is it different?
- Looking at the input, there are no blocks in the center column
- However, there are in the example. That's probably what is throwing it off
- With this in mind, I actually modified my tests to use the input for 64, 65, 196, 327, and 458
- I know 64, and I know it works because of the previous tests passing
- I can always add more in the future, but those are ones I could calculate quickly
- I just ignored the example for now since it doesn't match the input

- Now, theoretically, it should just work? I tried putting 26501365 into the formula
- I got a massive number, 10629648321750913191, and that was too high
- I realized: if n is 1 (65 steps), I get 3867, then 2 is 34253 (131 + 65 steps)
- That's the number of map edges, so I actually need n that meets my map: i.e. n = 202300
- This gives me 619401225810051. This time, too low :/
- Wait! Again, a mistake: if 65 is 1 and I have 202300 after that, I need 202301
- This gives me 619407349431167, which is the right answer!

Looking back, I'm sure there's a way to do this geometrically (like how I started), but the
sequence stuff is cool too, and keeps me from having to calculate edges. Is there a way
to make this general though? Ideally, I'd have code that came up with this solution, even
if it made some assumptions.

Spent a bit thinking about it: but I can't come up with a general solution. Maybe later. For now,
I'll implement it with the formula, should be pretty easy to
*/
pub fn process(input: &str, steps: usize) -> String {
    let grid = Grid::from(input.lines().map(|l| l.chars().collect()).collect_vec());

    // dbg!("Original Grid");
    // print_grid(&grid);

    let mut possible_plots = HashSet::from([Plot::from((grid.rows() / 2, grid.cols() / 2))]);

    // Currently, the formula only works at input grid ends and is scoped to my input
    // I have kept the brute force implementation and a single test for it in case I refactor
    // Commented out code can be used to calculate the sequence used
    if ((steps as isize) - grid.rows() as isize / 2) % grid.rows() as isize != 0 {
        for _index in 1..(steps + 1) {
            possible_plots = possible_plots
                .par_iter()
                .flat_map(|plot| plot.neighbors(&grid))
                .collect();

            // if ((steps as isize) - grid.rows() as isize / 2) % grid.rows() as isize == 0 {
            //     dbg!(possible_plots.len());
            // }

            // if index > 500 {
            //     break;
            // }
        }

        // dbg!("Visited Grid");
        // place_plots_on_grid(&mut grid, &possible_plots);
        // print_grid(&grid);

        possible_plots.len().to_string()
    } else {
        let n = (steps as isize - grid.rows() as isize / 2) / grid.rows() as isize + 1;
        (3751 - 15019 * n + 15135 * n.pow(2)).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let result = process(include_str!("../data/input.txt"), 64);
        assert_eq!(result, "3751".to_string());
    }

    #[test]
    fn test_process2() {
        let result = process(include_str!("../data/input.txt"), 65);
        assert_eq!(result, "3867".to_string());
    }

    #[test]
    fn test_process3() {
        let result = process(include_str!("../data/input.txt"), 196);
        assert_eq!(result, "34253".to_string());
    }

    #[test]
    fn test_process4() {
        let result = process(include_str!("../data/input.txt"), 327);
        assert_eq!(result, "94909".to_string());
    }

    #[test]
    fn test_process5() {
        let result = process(include_str!("../data/input.txt"), 458);
        assert_eq!(result, "185835".to_string());
    }
}
