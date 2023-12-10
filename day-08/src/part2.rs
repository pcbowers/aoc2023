use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::alphanumeric1,
    sequence::{terminated, tuple},
    IResult,
};
use num::Integer;
use std::collections::HashMap;

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn parse_point(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((
        terminated(alphanumeric1, tag(" = (")),
        terminated(alphanumeric1, tag(", ")),
        terminated(alphanumeric1, tag(")")),
    ))(input)
}

/*
As a little background for those who may run this themselves:
- I first tried the naive approach: moving each node one at a time, waiting until they were all at an end node
- I ran that for 3 hours: nothing. I don't even know if it was close, but clearly there was something faster
- At this point, I decided to start looking for patterns
- I looped through each starting node
- For each starting node, I printed the first end node it reached with its index
- That information wasn't super helpful, so I added 30*index after it hit the first end node
- This was interesting: each start node only ever hit one, and the indexes were direct multiples of the first
- Based on these findings, a loop is inevitable at that value, so it's simply LCM
- Kept the iteration, but this time just outputted the number, then calculate for the LCM to get the answer

^^^ This is important as it basically shows that the solution requires a certain kind of input. This is not a general solution.
*/

pub fn process(input: &str) -> String {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();
    let nodes: HashMap<&str, Node> = lines
        .filter_map(|line| parse_point(line).ok())
        .map(|(_, names)| {
            (
                names.0,
                Node {
                    left: names.1,
                    right: names.2,
                },
            )
        })
        .collect();

    let node_names = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .copied()
        .collect_vec();

    node_names
        .iter()
        .map(|node_name| {
            let mut current_node = *node_name;

            instructions
                .clone()
                .enumerate()
                .find_map(|(index, instruction)| {
                    if current_node.ends_with('Z') {
                        Some(index)
                    } else {
                        current_node = match instruction {
                            'R' => nodes[current_node].right,
                            'L' => nodes[current_node].left,
                            _ => unreachable!(),
                        };
                        None
                    }
                })
                .expect("An end node must be found")
        })
        .reduce(|acc, steps| acc.lcm(&steps))
        .expect("The Lowest Common Multiple must be calculated")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example2.txt"));
        assert_eq!(result, "6".to_string());
    }
}
