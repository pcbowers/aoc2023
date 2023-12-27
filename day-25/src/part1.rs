use rustworkx_core::connectivity::stoer_wagner_min_cut;

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, line_ending},
        multi::{fold_many1, separated_list1},
        sequence::{separated_pair, terminated},
        IResult,
    };
    use petgraph::{graph::UnGraph, Graph, Undirected};
    use std::collections::HashMap;

    fn name(input: &str) -> IResult<&str, &str> {
        alpha1(input)
    }

    fn wires(input: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(tag(" "), name)(input)
    }

    fn component(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
        separated_pair(name, tag(": "), wires)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Graph<&str, (), Undirected>> {
        let mut node_indices = HashMap::new();

        let mut parser = fold_many1(
            terminated(component, line_ending),
            UnGraph::<&str, ()>::new_undirected,
            |mut graph, (name, wires)| {
                let from = *node_indices
                    .entry(name)
                    .or_insert_with(|| graph.add_node(name));

                wires.iter().for_each(|wire| {
                    let to = *node_indices
                        .entry(wire)
                        .or_insert_with(|| graph.add_node(wire));

                    graph.add_edge(from, to, ());
                });

                graph
            },
        );

        parser(input)
    }
}

pub fn process(input: &str) -> String {
    let (_, components) = parser::parse(input).expect("should parse");

    let (wires_cut, sub_group) = stoer_wagner_min_cut(&components, |_| Ok::<usize, ()>(1))
        .expect("should not error")
        .expect("should make a single cut");

    assert_eq!(wires_cut, 3, "should only cut 3 wires");

    let group_size_a = sub_group.len();
    let group_size_b = components.node_count() - sub_group.len();

    (group_size_a * group_size_b).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "54".to_string());
    }
}
