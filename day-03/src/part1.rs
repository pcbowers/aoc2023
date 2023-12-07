use regex::Regex;

pub fn process(input: &str) -> String {
    let mut prev_lines = [""].iter().cloned().chain(input.lines());
    let mut next_lines = input.lines().skip(1).chain([""]);

    input
        .lines()
        .map(|curr_line| {
            let prev_line = prev_lines.next().unwrap();
            let next_line = next_lines.next().unwrap();
            Regex::new(r"\d+")
                .unwrap()
                .find_iter(curr_line)
                .map(|m| (m.range(), m.as_str().parse::<u32>().unwrap()))
                .map(|(range, possible_engine_num)| {
                    let extended_range = std::ops::Range {
                        start: range.start.saturating_sub(1),
                        end: (range.end + 1).min(curr_line.len()),
                    };

                    match (
                        prev_line
                            .get(extended_range.clone())
                            .unwrap_or(".")
                            .chars()
                            .any(|c| c != '.'),
                        next_line
                            .get(extended_range.clone())
                            .unwrap_or(".")
                            .chars()
                            .any(|c| c != '.'),
                        extended_range.start != range.start
                            && curr_line
                                .chars()
                                .nth(extended_range.start)
                                .map(|c| c != '.')
                                .unwrap_or(false),
                        extended_range.end != range.end
                            && curr_line
                                .chars()
                                .nth(extended_range.end - 1)
                                .map(|c| c != '.')
                                .unwrap_or(false),
                    ) {
                        (false, false, false, false) => 0,
                        _ => possible_engine_num,
                    }
                })
                .sum::<u32>()
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
        assert_eq!(result, "4361".to_string());
    }
}
