use error::Result;

pub fn part1(input: &str) -> Result<usize> {
    Ok(reduce(input.trim().as_bytes().iter()).len())
}

pub fn part2(input: &str) -> Result<usize> {
    Ok((b'a'..=b'z')
        .map(|unit| {
            reduce(
                input
                    .trim()
                    .as_bytes()
                    .iter()
                    .filter(|c| !unit.eq_ignore_ascii_case(c)),
            ).len()
        }).min()
        .unwrap())
}

fn reduce<'a>(chars: impl Iterator<Item = &'a u8>) -> Vec<&'a u8> {
    let mut res = Vec::new();
    for x in chars {
        match res.pop() {
            Some(y) => if !reacts(x, y) {
                res.push(y);
                res.push(x);
            },
            None => res.push(x),
        }
    }
    res
}

fn reacts(x: &u8, y: &u8) -> bool {
    *x != *y && x.eq_ignore_ascii_case(&y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        assert_eq!(part1("dabAcCaCBAcCcaDA"), Ok(10));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2("dabAcCaCBAcCcaDA"), Ok(4));
    }
}
