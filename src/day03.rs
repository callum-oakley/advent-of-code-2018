use error::{parse, re, Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Claim {
    id: i32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn squares(&self) -> Squares {
        Squares {
            claim: self.clone(),
            x: self.x,
            y: self.y,
        }
    }
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let caps = require_with!(
            re(r"#(\S+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)")?.captures(s),
            "couldn't parse claim {}",
            s,
        );

        Ok(Claim {
            id: parse(&caps[1])?,
            x: parse(&caps[2])?,
            y: parse(&caps[3])?,
            width: parse(&caps[4])?,
            height: parse(&caps[5])?,
        })
    }
}

struct Squares {
    claim: Claim,
    x: usize,
    y: usize,
}

impl Iterator for Squares {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let (x, y) = (self.x, self.y);
        if x >= self.claim.x + self.claim.width {
            return None;
        }

        self.y += 1;
        if self.y >= self.claim.y + self.claim.height {
            self.y = self.claim.y;
            self.x += 1;
        }

        Some((x, y))
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let claims = parse_input(input)?;

    let mut fabric = HashMap::new();

    for claim in claims.iter() {
        for square in claim.squares() {
            fabric.entry(square).and_modify(|n| *n += 1).or_insert(1);
        }
    }

    Ok(fabric.values().filter(|n| **n >= 2).count())
}

pub fn part2(input: &str) -> Result<i32> {
    let claims = parse_input(input)?;

    let mut fabric = HashMap::new();

    for claim in claims.iter() {
        for square in claim.squares() {
            fabric.entry(square).and_modify(|n| *n += 1).or_insert(1);
        }
    }

    Ok(require_with!(
        claims.iter().find(|c| c.squares().all(|s| fabric[&s] == 1)),
        "all claims overlap!"
    ).id)
}

fn parse_input(input: &str) -> Result<Vec<Claim>> {
    input.trim().lines().map(|l| parse(l)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
        #1 @ 1,3: 4x4
        #2 @ 3,1: 4x4
        #3 @ 5,5: 2x2
    ";

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE), Ok(4));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE), Ok(3));
    }
}
