use cached::proc_macro::cached;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!("Must match one of these characters"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DamagedGroup {
    Exhausted,
    Overflowed,
    NotStarted,
    NotFilled,
    Filled,
}

impl DamagedGroup {
    pub fn current_state(count: usize, groups: &[usize]) -> Self {
        match (count, groups.len(), Some(&count) == groups.first()) {
            // No count with no groups means exhausted.
            (0, 0, _) => DamagedGroup::Exhausted,
            // Some count with no groups means overflowed.
            (_, 0, _) => DamagedGroup::Overflowed,
            // No count with some groups means not started.
            (0, _, _) => DamagedGroup::NotStarted,
            // Some count with an unmatched group means not filled.
            (_, _, false) => DamagedGroup::NotFilled,
            // Some count with 1 group and a matched group means exhausted.
            // This check is necessary to ensure no false negatives.
            // This ensures Exhausted can match regardless of whether or not it just happened.
            (_, 1, true) => DamagedGroup::Exhausted,
            // Some count with a matched group means filled.
            // Because of the Exhausted check above, this only means the current group is filled.
            // It does not mean that we've reached the end of the groups like Exhausted does.
            (_, _, true) => DamagedGroup::Filled,
        }
    }
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .progress_count(input.lines().count() as u64)
        .par_bridge()
        .filter_map(|line| {
            line.split_once(' ').map(|(springs_line, groups_line)| {
                (
                    springs_line.chars().map(Spring::from).collect_vec(),
                    groups_line
                        .split(',')
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect_vec(),
                )
            })
        })
        .map(|(springs, groups)| arrangements(&springs, &groups, 0))
        .sum::<usize>()
        .to_string()
}

#[cached(
    key = "(Vec<Spring>, Vec<usize>, usize)",
    convert = r#"{ (springs.to_vec(), groups.to_vec(), count) }"#
)]
fn arrangements(springs: &[Spring], groups: &[usize], count: usize) -> usize {
    use DamagedGroup::*;
    use Spring::*;

    let current_state = DamagedGroup::current_state(count, groups);

    if springs.is_empty() {
        // We're at the end! Let's make sure all groups are exhausted.
        // If they're not exhausted, they're not possible.
        return match current_state {
            Exhausted => 1,
            _ => 0,
        };
    }

    // Sometimes, we'll overflow into another group when there isn't one.
    // Since this leads to an impossible arrangement, let's not count it.
    if current_state == Overflowed {
        return 0;
    }

    // Sometimes, we'll have more damaged nodes left than we do possible groups.
    // Since this leads to an impossible arrangement, let's not count it.
    if springs.iter().filter(|c| c == &&Damaged).count() > groups.iter().sum() {
        return 0;
    }

    // We can safely unwrap since we've already done a check for `is_empty`
    match (springs.iter().next().unwrap(), current_state) {
        (Operational, Exhausted) => arrangements(&springs[1..], groups, count),
        (Operational, NotStarted) => arrangements(&springs[1..], groups, count),
        (Operational, NotFilled) => 0,
        (Operational, Filled) => arrangements(&springs[1..], &groups[1..], 0),
        (Damaged, Exhausted) => 0,
        (Damaged, NotStarted) => arrangements(&springs[1..], groups, count + 1),
        (Damaged, NotFilled) => arrangements(&springs[1..], groups, count + 1),
        (Damaged, Filled) => 0,
        (Unknown, Exhausted) => arrangements(&springs[1..], groups, count),
        (Unknown, NotStarted) => {
            arrangements(&springs[1..], groups, count + 1)
                + arrangements(&springs[1..], groups, count)
        }
        (Unknown, NotFilled) => arrangements(&springs[1..], groups, count + 1),
        (Unknown, Filled) => arrangements(&springs[1..], &groups[1..], 0),
        _ => unreachable!("Must match one of these state combinations"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "21".to_string());
    }
}
