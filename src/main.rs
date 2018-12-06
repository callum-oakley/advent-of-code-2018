#[macro_use]
extern crate simple_error;
extern crate regex;

use std::{env, fmt, process};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod error;
use error::Result;

#[derive(Debug, PartialEq)]
struct Answer {
    part1: String,
    part2: String,
}

impl Answer {
    fn new(part1: impl fmt::Display, part2: impl fmt::Display) -> Self {
        Answer {
            part1: part1.to_string(),
            part2: part2.to_string(),
        }
    }
}

fn run_day(day: u8) -> Result<Answer> {
    let input01 = include_str!("input/day01");
    let input02 = include_str!("input/day02");
    let input03 = include_str!("input/day03");
    let input04 = include_str!("input/day04");
    let input05 = include_str!("input/day05");
    let input06 = include_str!("input/day06");

    match day {
        1 => Ok(Answer::new(day01::part1(input01)?, day01::part2(input01)?)),
        2 => Ok(Answer::new(day02::part1(input02)?, day02::part2(input02)?)),
        3 => Ok(Answer::new(day03::part1(input03)?, day03::part2(input03)?)),
        4 => Ok(Answer::new(day04::part1(input04)?, day04::part2(input04)?)),
        5 => Ok(Answer::new(day05::part1(input05)?, day05::part2(input05)?)),
        6 => Ok(Answer::new(
            day06::part1(input06)?,
            day06::part2(input06, 10000)?,
        )),
        day => bail!("day {} not yet implemented", day),
    }
}

fn run() -> Result<Answer> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        bail!("expected exactly one argument");
    }

    run_day(try_with!(
        args.first().unwrap().parse(),
        "failed to parse day"
    ))
}

fn main() {
    process::exit(match run() {
        Ok(answer) => {
            println!("part1: {}", answer.part1);
            println!("part2: {}", answer.part2);
            0
        }
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_days() {
        assert_eq!(run_day(1), Ok(Answer::new(470, 790)));
        assert_eq!(
            run_day(2),
            Ok(Answer::new(6175, "asgwjcmzredihqoutcylvzinx"))
        );
        assert_eq!(run_day(3), Ok(Answer::new(106501, 632)));
        assert_eq!(run_day(4), Ok(Answer::new(84834, 53427)));
        assert_eq!(run_day(5), Ok(Answer::new(11310, 6020)));
        assert_eq!(run_day(6), Ok(Answer::new(3449, 44868)));
    }
}
