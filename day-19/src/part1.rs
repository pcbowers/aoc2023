use std::collections::HashMap;

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
    let (_, (workflows, parts)) = parser::parse(input).expect("Input must be parsable");

    parts
        .iter()
        .filter(|part| calculate_part_acceptance("in", &workflows, part))
        .map(|part| part.ratings.values().map(|r| r.value).sum::<usize>())
        .sum::<usize>()
        .to_string()
}

fn calculate_part_acceptance<'a>(
    workflow_name: &'a str,
    workflows: &'a HashMap<&'a str, Workflow<'a>>,
    part: &'a Part,
) -> bool {
    let Some(workflow) = workflows.get(workflow_name) else {
        unreachable!("The '{workflow_name}' Workflow is not recognized");
    };

    for rule in workflow.rules.iter() {
        let Some(destination) = (match rule {
            Rule::Test(rating, destination) => process_rating(rating, part, destination),
            Rule::Default(destination) => Some(destination),
        }) else {
            continue;
        };

        return match destination {
            Destination::Accepted => true,
            Destination::Rejected => false,
            Destination::Workflow(new_workflow_name) => {
                calculate_part_acceptance(new_workflow_name, workflows, part)
            }
        };
    }

    unreachable!("The Part must be accepted or rejected")
}

fn process_rating<'a>(
    rating: &Rating,
    part: &Part,
    destination: &'a Destination<'a>,
) -> Option<&'a Destination<'a>> {
    let Some(part_rating) = part.ratings.get(&rating.category) else {
        unreachable!("The '{:?}' Category is not recognized", rating.category)
    };

    if match rating.condition {
        Condition::Less => part_rating.value < rating.value,
        Condition::Greater => part_rating.value > rating.value,
        Condition::Equal => unreachable!("The 'Equal' Condition should not exist on a Rule Rating"),
    } {
        Some(destination)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "19114".to_string());
    }
}
