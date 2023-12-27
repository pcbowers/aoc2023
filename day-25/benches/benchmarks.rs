use day_25::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(max_time = 100)]
fn part1() {
    part1::process(divan::black_box(include_str!("../data/input.txt",)));
}
