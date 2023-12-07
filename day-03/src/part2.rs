use regex::Regex;
use std::collections::HashMap;

pub fn process(input: &str) -> String {
    let mut engine_parts: HashMap<String, Vec<u32>> = HashMap::new();
    let mut prev_lines = [""].iter().cloned().chain(input.lines());
    let mut next_lines = input.lines().skip(1).chain([""]);

    input.lines().for_each(|curr_line| {
        let prev_line = prev_lines.next().unwrap();
        let next_line = next_lines.next().unwrap();
        Regex::new(r"\d+")
            .unwrap()
            .find_iter(curr_line)
            .map(|m| (m.range(), m.as_str().parse::<u32>().unwrap()))
            .for_each(|(range, engine_num)| {
                let extended_range = std::ops::Range {
                    start: range.start.saturating_sub(1),
                    end: (range.end + 1).min(curr_line.len()),
                };

                prev_line
                    .get(extended_range.clone())
                    .unwrap_or(".")
                    .chars()
                    .enumerate()
                    .filter_map(|(index, c)| {
                        if c.eq(&'*') {
                            Some(index + extended_range.start)
                        } else {
                            None
                        }
                    })
                    .for_each(|possible_engine_part| {
                        engine_parts
                            .entry(format!("{prev_line} {possible_engine_part}"))
                            .and_modify(|arr| arr.push(engine_num))
                            .or_insert(Vec::from([engine_num]));
                    });

                next_line
                    .get(extended_range.clone())
                    .unwrap_or(".")
                    .chars()
                    .enumerate()
                    .filter_map(|(index, c)| {
                        if c.eq(&'*') {
                            Some(index + extended_range.start)
                        } else {
                            None
                        }
                    })
                    .for_each(|possible_engine_part| {
                        engine_parts
                            .entry(format!("{next_line} {possible_engine_part}"))
                            .and_modify(|arr| arr.push(engine_num))
                            .or_insert(Vec::from([engine_num]));
                    });

                if curr_line
                    .chars()
                    .nth(extended_range.start)
                    .filter(|c| extended_range.start != range.start && c == &'*')
                    .is_some()
                {
                    let possible_engine_part = extended_range.start;
                    engine_parts
                        .entry(format!("{curr_line} {possible_engine_part}"))
                        .and_modify(|arr| arr.push(engine_num))
                        .or_insert(Vec::from([engine_num]));
                }

                if curr_line
                    .chars()
                    .nth(extended_range.end - 1)
                    .filter(|c| c == &'*' && extended_range.end != range.end)
                    .is_some()
                {
                    let possible_engine_part = extended_range.end - 1;
                    engine_parts
                        .entry(format!("{curr_line} {possible_engine_part}"))
                        .and_modify(|arr| arr.push(engine_num))
                        .or_insert(Vec::from([engine_num]));
                }
            })
    });

    engine_parts
        .iter()
        .filter_map(|(_, engine_parts)| {
            if engine_parts.len() == 2 {
                Some(engine_parts.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(include_str!("../data/example.txt"));
        assert_eq!(result, "467835".to_string());
    }
}
