use crate::error::{parse, re, require_with, Result};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> Result<i32> {
    let points = parse_input(input)?;

    let min_x = *points.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *points.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();

    let mut areas = HashMap::new();
    for &(x, y) in points.iter() {
        areas.insert((x, y), 0);
    }

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if let Some(p) = unique_closest_point(&points, (x, y)) {
                // The points with inifinite closest areas are the points that are closest to the
                // boundary.
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    areas.remove(p);
                } else {
                    areas.entry(*p).and_modify(|n| *n += 1);
                }
            }
        }
    }

    Ok(*areas.values().max().unwrap())
}

pub fn part2(input: &str, total_distance: i32) -> Result<usize> {
    let points = parse_input(input)?;

    // It is feasible that we would need to check a larger area than this, but it turns out this
    // isn't required for the correct solution.
    let min_x = *points.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *points.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();

    Ok((min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).map(move |y| (x, y)))
        .filter(|q| points.iter().map(|p| distance(*p, *q)).sum::<i32>() < total_distance)
        .count())
}

fn distance(p: (i32, i32), q: (i32, i32)) -> i32 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn unique_closest_point<'a>(
    points: &'a HashSet<(i32, i32)>,
    q: (i32, i32),
) -> Option<&'a (i32, i32)> {
    let (min, unique) = points.iter().fold((None, true), |(min, unique), p| {
        if let Some((_, min_dist)) = min {
            let dist = distance(*p, q);
            match dist.cmp(&min_dist) {
                Ordering::Less => (Some((p, dist)), true),
                Ordering::Equal => (min, false),
                Ordering::Greater => (min, unique),
            }
        } else {
            (Some((p, distance(*p, q))), unique)
        }
    });

    if unique {
        min.map(|(p, _)| p)
    } else {
        None
    }
}

fn parse_input(input: &str) -> Result<HashSet<(i32, i32)>> {
    input
        .trim()
        .lines()
        .map(|l| {
            let caps = require_with!(
                re(r"(\d+),\s(\d+)")?.captures(l),
                "couldn't parse coordinate {}",
                l,
            );
            Ok((parse(&caps[1])?, parse(&caps[2])?))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
        1, 1
        1, 6
        8, 3
        3, 4
        5, 5
        8, 9
    ";

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE), Ok(17));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE, 32), Ok(16));
    }
}
