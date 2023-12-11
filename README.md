# aoc2023

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2023/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2023/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2023/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2023/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2023/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2023/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2023/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2023/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2023/day/9) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

## Benchmarks

See benchmarks [here](./benchmarks.txt). Benchmarks are run ad-hoc without closing everything else. All benchmarks are run in WSL (Ubuntu 22.04.3 LTS) on my Windows PC (13th Gen Intel Core i9-13900K 16-Core Processor with 32GB Memory).

## Usage

I use the following command line tools (all can be installed via cargo):

* [rustfmt](https://github.com/rust-lang/rustfmt)
* [clippy](https://github.com/rust-lang/rust-clippy)
* [nextest](https://github.com/nextest-rs/nextest)
* [cargo-generate](https://github.com/cargo-generate/cargo-generate)
* [just](https://github.com/casey/just)
* [aoc-cli](https://github.com/scarvalhojr/aoc-cli)

To install them all, run:

```
rustup component add rustfmt clippy
cargo install nextest cargo-generate just aoc-cli
```

Instead of remembering all the commands that need to be run, I've set up a [`justfile`](./justfile). For a list of the predefined commands, run `just help`. All commands used from linting to benchmarking are available through the just runner.
