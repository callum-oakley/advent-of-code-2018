use crate::error::Result;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Cart {
    pos: (usize, usize),
    dir: char,
    turns: usize,
    crashed: bool,
}

impl Cart {
    fn tick(&mut self) {
        match self.dir {
            '^' => self.pos.1 -= 1,
            '>' => self.pos.0 += 1,
            'v' => self.pos.1 += 1,
            '<' => self.pos.0 -= 1,
            _ => unreachable!(),
        }
    }

    fn turn(&mut self, c: char) {
        match c {
            '/' => match self.dir {
                '^' => self.dir = '>',
                '>' => self.dir = '^',
                'v' => self.dir = '<',
                '<' => self.dir = 'v',
                _ => unreachable!(),
            },
            '\\' => match self.dir {
                '^' => self.dir = '<',
                '>' => self.dir = 'v',
                'v' => self.dir = '>',
                '<' => self.dir = '^',
                _ => unreachable!(),
            },
            '+' => {
                match self.turns % 3 {
                    0 => match self.dir {
                        '^' => self.dir = '<',
                        '>' => self.dir = '^',
                        'v' => self.dir = '>',
                        '<' => self.dir = 'v',
                        _ => unreachable!(),
                    },
                    2 => match self.dir {
                        '^' => self.dir = '>',
                        '>' => self.dir = 'v',
                        'v' => self.dir = '<',
                        '<' => self.dir = '^',
                        _ => unreachable!(),
                    },
                    _ => {}
                }
                self.turns += 1;
            }
            _ => unreachable!(),
        }
    }
}

pub fn part1(input: &str) -> Result<String> {
    let (mut carts, tracks) = parse_input(input);

    loop {
        sort_carts(&mut carts);
        for cart in carts.iter() {
            let pos = cart.borrow().pos;
            if let Some(c) = tracks.get(&pos) {
                cart.borrow_mut().turn(*c);
            }
            cart.borrow_mut().tick();
            if let Some((x, y)) = mark_crashed(&carts) {
                return Ok(format!("{},{}", x, y));
            }
        }
    }
}

pub fn part2(input: &str) -> Result<String> {
    let (mut carts, tracks) = parse_input(input);

    loop {
        sort_carts(&mut carts);
        for cart in carts.iter() {
            let pos = cart.borrow().pos;
            if let Some(c) = tracks.get(&pos) {
                cart.borrow_mut().turn(*c);
            }
            cart.borrow_mut().tick();
            mark_crashed(&carts);
        }
        carts.retain(|cart| !cart.borrow().crashed);
        if carts.len() == 1 {
            let (x, y) = carts[0].borrow().pos;
            return Ok(format!("{},{}", x, y));
        }
    }
}

fn mark_crashed(carts: &Vec<RefCell<Cart>>) -> Option<(usize, usize)> {
    for i in 0..carts.len() {
        for j in 0..i {
            let mut cart_i = carts[i].borrow_mut();
            let mut cart_j = carts[j].borrow_mut();
            if !cart_i.crashed && !cart_j.crashed && cart_i.pos == cart_j.pos {
                cart_i.crashed = true;
                cart_j.crashed = true;
                return Some(cart_i.pos);
            }
        }
    }
    None
}

fn sort_carts(carts: &mut Vec<RefCell<Cart>>) {
    carts.sort_unstable_by(|a, b| {
        let a = a.borrow();
        let b = b.borrow();
        if a.pos.1 == b.pos.1 {
            a.pos.0.cmp(&b.pos.0)
        } else {
            a.pos.1.cmp(&b.pos.1)
        }
    });
}

fn parse_input(input: &str) -> (Vec<RefCell<Cart>>, HashMap<(usize, usize), char>) {
    let mut carts = Vec::new();
    let mut tracks = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' | '>' | 'v' | '<' => carts.push(RefCell::new(Cart {
                    pos: (x, y),
                    dir: c,
                    turns: 0,
                    crashed: false,
                })),
                '+' | '/' | '\\' => {
                    tracks.insert((x, y), c);
                }
                _ => {}
            }
        }
    }

    (carts, tracks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &'static str = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
";

    const SAMPLE_2: &'static str = r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
";

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE_1), Ok("7,3".to_string()));
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE_2), Ok("6,4".to_string()));
    }
}
