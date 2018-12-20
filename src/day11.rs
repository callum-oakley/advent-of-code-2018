use crate::error::Result;
use std::collections::HashMap;

pub fn part1(serial: i32) -> Result<String> {
    let (x, y) = (1..=300)
        .flat_map(|x| (1..=300).map(move |y| (x, y)))
        .max_by_key(|(x, y)| {
            (0..3)
                .flat_map(|i| (0..3).map(move |j| power(x + i, y + j, serial)))
                .sum::<i32>()
        })
        .unwrap();
    Ok(format!("{},{}", x, y))
}

pub fn part2(serial: i32) -> Result<String> {
    let mut squares = HashMap::new();
    for (x, y) in (1..=300).flat_map(|x| (1..=300).map(move |y| (x, y))) {
        let mut size = 1;
        let mut p = 0;
        while x + size - 1 <= 300 && y + size - 1 <= 300 {
            p += power(x + size - 1, y + size - 1, serial);
            for i in 0..size - 1 {
                p += power(x + i, y + size - 1, serial) + power(x + size - 1, y + i, serial);
            }
            squares.insert((x, y, size), p);
            size += 1;
        }
    }
    let (x, y, size) = squares.keys().max_by_key(|k| squares[k]).unwrap();
    Ok(format!("{},{},{}", x, y, size))
}

fn power(x: i32, y: i32, serial: i32) -> i32 {
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
