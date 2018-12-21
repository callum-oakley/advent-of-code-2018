use crate::error::Result;
use std::cmp::max;

pub fn part1(serial: i32) -> Result<String> {
    let (x, y) = (1..=300usize)
        .flat_map(|x| (1..=300usize).map(move |y| (x, y)))
        .max_by_key(|(x, y)| {
            (0..3)
                .flat_map(|i| (0..3usize).map(move |j| power(x + i, y + j, serial)))
                .sum::<i32>()
        })
        .unwrap();
    Ok(format!("{},{}", x, y))
}

pub fn part2(serial: i32) -> Result<String> {
    // https://en.wikipedia.org/wiki/Summed-area_table
    let mut sat = vec![0; 301 * 301];
    for (x, y) in (1..=300).flat_map(|x| (1..=300).map(move |y| (x, y))) {
        sat[x + y * 301] = power(x, y, serial) + sat[x + (y - 1) * 301] + sat[x - 1 + y * 301]
            - sat[x - 1 + (y - 1) * 301];
    }

    let (x, y, size) = (1..=300usize)
        .flat_map(|x| (1..=300usize).map(move |y| (x, y)))
        .flat_map(|(x, y)| (1..=301usize - max(x, y)).map(move |size| (x, y, size)))
        .max_by_key(|(x, y, size)| {
            sat[x + size - 1 + (y + size - 1) * 301] + sat[x - 1 + (y - 1) * 301]
                - sat[x - 1 + (y + size - 1) * 301]
                - sat[x + size - 1 + (y - 1) * 301]
        })
        .unwrap();

    Ok(format!("{},{},{}", x, y, size))
}

fn power(x: usize, y: usize, serial: i32) -> i32 {
    let x = x as i32;
    let y = y as i32;
    ((x + 10) * y + serial) * (x + 10) / 100 % 10 - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_level() {
        assert_eq!(power(3, 5, 8), 4);
        assert_eq!(power(122, 79, 57), -5);
        assert_eq!(power(217, 196, 39), 0);
        assert_eq!(power(101, 153, 71), 4);
    }

    #[test]
    fn sample_part1() {
        assert_eq!(part1(18), Ok("33,45".to_string()));
        assert_eq!(part1(42), Ok("21,61".to_string()));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(18), Ok("90,269,16".to_string()));
        assert_eq!(part2(42), Ok("232,251,12".to_string()));
    }
}
