use crate::error::{re, require_with, Result};
use std::collections::HashSet;

type State = HashSet<i64>;
type Rule = (bool, bool, bool, bool, bool);
type Rules = HashSet<Rule>;

pub fn part1(input: &str) -> Result<i64> {
    run(input, 20)
}

// Part 2 by inspection. After a while the sum stabalises as 67 * generation, so the final answer
// is 67 * 50000000000 = 3350000000000

fn run(input: &str, generations: i64) -> Result<i64> {
    let (mut state, rules) = parse_input(input)?;

    for _generation in 1..=generations {
        let mut next_state = HashSet::new();
        for i in (state.iter().min().unwrap() - 2)..=(state.iter().max().unwrap() + 2) {
            if rules.contains(&(
                state.contains(&(i - 2)),
                state.contains(&(i - 1)),
                state.contains(&(i)),
                state.contains(&(i + 1)),
                state.contains(&(i + 2)),
            )) {
                next_state.insert(i);
            }
        }
        state = next_state;
    }

    Ok(state.iter().sum())
}

fn parse_input(input: &str) -> Result<(State, Rules)> {
    let mut lines = input.trim().lines();

    let initial_state = parse_initial_state(lines.next().unwrap())?;
    let rules = lines.filter_map(|l| parse_rule(l)).collect();

    Ok((initial_state, rules))
}

fn parse_initial_state(s: &str) -> Result<State> {
    let mut state = HashSet::new();

    let caps = require_with!(
        re(r"initial state: ([#.]*)")?.captures(s),
        "couldn't parse initial state {}",
        s,
    );

    for (i, c) in caps[1].chars().enumerate() {
        if c == '#' {
            state.insert(i as i64);
        }
    }

    Ok(state)
}

fn parse_rule(s: &str) -> Option<Rule> {
    if let Ok(r) = re(r"([#.])([#.])([#.])([#.])([#.]) => #") {
        if let Some(caps) = r.captures(s) {
            Some((
                &caps[1] == "#",
                &caps[2] == "#",
                &caps[3] == "#",
                &caps[4] == "#",
                &caps[5] == "#",
            ))
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
        initial state: #..#.#..##......###...###

        ...## => #
        ..#.. => #
        .#... => #
        .#.#. => #
        .#.## => #
        .##.. => #
        .#### => #
        #.#.# => #
        #.### => #
        ##.#. => #
        ##.## => #
        ###.. => #
        ###.# => #
        ####. => #
    ";

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE), Ok(325));
    }
}
