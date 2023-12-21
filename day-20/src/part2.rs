use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use num::Integer;

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

/*
Writing this comment as I go, I think the problem may be similar to day 08:

- I ran this for a good long while (15ish minutes) and it never finished
- Looking at the numbers using 100,000,000 loops: 4-28 highs and 0 lows
- Apparently, I don't need to worry about multiple lows, just getting a single one would be nice

- I had no idea where to go next though, so I began to debug
- I created a HashSet to see what fed into rx: just ql
- ql is a conjunction, and based on the memory HashMap, it has 4 that feed into it: ss, fz, mf, fh
- I realized stupidly that I could probably just look at the input. It confirmed my findings

- For ql to output low, it needs to receive all highs.
- Interestingly ss, fz, mf, and fh are all conjunctions so they need to *not* receive all highs
- Also, each only has one item that feeds into it: lr, tf, hl, and sn respectively
- That means, ss, fz, mf, and fh just do the opposite of lr, tf, hl, and sn
- Thus, when ss, fz, mf, and fh all receive lows, we should get all highs, which is what ql needs

- Now, I decided to see how often ss spits out a low pulse
- I got 3881, 7762, 11643, 15524, 19405, 23286 ... all exactly 3881 apart
- For fz: 3793, 7586, 11379 ... all exactly 3793 apart
- For mf: 3761, 7522, 11283 ... all exactly 3761 apart
- For fh: 3847, 7694, 11541 ... all exactly 3847 apart
- Clearly, they loop exactly that many times, so it's probably LCM
- Just to see, I tried it with 212986464842911 (the LCM of those 4). It worked!

Please enjoy the stream of consciousness above.I wanted to make a solution that could calculate it
generally, but couldn't come up with a way. I think that's the point of today, so instead, I
created a solution that makes these assumptions:

- rx has exactly one conjunction pulsing into it
- The conjunction pulsing into rx has only conjunctions modules pulsing into it
- These conjunctions each only have one other module pulsing into them

I don't know if these assumptions hold true for all inputs, but they do hold true for mine!
*/
pub fn process(input: &str) -> String {
    let (_, (memories, mut modules)) = parser::parse(input).expect("should parse");

    // Insert memories into modules
    modules.iter_mut().for_each(|(name, module)| {
        if let ModuleKind::Conjunction(memory) = &mut module.kind {
            if let Some(new_memory) = memories.get(name) {
                *memory = new_memory.clone();
            }
        }
    });

    let final_module = "rx";

    // Get the conjunction modules pulsing towards the final module
    let final_conjunctions = modules
        .values()
        .filter(|m| m.targets.contains(&final_module))
        .collect_vec();

    // Ensure there is only one conjunction pulsing towards the final module
    assert!(
        final_conjunctions.len() == 1,
        "should be exactly one final conjunction"
    );

    // Extract the only final conjunction that pulses towards the final module
    let final_conjunction = final_conjunctions[0];

    // Extract the deciding conjunctions that pulse towards the final conjunction
    let deciding_conjunctions = modules
        .values()
        .filter(|module| module.targets.contains(&final_conjunction.name))
        .inspect(|module| {
            // Ensure the matching module is a conjunction
            assert!(
                matches!(module.kind, ModuleKind::Conjunction(_)),
                "should be a conjunction"
            );

            // Ensure the matching module has exactly one module pulsing towards it
            assert!(
                modules
                    .values()
                    .filter(|other| other.targets.contains(&module.name))
                    .at_most_one()
                    .is_ok(),
                "should have exactly one memory input"
            );
        })
        .map(|module| module.name)
        .collect_vec();

    // Set up variables to store the number of button presses
    let mut presses_until_low_pulse: HashMap<&str, usize> = HashMap::new();
    let mut button_press_count: usize = 0;

    // Loop until we've calculated the low pulse count for each deciding conjunction
    'outer: loop {
        button_press_count += 1;

        // Process the button press, adding to the stack as pulses distribute
        let mut stack = VecDeque::from([("button", Pulse::Low, "broadcaster")]);
        while let Some((from, pulse, to)) = stack.pop_front() {
            // Check to see if we've hit one of our deciding conjunctions with a low pulse
            if deciding_conjunctions.contains(&to) && pulse == Pulse::Low {
                presses_until_low_pulse
                    .entry(to)
                    .or_insert(button_press_count);

                // End once we've processed all deciding conjunctions
                if presses_until_low_pulse.len() == deciding_conjunctions.len() {
                    break 'outer;
                }
            }

            // Process the next pulses, adding them to the stack as needed
            if let Some(to_module) = modules.get_mut(to) {
                if let Some((pulse, targets)) = to_module.output(pulse, from) {
                    stack.extend(targets.into_iter().map(|new_to| (to, pulse, new_to)));
                }
            }
        }
    }

    // Calculate the lowest common multiple from our presses
    presses_until_low_pulse
        .into_values()
        .reduce(|acc, asdf| acc.lcm(&asdf))
        .expect("should resolve to the lowest common multiple")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/input.txt"));
        assert_eq!(result, "212986464842911".to_string());
    }
}
