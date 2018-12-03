use error::{parse, re, Error, Result};
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Claim {
    id: i32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
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
            w: parse(&caps[4])?,
            h: parse(&caps[5])?,
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
        if x >= self.claim.x + self.claim.w {
            return None;
        }

        self.y += 1;
        if self.y >= self.claim.y + self.claim.h {
            self.y = self.claim.y;
            self.x += 1;
        }

        Some((x, y))
    }
}

struct Fabric {
    width: usize,
    height: usize,
    v: Vec<i32>,
}

impl Fabric {
    fn new(width: usize, height: usize) -> Self {
        Fabric {
            width,
            height,
            v: vec![0; width * height],
        }
    }
}

impl Index<(usize, usize)> for Fabric {
    type Output = i32;

    fn index(&self, (x, y): (usize, usize)) -> &i32 {
        if x >= self.width || y >= self.height {
            panic!("({}, {}) out of bounds", x, y);
        }
        &self.v[x + self.width * y]
    }
}

impl IndexMut<(usize, usize)> for Fabric {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut i32 {
        if x >= self.width || y >= self.height {
            panic!("({}, {}) out of bounds", x, y);
        }
        &mut self.v[x + self.width * y]
    }
}

impl IntoIterator for Fabric {
    type Item = i32;
    type IntoIter = ::std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let claims = parse_input(input)?;

    let mut fabric = Fabric::new(1000, 1000);

    for claim in claims.iter() {
        for square in claim.squares() {
            fabric[square] += 1;
        }
    }

    Ok(fabric.into_iter().filter(|n| *n >= 2).count())
}

pub fn part2(input: &str) -> Result<i32> {
    let claims = parse_input(input)?;

    let mut fabric = Fabric::new(1000, 1000);

    for claim in claims.iter() {
        for square in claim.squares() {
            fabric[square] += 1;
        }
    }

    Ok(require_with!(
        claims.iter().find(|c| c.squares().all(|s| fabric[s] == 1)),
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
