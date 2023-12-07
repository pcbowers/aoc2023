use std::collections::HashMap;

pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (_, games) = line.split_once(": ").unwrap();
            let mut colors = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            games.split("; ").for_each(|game| {
                game.split(", ").for_each(|draw| {
                    let (num, color) = draw.split_once(' ').unwrap();
                    colors
                        .entry(color)
                        .and_modify(|max| *max = num.parse::<u32>().unwrap().max(*max));
                })
            });

            colors.values().product::<u32>()
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
        assert_eq!(result, "2286".to_string());
    }
}
