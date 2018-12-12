use crate::error::Result;
use crate::ring::Ring;
use std::collections::HashMap;

type Score = i64;
type Elf = i32;

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

    fn high_score(&self) -> i64 {
        self.scores.values().cloned().max().unwrap_or(0)
    }
}

pub fn part1(players: i32, last_marble: i64) -> Result<i64> {
    let mut game = Game::new();
    for (marble, elf) in (1..=last_marble).zip((0..players).cycle()) {
        game.play_turn(marble, elf);
    }
    Ok(game.high_score())
}

pub fn part2(players: i32, last_marble: i64) -> Result<i64> {
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
