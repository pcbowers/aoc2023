use day_24::part1::process;

fn main() {
    let result = process(
        include_str!("../../data/input.txt"),
        200000000000000,
        400000000000000,
    );
    println!("Part 1 Answer: {}", result);
}
