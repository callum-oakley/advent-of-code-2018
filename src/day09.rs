use crate::de_list::DeList;
use crate::error::Result;
use std::collections::HashMap;
use std::rc::Rc;

type Score = i64;
type Elf = i32;

#[derive(Debug)]
struct Ring(Rc<DeList<Score>>);

impl Ring {
    fn new() -> Self {
        let root = Rc::new(DeList::new(0));
        root.set_left(&root);
        root.set_right(&root);
        Ring(root)
    }

    fn insert(&mut self, value: Score) {
        let new = Rc::new(DeList::new(value));
        let left = self.0.left();

        new.set_left(&left);
        left.set_right(&new);

        new.set_right(&self.0);
        self.0.set_left(&new);

        self.rotate_left(1);
    }

    fn remove(&mut self) -> Score {
        let left = self.0.left();
        let right = self.0.right();

        let value = *self.0.value();

        left.set_right(&right);
        right.set_left(&left);

        *self = Ring(right);

        value
    }

    fn rotate_left(&mut self, n: usize) {
        for _ in 0..n {
            *self = Ring(self.0.left());
        }
    }

    fn rotate_right(&mut self, n: usize) {
        for _ in 0..n {
            *self = Ring(self.0.right());
        }
    }
}
#[derive(Debug)]
struct Game {
    marbles: Ring,
    scores: HashMap<Elf, Score>,
}

impl Game {
    fn new() -> Self {
        Game {
            marbles: Ring::new(),
            scores: HashMap::new(),
        }
    }

    fn play_turn(&mut self, marble: Score, elf: Elf) {
        if marble % 23 == 0 {
            self.marbles.rotate_left(7);
            *self.scores.entry(elf).or_insert(0) += marble + self.marbles.remove();
        } else {
            self.marbles.rotate_right(2);
            self.marbles.insert(marble);
        }
    }

    fn high_score(&self) -> Score {
        self.scores.values().cloned().max().unwrap_or(0)
    }
}

pub fn part1(players: Elf, last_marble: Score) -> Result<Score> {
    let mut game = Game::new();
    for (marble, elf) in (1..=last_marble).zip((0..players).cycle()) {
        game.play_turn(marble, elf);
    }
    Ok(game.high_score())
}

pub fn part2(players: Elf, last_marble: Score) -> Result<Score> {
    let mut game = Game::new();
    for (marble, elf) in (1..=last_marble * 100).zip((0..players).cycle()) {
        game.play_turn(marble, elf);
    }
    Ok(game.high_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        assert_eq!(part1(9, 25), Ok(32));
        assert_eq!(part1(10, 1618), Ok(8317));
        assert_eq!(part1(13, 7999), Ok(146373));
        assert_eq!(part1(17, 1104), Ok(2764));
        assert_eq!(part1(21, 6111), Ok(54718));
        assert_eq!(part1(30, 5807), Ok(37305));
    }
}
