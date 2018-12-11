use crate::error::{parse, require_with, Result};

struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    fn from_iterator(ns: &mut impl Iterator<Item = i32>) -> Result<Node> {
        let no_of_children = require_with!(ns.next(), "couldn't create node");
        let no_of_metadata = require_with!(ns.next(), "couldn't create node");
        let children = (0..no_of_children)
            .map(|_| Node::from_iterator(ns))
            .collect::<Result<_>>()?;
        let metadata = ns.take(no_of_metadata as usize).collect();
        Ok(Node { children, metadata })
    }

    fn sum_metadata(&self) -> i32 {
        self.metadata.iter().sum::<i32>()
            + self
                .children
                .iter()
                .map(|child| child.sum_metadata())
                .sum::<i32>()
    }

    fn value(&self) -> i32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|i| self.children.get((i - 1) as usize))
                .map(|child| child.value())
                .sum::<i32>()
        }
    }
}

pub fn part1(input: &str) -> Result<i32> {
    Ok(parse_input(input)?.sum_metadata())
}

pub fn part2(input: &str) -> Result<i32> {
    Ok(parse_input(input)?.value())
}

fn parse_input(input: &str) -> Result<Node> {
    let ns: Vec<i32> = input
        .trim()
        .split(' ')
        .map(|w| parse(w))
        .collect::<Result<_>>()?;

    Node::from_iterator(&mut ns.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        assert_eq!(part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), Ok(138));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), Ok(66));
    }
}
