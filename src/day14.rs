use crate::error::Result;

pub fn part1(input: usize) -> Result<String> {
    let (mut scores, mut i, mut j) = (vec![3, 7], 0, 1);

    while scores.len() < input + 10 {
        step(&mut scores, &mut i, &mut j);
    }

    Ok(display(&scores[input..input + 10]))
}

pub fn part2(input: &str) -> Result<usize> {
    let (mut scores, mut i, mut j) = (vec![3, 7], 0, 1);

    while scores.len() < input.len() + 1
        || !display(&scores[scores.len() - input.len() - 1..scores.len()]).contains(input)
    {
        step(&mut scores, &mut i, &mut j);
    }

    if display(&scores[scores.len() - input.len()..scores.len()]) == input {
        Ok(scores.len() - input.len())
    } else {
        Ok(scores.len() - input.len() - 1)
    }
}

fn step(scores: &mut Vec<usize>, i: &mut usize, j: &mut usize) {
    let sum = scores[*i] + scores[*j];

    if sum < 10 {
        scores.push(sum);
    } else {
        scores.append(&mut vec![sum / 10, sum % 10]);
    }

    *i = (*i + 1 + scores[*i]) % scores.len();
    *j = (*j + 1 + scores[*j]) % scores.len();
}

fn display(v: &[usize]) -> String {
    v.iter().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        assert_eq!(part1(9), Ok("5158916779".to_string()));
        assert_eq!(part1(5), Ok("0124515891".to_string()));
        assert_eq!(part1(18), Ok("9251071085".to_string()));
        assert_eq!(part1(2018), Ok("5941429882".to_string()));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2("51589"), Ok(9));
        assert_eq!(part2("01245"), Ok(5));
        assert_eq!(part2("92510"), Ok(18));
        assert_eq!(part2("59414"), Ok(2018));
    }
}
