use std::{collections::HashMap, ops::Range};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Condition {
    Less,
    Greater,
    Equal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rating {
    category: Category,
    condition: Condition,
    value: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Destination<'a> {
    Accepted,
    Rejected,
    Workflow(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule<'a> {
    Test(Rating, Destination<'a>),
    Default(Destination<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    ratings: HashMap<Category, Rating>,
}

mod parser {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{self, alpha1, line_ending},
        combinator::{map, opt, value},
        multi::{fold_many1, many1, separated_list1},
        sequence::{delimited, separated_pair, terminated, tuple},
        IResult, Parser,
    };
    use std::collections::HashMap;

    fn condition(input: &str) -> IResult<&str, Condition> {
        alt((
            value(Condition::Less, tag("<")),
            value(Condition::Greater, tag(">")),
            value(Condition::Equal, tag("=")),
        ))(input)
    }

    fn category(input: &str) -> IResult<&str, Category> {
        alt((
            value(Category::X, tag("x")),
            value(Category::M, tag("m")),
            value(Category::A, tag("a")),
            value(Category::S, tag("s")),
        ))(input)
    }

    fn rating(input: &str) -> IResult<&str, Rating> {
        map(
            tuple((category, condition, complete::u64.map(|n| n as usize))),
            |(category, condition, value)| Rating {
                category,
                condition,
                value,
            },
        )(input)
    }

    fn ratings(input: &str) -> IResult<&str, HashMap<Category, Rating>> {
        fold_many1(
            terminated(rating, opt(tag(","))),
            HashMap::new,
            |mut ratings, rating| {
                ratings.insert(rating.category, rating);
                ratings
            },
        )(input)
    }

    fn destination(input: &str) -> IResult<&str, Destination> {
        alt((
            value(Destination::Accepted, tag("A")),
            value(Destination::Rejected, tag("R")),
            alpha1.map(Destination::Workflow),
        ))(input)
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        alt((
            separated_pair(rating, tag(":"), destination).map(|(r, d)| Rule::Test(r, d)),
            destination.map(Rule::Default),
        ))(input)
    }

    fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
        separated_list1(tag(","), rule)(input)
    }

    fn workflow(input: &str) -> IResult<&str, (&str, Workflow)> {
        map(
            tuple((alpha1, delimited(tag("{"), rules, tag("}")))),
            |(name, rules)| (name, Workflow { name, rules }),
        )(input)
    }

    fn workflows(input: &str) -> IResult<&str, HashMap<&str, Workflow>> {
        fold_many1(
            terminated(workflow, line_ending),
            HashMap::new,
            |mut workflows, (name, workflow)| {
                workflows.insert(name, workflow);
                workflows
            },
        )(input)
    }

    fn part(input: &str) -> IResult<&str, Part> {
        map(delimited(tag("{"), ratings, tag("}")), |ratings| Part {
            ratings,
        })(input)
    }

    fn parts(input: &str) -> IResult<&str, Vec<Part>> {
        separated_list1(line_ending, part)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, (HashMap<&str, Workflow>, Vec<Part>)> {
        separated_pair(workflows, many1(line_ending), parts)(input)
    }
}

pub fn process(input: &str) -> String {
    let (_, (workflows, _)) = parser::parse(input).expect("Input must be parsable");

    calculate_accepted_ranges(
        "in",
        &workflows,
        HashMap::from([
            (Category::X, 1_usize..4001),
            (Category::M, 1_usize..4001),
            (Category::A, 1_usize..4001),
            (Category::S, 1_usize..4001),
        ]),
    )
    .iter()
    .map(|ranges| ranges.values().map(|r| r.end - r.start).product::<usize>())
    .sum::<usize>()
    .to_string()
}

fn calculate_accepted_ranges(
    workflow_name: &str,
    workflows: &HashMap<&str, Workflow>,
    ranges: HashMap<Category, Range<usize>>,
) -> Vec<HashMap<Category, Range<usize>>> {
    let mut final_ranges = Vec::new();

    let Some(workflow) = workflows.get(workflow_name) else {
        unreachable!("The '{workflow_name}' Workflow is not recognized");
    };

    // This stack ensures we don't calculate ranges that have already been processed
    let mut stack = Vec::from([ranges.clone()]);

    for rule in workflow.rules.iter() {
        // If the stack is empty, it means there are no more paths left to try, so we can end early
        let Some(curr_ranges) = stack.pop() else {
            break;
        };

        // get_possible_paths can return max 2 paths: one for a failed test case, one for a successful one
        // Often, it will only return 1 path when it's a default rule or the ranges don't satisfy both
        for (new_ranges, possible_destination) in get_possible_paths(rule, curr_ranges) {
            // If the rule failed, we should use the new range when running calculations for the next rule
            let Some(destination) = possible_destination else {
                stack.push(new_ranges);
                continue;
            };

            match destination {
                Destination::Rejected => (),
                Destination::Accepted => final_ranges.push(new_ranges),
                Destination::Workflow(new_workflow_name) => final_ranges.extend(
                    calculate_accepted_ranges(new_workflow_name, workflows, new_ranges),
                ),
            };
        }
    }

    final_ranges
}

#[allow(clippy::type_complexity)]
fn get_possible_paths<'a>(
    rule: &'a Rule,
    ranges: HashMap<Category, Range<usize>>,
) -> Vec<(HashMap<Category, Range<usize>>, Option<&'a Destination<'a>>)> {
    use Condition::*;

    match rule {
        Rule::Default(destination) => Vec::from([(ranges, Some(destination))]),
        Rule::Test(rating, destination) => {
            let Some(range) = ranges.get(&rating.category) else {
                unreachable!("The '{:?}' Category is not recognized", rating.category)
            };

            let mut paths = Vec::new();

            if let Some(true_range) = match rating.condition {
                Less if range.start < rating.value => Some(range.start..rating.value),
                Greater if range.end - 1 > rating.value => Some((rating.value + 1)..range.end),
                Less | Greater => None,
                Equal => unreachable!("The 'Equal' Condition should not exist on a Rule Rating"),
            } {
                let mut new_ranges = ranges.clone();
                new_ranges.insert(rating.category, true_range);
                paths.push((new_ranges, Some(destination)))
            }

            if let Some(false_range) = match rating.condition {
                Less if range.end > rating.value => Some(rating.value..range.end),
                Greater if range.start <= rating.value => Some(range.start..(rating.value + 1)),
                Less | Greater => None,
                Equal => unreachable!("The 'Equal' Condition should not exist on a Rule Rating"),
            } {
                let mut new_ranges = ranges.clone();
                new_ranges.insert(rating.category, false_range);
                paths.push((new_ranges, None))
            }

            paths
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "167409079868000".to_string());
    }
}
