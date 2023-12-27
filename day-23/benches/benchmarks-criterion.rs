use criterion::{criterion_group, criterion_main, Criterion};
use day_23::*;

fn criterion_benchmark_part1(c: &mut Criterion) {
    let input = include_str!("../data/input.txt");

    let mut group = c.benchmark_group("day_23::part1");
    group.bench_with_input("part1", input, |b, input| b.iter(|| part1::process(input)));

    group.finish();
}

fn criterion_benchmark_part2(c: &mut Criterion) {
    let input = include_str!("../data/input.txt");

    let mut group = c.benchmark_group("day_23::part2");
    group.bench_with_input("part2", input, |b, input| b.iter(|| part2::process(input)));

    group.finish();
}

criterion_group!(
    benches,
    criterion_benchmark_part1,
    criterion_benchmark_part2
);
criterion_main!(benches);