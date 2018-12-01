use error::{parse, re, Result};
use std::collections::HashSet;

pub fn part1(input: &str) -> Result<i32> {
    Ok(parse_input(input)?.iter().sum())
}

pub fn part2(input: &str) -> Result<i32> {
    let mut frequency = 0;
    let mut seen = HashSet::new();

    for change in parse_input(input)?.iter().cycle() {
        if !seen.insert(frequency) {
            break;
        }
        frequency += change;
    }

    Ok(frequency)
}

fn parse_input(input: &str) -> Result<Vec<i32>> {
    re(r"[+-]\d+")?
        .find_iter(input)
        .map(|s| parse::<i32>(s.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        assert_eq!(part1("+1, -2, +3, +1"), Ok(3));
        assert_eq!(part1("+1, +1, +1"), Ok(3));
        assert_eq!(part1("+1, +1, -2"), Ok(0));
        assert_eq!(part1("-1, -2, -3"), Ok(-6));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2("+1, -2, +3, +1"), Ok(2));
        assert_eq!(part2("+1, -1"), Ok(0));
        assert_eq!(part2("+3, +3, +4, -2, -4"), Ok(10));
        assert_eq!(part2("-6, +3, +8, +5, -6"), Ok(5));
        assert_eq!(part2("+7, +7, -2, -7, -4"), Ok(14));
    }
}
