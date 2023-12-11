use day_10::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../data/input.txt",)));
}

#[divan::bench(max_time = 100)]
fn part2() {
    part2::process(divan::black_box(include_str!("../data/input.txt",)));
}
