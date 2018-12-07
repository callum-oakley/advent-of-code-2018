use crate::error::{parse, re, require_with, Result};
use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> Result<String> {
    work(input, 1, 0).map(|(s, _)| s)
}

pub fn part2(input: &str, workers: u8, base_seconds: u32) -> Result<u32> {
    work(input, workers, base_seconds).map(|(_, t)| t)
}

struct Job {
    step: char,
    time_remaining: u32,
}

fn work(input: &str, workers: u8, base_seconds: u32) -> Result<(String, u32)> {
    let Project {
        mut ready,
        blocks,
        mut blocked_by,
    } = parse_input(input)?;

    let mut in_progress = HashMap::new();
    let mut idle: HashSet<_> = (0..workers).collect();
    let mut steps = String::new();
    let mut t = 0;

    while !ready.is_empty() || idle.len() != workers as usize {
        for worker in idle.clone().iter() {
            in_progress.remove(worker);
            ready.sort_unstable_by(|a, b| b.cmp(a));
            if let Some(step) = ready.pop() {
                steps.push(step);
                idle.remove(&worker);
                in_progress.insert(
                    *worker,
                    Job {
                        step,
                        time_remaining: time_to_complete(base_seconds, step),
                    },
                );
            }
        }

        t += 1;

        for (worker, job) in in_progress.iter_mut() {
            job.time_remaining -= 1;
            if job.time_remaining == 0 {
                idle.insert(*worker);
                if let Some(blocked) = blocks.get(&job.step) {
                    for b in blocked {
                        if let Some(blocking) = blocked_by.get_mut(&b) {
                            blocking.remove(&job.step);
                            if blocking.is_empty() {
                                ready.push(*b);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok((steps, t))
}

fn time_to_complete(base_seconds: u32, c: char) -> u32 {
    base_seconds + u32::from(*c.to_string().as_bytes().iter().next().unwrap()) - 64
}

struct Project {
    ready: Vec<char>,
    blocks: HashMap<char, HashSet<char>>,
    blocked_by: HashMap<char, HashSet<char>>,
}

fn parse_input(input: &str) -> Result<Project> {
    let mut blocks = HashMap::new();
    let mut blocked_by = HashMap::new();

    for l in input.trim().lines() {
        let caps = require_with!(
            re(r"Step (.) must be finished before step (.) can begin.")?.captures(l),
            "couldn't parse step {}",
            l,
        );

        let blocking = parse(&caps[1])?;
        let blocked = parse(&caps[2])?;

        blocks
            .entry(blocking)
            .or_insert_with(HashSet::new)
            .insert(blocked);

        blocked_by
            .entry(blocked)
            .or_insert_with(HashSet::new)
            .insert(blocking);
    }

    let ready = blocks
        .keys()
        .cloned()
        .collect::<HashSet<_>>()
        .difference(&blocked_by.keys().cloned().collect())
        .cloned()
        .collect();

    Ok(Project {
        ready,
        blocks,
        blocked_by,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
        Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.
    ";

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE), Ok("CABDFE".to_string()));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE, 2, 0), Ok(15));
    }
}
