use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleKind<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Pulse>),
}

impl<'a> ModuleKind<'a> {
    fn output(&mut self, pulse: Pulse, from: &'a str) -> Option<Pulse> {
        match self {
            ModuleKind::Broadcast => Some(pulse),
            ModuleKind::FlipFlop(is_on) => pulse.eq(&Pulse::Low).then(|| {
                *is_on = !*is_on;
                match is_on {
                    false => Pulse::Low,
                    true => Pulse::High,
                }
            }),
            ModuleKind::Conjunction(memory) => {
                memory.insert(from, pulse);
                if memory.values().all(|pulse| pulse == &Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module<'a> {
    name: &'a str,
    kind: ModuleKind<'a>,
    targets: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn output(&mut self, pulse: Pulse, from: &'a str) -> Option<(Pulse, Vec<&'a str>)> {
        if let Some(new_pulse) = self.kind.output(pulse, from) {
            Some((new_pulse, self.targets.to_vec()))
        } else {
            None
        }
    }
}

mod parser {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, line_ending},
        combinator::{map, value},
        multi::{fold_many1, separated_list1},
        sequence::{terminated, tuple},
        IResult,
    };
    use std::collections::HashMap;

    fn name(input: &str) -> IResult<&str, &str> {
        terminated(alpha1, tag(" -> "))(input)
    }

    fn kind(input: &str) -> IResult<&str, ModuleKind> {
        alt((
            value(ModuleKind::FlipFlop(false), tag("%")),
            value(ModuleKind::Conjunction(HashMap::new()), tag("&")),
            value(ModuleKind::Broadcast, tag("")),
        ))(input)
    }

    fn targets(input: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(tag(", "), alpha1)(input)
    }

    fn module(input: &str) -> IResult<&str, (&str, Module)> {
        map(tuple((kind, name, targets)), |(kind, name, targets)| {
            (
                name,
                Module {
                    kind,
                    name,
                    targets,
                },
            )
        })(input)
    }

    type Memories<'a> = HashMap<&'a str, HashMap<&'a str, Pulse>>;
    type Modules<'a> = HashMap<&'a str, Module<'a>>;

    pub fn parse(input: &str) -> IResult<&str, (Memories, Modules)> {
        fold_many1(
            terminated(module, line_ending),
            || (HashMap::new(), HashMap::new()),
            |(mut names, mut modules), (name, module)| {
                for &target in &module.targets {
                    names
                        .entry(target)
                        .or_insert_with(HashMap::new)
                        .insert(name, Pulse::Low);
                }

                modules.insert(name, module);
                (names, modules)
            },
        )(input)
    }
}

pub fn process(input: &str) -> String {
    let (_, (memories, mut modules)) = parser::parse(input).expect("should parse");

    modules.iter_mut().for_each(|(name, module)| {
        if let ModuleKind::Conjunction(memory) = &mut module.kind {
            if let Some(new_memory) = memories.get(name) {
                *memory = new_memory.clone();
            }
        }
    });

    let mut low_pulses_count: usize = 0;
    let mut high_pulses_count: usize = 0;

    for _ in 0..1000 {
        low_pulses_count += 1;
        let mut stack = VecDeque::from([("button", Pulse::Low, "broadcaster")]);

        while let Some((from, pulse, to)) = stack.pop_front() {
            if let Some(to_module) = modules.get_mut(to) {
                if let Some((pulse, targets)) = to_module.output(pulse, from) {
                    match pulse {
                        Pulse::High => high_pulses_count += targets.len(),
                        Pulse::Low => low_pulses_count += targets.len(),
                    }

                    stack.extend(targets.into_iter().map(|new_to| (to, pulse, new_to)));
                }
            }
        }
    }

    (low_pulses_count * high_pulses_count).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let result = process(include_str!("../data/example1.txt"));
        assert_eq!(result, "32000000".to_string());
    }
    #[test]
    fn test_process2() {
        let result = process(include_str!("../data/example2.txt"));
        assert_eq!(result, "11687500".to_string());
    }
}
