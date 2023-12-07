use day_05::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../data/input.txt",)));
}

#[divan::bench(sample_count = 10)]
fn part2() {
    part2::process(divan::black_box(include_str!("../data/input.txt",)));
}
