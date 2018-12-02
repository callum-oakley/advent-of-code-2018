use error::{re, Result};
use std::collections::HashMap;

pub fn part1(input: &str) -> Result<usize> {
    let ids = parse_input(input)?;
    let contains2 = ids.iter().filter(|id| contains_repeats(2, id)).count();
    let contains3 = ids.iter().filter(|id| contains_repeats(3, id)).count();
    Ok(contains2 * contains3)
}

pub fn part2(input: &str) -> Result<String> {
    let ids = parse_input(input)?;

    for a in ids.iter() {
        for b in ids.iter() {
            let common = common_letters(a, b);
            if common.misses == 1 {
                return Ok(common.letters);
            }
        }
    }

    bail!("couldn't find the prototype boxes");
}

fn contains_repeats(n: i32, id: &str) -> bool {
    let mut frequencies = HashMap::new();
    for c in id.chars() {
        frequencies.entry(c).and_modify(|f| *f += 1).or_insert(1);
    }
    frequencies.values().any(|f| *f == n)
}

struct CommonLetters {
    letters: String,
    misses: i32,
}

fn common_letters(a: &str, b: &str) -> CommonLetters {
    let mut res = CommonLetters {
        letters: String::new(),
        misses: 0,
    };

    for (a, b) in a.chars().zip(b.chars()) {
        if a == b {
            res.letters.push(a);
        } else {
            res.misses += 1;
        }
    }

    res
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
