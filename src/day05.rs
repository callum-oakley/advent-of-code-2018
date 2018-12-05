use error::Result;

pub fn part1(input: &str) -> Result<usize> {
    let mut chars = input.trim().as_bytes().iter().collect();
    reduce(&mut chars);
    Ok(chars.len())
}

pub fn part2(input: &str) -> Result<usize> {
    Ok((b'a'..=b'z')
        .map(|unit| {
            let mut chars = input
                .trim()
                .as_bytes()
                .iter()
                .filter(|c| !unit.eq_ignore_ascii_case(c))
                .collect();
            reduce(&mut chars);
            chars.len()
        }).min()
        .unwrap())
}

fn reduce(chars: &mut Vec<&u8>) {
    let mut i = 0;
    while i + 1 < chars.len() {
        if reacts(*chars[i], *chars[i + 1]) {
            chars.remove(i);
            chars.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
}

fn reacts(x: u8, y: u8) -> bool {
    (x.is_ascii_lowercase() && y.is_ascii_uppercase()
        || x.is_ascii_uppercase() && y.is_ascii_lowercase())
        && x.eq_ignore_ascii_case(&y)
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
