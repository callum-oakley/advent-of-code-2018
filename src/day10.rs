use crate::error::{parse, re, require_with, Error, Result};
use crate::point::Point;
use std::str::FromStr;

struct Light {
    position: Point,
    velocity: Point,
}

impl Light {
    fn tick(&mut self) {
        self.position += self.velocity;
    }
}

impl FromStr for Light {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let caps = require_with!(
            re(r"position=<\s*(-?\d+),\s*(-?\d+)>\s*velocity=<\s*(-?\d+),\s*(-?\d+)>")?.captures(s),
            "couldn't parse light {}",
            s
        );

        Ok(Light {
            position: Point {
                x: parse(&caps[1])?,
                y: parse(&caps[2])?,
            },
            velocity: Point {
                x: parse(&caps[3])?,
                y: parse(&caps[4])?,
            },
        })
    }
}

struct Sky {
    lights: Vec<Light>,
}

impl ToString for Sky {
    fn to_string(&self) -> String {
        let xs = self.lights.iter().map(|l| l.position.x);
        let ys = self.lights.iter().map(|l| l.position.y);
        let min_x = xs.clone().min().unwrap();
        let max_x = xs.clone().max().unwrap();
        let min_y = ys.clone().min().unwrap();
        let max_y = ys.clone().max().unwrap();

        if max_x - min_x > 100 || max_y - min_y > 100 {
            return format!("({}x{})", max_x - min_x, max_y - min_y);
        }

        let mut s = "\n".to_string();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.lights.iter().any(|l| l.position == Point { x, y }) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }

        s
    }
}

pub fn part1(input: &str, seconds: i32) -> Result<String> {
    let mut sky = parse_input(input)?;

    for _second in 1..=seconds {
        for light in sky.lights.iter_mut() {
            light.tick();
        }
        // println!("After {} seconds:\n{}", second, sky.to_string());
    }

    Ok(sky.to_string())
}

fn parse_input(input: &str) -> Result<Sky> {
    let lights: Vec<_> = input
        .trim()
        .lines()
        .map(|l| parse::<Light>(l))
        .collect::<Result<_>>()?;

    Ok(Sky { lights })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
        position=< 9,  1> velocity=< 0,  2>
        position=< 7,  0> velocity=<-1,  0>
        position=< 3, -2> velocity=<-1,  1>
        position=< 6, 10> velocity=<-2, -1>
        position=< 2, -4> velocity=< 2,  2>
        position=<-6, 10> velocity=< 2, -2>
        position=< 1,  8> velocity=< 1, -1>
        position=< 1,  7> velocity=< 1,  0>
        position=<-3, 11> velocity=< 1, -2>
        position=< 7,  6> velocity=<-1, -1>
        position=<-2,  3> velocity=< 1,  0>
        position=<-4,  3> velocity=< 2,  0>
        position=<10, -3> velocity=<-1,  1>
        position=< 5, 11> velocity=< 1, -2>
        position=< 4,  7> velocity=< 0, -1>
        position=< 8, -2> velocity=< 0,  1>
        position=<15,  0> velocity=<-2,  0>
        position=< 1,  6> velocity=< 1,  0>
        position=< 8,  9> velocity=< 0, -1>
        position=< 3,  3> velocity=<-1,  1>
        position=< 0,  5> velocity=< 0, -1>
        position=<-2,  2> velocity=< 2,  0>
        position=< 5, -2> velocity=< 1,  2>
        position=< 1,  4> velocity=< 2,  1>
        position=<-2,  7> velocity=< 2, -2>
        position=< 3,  6> velocity=<-1, -1>
        position=< 5,  0> velocity=< 1,  0>
        position=<-6,  0> velocity=< 2,  0>
        position=< 5,  9> velocity=< 1, -2>
        position=<14,  7> velocity=<-2,  0>
        position=<-3,  6> velocity=< 2, -1>
    ";

    const HI: &'static str = "
        #...#..###
        #...#...#.
        #...#...#.
        #####...#.
        #...#...#.
        #...#...#.
        #...#...#.
        #...#..###
    ";

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1(SAMPLE, 3),
            Ok(HI
                .trim_end()
                .lines()
                .map(|l| format!("{}\n", l.trim_start()))
                .collect())
        )
    }
}
