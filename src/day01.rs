use error::{parse, Result};
use std::collections::HashSet;

pub fn part1(input: &str) -> Result<i32> {
    Ok(parse_input(input)?.iter().sum())
}

pub fn part2(input: &str) -> Result<i32> {
    let mut frequency = 0;
    let mut seen = HashSet::new();
    seen.insert(frequency);

    for change in parse_input(input)?.iter().cycle() {
        frequency += change;
        if seen.contains(&frequency) {
            return Ok(frequency);
        }
        seen.insert(frequency);
    }
    unreachable!()
}

fn parse_input(input: &str) -> Result<Vec<i32>> {
    input.lines().map(|l| parse::<i32>(l)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        assert_eq!(part1("+1\n-2\n+3\n+1\n"), Ok(3));
        assert_eq!(part1("+1\n+1\n+1\n"), Ok(3));
        assert_eq!(part1("+1\n+1\n-2\n"), Ok(0));
        assert_eq!(part1("-1\n-2\n-3\n"), Ok(-6));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2("+1\n-2\n+3\n+1\n"), Ok(2));
        assert_eq!(part2("+1\n-1\n"), Ok(0));
        assert_eq!(part2("+3\n+3\n+4\n-2\n-4\n"), Ok(10));
        assert_eq!(part2("-6\n+3\n+8\n+5\n-6\n"), Ok(5));
        assert_eq!(part2("+7\n+7\n-2\n-7\n-4\n"), Ok(14));
    }
}
