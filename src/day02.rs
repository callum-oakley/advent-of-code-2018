use crate::error::{bail, re, Result};
use std::collections::HashMap;

pub fn part1(input: &str) -> Result<usize> {
    let ids = parse_input(input)?;

    let mut twos = 0;
    let mut threes = 0;

    for id in ids {
        let mut frequencies = HashMap::new();
        for c in id.chars() {
            frequencies.entry(c).and_modify(|f| *f += 1).or_insert(1);
        }
        if frequencies.values().any(|f| *f == 2) {
            twos += 1
        }
        if frequencies.values().any(|f| *f == 3) {
            threes += 1
        }
    }

    Ok(twos * threes)
}

pub fn part2(input: &str) -> Result<String> {
    let ids = parse_input(input)?;

    for (i, a) in ids.iter().enumerate() {
        for b in ids.iter().skip(i + 1) {
            let mut common = String::new();
            let mut misses = 0;

            for (a, b) in a.chars().zip(b.chars()) {
                if a == b {
                    common.push(a);
                } else {
                    misses += 1;
                    if misses > 1 {
                        break;
                    }
                }
            }

            if misses == 1 {
                return Ok(common);
            }
        }
    }

    bail!("couldn't find the prototype boxes");
}

fn parse_input(input: &str) -> Result<Vec<&str>> {
    Ok(re(r"[a-z]+")?
        .find_iter(input)
        .map(|m| m.as_str())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "abcdef, bababc, abbcde, abcccd, aabcdd, abcdee, ababab";

        assert_eq!(part1(sample), Ok(12));
    }

    #[test]
    fn sample_part2() {
        let sample = "abcde, fghij, klmno, pqrst, fguij, axcye, wvxyz";

        assert_eq!(part2(sample), Ok("fgij".to_string()))
    }
}
