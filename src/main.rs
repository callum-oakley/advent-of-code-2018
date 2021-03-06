use std::{env, fmt, process};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod de_list;
mod error;
mod point;

use crate::error::{bail, try_with, Result};

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
    let input07 = include_str!("input/day07");
    let input08 = include_str!("input/day08");
    let input10 = include_str!("input/day10");
    let input12 = include_str!("input/day12");
    let input13 = include_str!("input/day13");

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
        7 => Ok(Answer::new(
            day07::part1(input07)?,
            day07::part2(input07, 5, 60)?,
        )),
        8 => Ok(Answer::new(day08::part1(input08)?, day08::part2(input08)?)),
        9 => Ok(Answer::new(
            day09::part1(466, 71436)?,
            day09::part2(466, 71436)?,
        )),
        10 => Ok(Answer::new(day10::part1(input10, 10009)?, 10009)),
        11 => Ok(Answer::new(day11::part1(3613)?, day11::part2(3613)?)),
        12 => Ok(Answer::new(day12::part1(input12)?, 3_350_000_000_000i64)),
        13 => Ok(Answer::new(day13::part1(input13)?, day13::part2(input13)?)),
        14 => Ok(Answer::new(day14::part1(409_551)?, day14::part2("409551")?)),
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

    const FBHKLEAG: &'static str = "
        ######..#####...#....#..#....#..#.......######....##.....####.
        #.......#....#..#....#..#...#...#.......#........#..#...#....#
        #.......#....#..#....#..#..#....#.......#.......#....#..#.....
        #.......#....#..#....#..#.#.....#.......#.......#....#..#.....
        #####...#####...######..##......#.......#####...#....#..#.....
        #.......#....#..#....#..##......#.......#.......######..#..###
        #.......#....#..#....#..#.#.....#.......#.......#....#..#....#
        #.......#....#..#....#..#..#....#.......#.......#....#..#....#
        #.......#....#..#....#..#...#...#.......#.......#....#..#...##
        #.......#####...#....#..#....#..######..######..#....#...###.#
    ";

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
        assert_eq!(
            run_day(7),
            Ok(Answer::new("ABGKCMVWYDEHFOPQUILSTNZRJX", 898))
        );
        assert_eq!(run_day(8), Ok(Answer::new(42254, 25007)));
        assert_eq!(run_day(9), Ok(Answer::new(382055, 3_133_277_384i64)));
        assert_eq!(
            run_day(10),
            Ok(Answer::new(
                FBHKLEAG
                    .trim_end()
                    .lines()
                    .map(|l| format!("{}\n", l.trim_start()))
                    .collect::<String>(),
                10009
            ))
        );
        assert_eq!(run_day(11), Ok(Answer::new("20,54", "233,93,13")));
        assert_eq!(run_day(12), Ok(Answer::new("3405", 3_350_000_000_000i64)));
        assert_eq!(run_day(13), Ok(Answer::new("41,22", "84,90")));
        assert_eq!(run_day(14), Ok(Answer::new("1631191756", 20219475)));
    }
}
