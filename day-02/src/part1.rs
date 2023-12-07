use std::collections::HashMap;

pub fn process(input: &str) -> String {
    let color_max = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    input
        .lines()
        .filter_map(|line| {
            let (id, games) = line.split_once(": ")?;
            if games.split("; ").any(|game| {
                game.split(", ").any(|draw| {
                    let (num, color) = draw.split_once(' ').unwrap();
                    num.parse::<u32>().unwrap() > color_max[color]
                })
            }) {
                None
            } else {
                Some(id[5..].parse::<u32>().unwrap())
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
        assert_eq!(result, "8".to_string());
    }
}
