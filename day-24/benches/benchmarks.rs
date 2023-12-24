use day_24::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(max_time = 100)]
fn part1() {
    part1::process(
        divan::black_box(include_str!("../data/input.txt",)),
        divan::black_box(200000000000000),
        divan::black_box(400000000000000),
    );
}

#[divan::bench(max_time = 100)]
fn part2() {
    part2::process(divan::black_box(include_str!("../data/input.txt",)));
}
