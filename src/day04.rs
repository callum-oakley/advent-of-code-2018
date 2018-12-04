use error::{parse, re, Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Log {
    minute: u8,
    event: Event,
}

impl FromStr for Log {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let caps = require_with!(
            re(r"\[....-..-.. ..:(..)\] (.*)")?.captures(s),
            "couldn't parse log {}",
            s
        );

        Ok(Log {
            minute: parse(&caps[1])?,
            event: parse(&caps[2])?,
        })
    }
}

#[derive(Debug)]
enum Event {
    BeginsShift(i32),
    FallsAsleep,
    WakesUp,
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match require_with!(s.chars().next(), "empty event") {
            'G' => Ok(Event::BeginsShift(parse(
                &require_with!(re(r"#(\d+)")?.captures(s), "couldn't parse event {}", s)[1],
            )?)),
            'f' => Ok(Event::FallsAsleep),
            'w' => Ok(Event::WakesUp),
            _ => bail!("couldn't parse event {}", s),
        }
    }
}

pub fn part1(input: &str) -> Result<i32> {
    let guards = parse_input(input)?;

    let id = *guards
        .keys()
        .max_by_key(|g| guards[g].values().sum::<i32>())
        .unwrap();

    let minute = *guards[&id].keys().max_by_key(|m| guards[&id][m]).unwrap();

    Ok(id * minute as i32)
}

pub fn part2(input: &str) -> Result<i32> {
    let guards = parse_input(input)?;

    let (id, minute) = guards
        .keys()
        .flat_map(|g| (0..49).map(move |m| (g, m)))
        .max_by_key(|(g, m)| guards[g].get(m).unwrap_or(&0))
        .unwrap();

    Ok(id * minute as i32)
}

fn parse_input(input: &str) -> Result<HashMap<i32, HashMap<u8, i32>>> {
    let mut raw_logs = input.trim().lines().map(|l| l.trim()).collect::<Vec<_>>();
    raw_logs.sort_unstable();
    let logs = raw_logs
        .iter()
        .map(|l| parse::<Log>(l))
        .collect::<Result<Vec<_>>>()?;

    // Guard ID -> Minute -> times asleep at that minute
    let mut guards = HashMap::new();
    let mut on_duty = None;
    let mut fell_asleep_at = None;

    for log in logs {
        match log.event {
            Event::BeginsShift(id) => on_duty = Some(id),
            Event::FallsAsleep => fell_asleep_at = Some(log.minute),
            Event::WakesUp => {
                for m in fell_asleep_at.take().unwrap()..log.minute {
                    guards
                        .entry(on_duty.unwrap())
                        .or_insert(HashMap::new())
                        .entry(m)
                        .and_modify(|s| *s += 1)
                        .or_insert(1);
                }
            }
        }
    }

    Ok(guards)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
        [1518-11-01 00:00] Guard #10 begins shift
        [1518-11-01 00:05] falls asleep
        [1518-11-01 00:25] wakes up
        [1518-11-01 00:30] falls asleep
        [1518-11-01 00:55] wakes up
        [1518-11-01 23:58] Guard #99 begins shift
        [1518-11-02 00:40] falls asleep
        [1518-11-02 00:50] wakes up
        [1518-11-03 00:05] Guard #10 begins shift
        [1518-11-03 00:24] falls asleep
        [1518-11-03 00:29] wakes up
        [1518-11-04 00:02] Guard #99 begins shift
        [1518-11-04 00:36] falls asleep
        [1518-11-04 00:46] wakes up
        [1518-11-05 00:03] Guard #99 begins shift
        [1518-11-05 00:45] falls asleep
        [1518-11-05 00:55] wakes up
    ";

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE), Ok(240));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE), Ok(4455));
    }
}
