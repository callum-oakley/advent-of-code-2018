use error::{parse, re, Result};

const FABRIC_W: usize = 1000;
const FABRIC_H: usize = 1000;

#[derive(Debug, Clone)]
struct Claim {
    id: i32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Claim {
    fn new(s: &str) -> Result<Self> {
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

    fn squares(&self) -> Squares {
        Squares {
            claim: self.clone(),
            x: self.x,
            y: self.y,
        }
    }
}

struct Squares {
    claim: Claim,
    x: usize,
    y: usize,
}

// TODO fabric struct to abstract the (x, y) access

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

pub fn part1(input: &str) -> Result<usize> {
    let claims = input
        .trim()
        .lines()
        .map(|l| Claim::new(l))
        .collect::<Result<Vec<_>>>()?;

    let mut fabric = vec![0; FABRIC_W * FABRIC_H];

    for claim in claims.iter() {
        for (x, y) in claim.squares() {
            fabric[x + FABRIC_W * y] += 1;
        }
    }

    Ok(fabric.into_iter().filter(|n| *n >= 2).count())
}

pub fn part2(input: &str) -> Result<i32> {
    let claims = input
        .trim()
        .lines()
        .map(|l| Claim::new(l))
        .collect::<Result<Vec<_>>>()?;

    let mut fabric = vec![0; FABRIC_W * FABRIC_H];

    for claim in claims.iter() {
        for (x, y) in claim.squares() {
            fabric[x + FABRIC_W * y] += 1;
        }
    }

    Ok(require_with!(
        claims
            .iter()
            .find(|c| c.squares().all(|(x, y)| fabric[x + FABRIC_W * y] == 1)),
        "all claims overlap!"
    ).id)
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
