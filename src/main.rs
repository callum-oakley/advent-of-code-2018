#[macro_use]
extern crate simple_error;
extern crate regex;

use std::{env, fmt, process};

mod day01;
mod day02;
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

    match day {
        1 => Ok(Answer::new(day01::part1(input01)?, day01::part2(input01)?)),
        2 => Ok(Answer::new(day02::part1(input02)?, day02::part2(input02)?)),
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
    }
}
