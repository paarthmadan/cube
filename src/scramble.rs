use std::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl Distribution<Move> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        match rng.gen_range(0, 6) {
            0 => Move::Up,
            1 => Move::Down,
            2 => Move::Left,
            3 => Move::Right,
            4 => Move::Front,
            _ => Move::Back,
        }
    }
}

enum Turn {
    Forward,
    Reverse,
    Double,
}

impl Distribution<Turn> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Turn {
        match rng.gen_range(0, 3) {
            0 => Turn::Forward,
            1 => Turn::Reverse,
            _ => Turn::Double,
        }
    }
}

impl Turn {
    fn resolve(&self) -> &str {
        match *self {
            Turn::Forward => "",
            Turn::Reverse => "'",
            Turn::Double => "2",
        }
    }
}

impl Move {
    fn resolve(&self) -> &str {
        match *self {
            Move::Up => "U",
            Move::Down => "D",
            Move::Left => "L",
            Move::Right => "R",
            Move::Front => "F",
            Move::Back => "B",
        }
    }
}

pub struct Scramble {
    moves: Vec<String>,
}

impl fmt::Display for Scramble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.moves.join(" "))
    }
}

impl Default for Scramble {
    fn default() -> Self {
        let mut moves: Vec<String> = Vec::new();
        let mut prev: Option<Move> = None;

        for _ in 1..=20 {
            let mut r: Move = rand::random();
            let mv = match prev {
                Some(p) => {
                    while r == p {
                        r = rand::random();
                    }
                    r
                }
                None => r,
            };

            prev = Some(mv);

            let turn: Turn = rand::random();

            let s = format!("{}{}", mv.resolve(), turn.resolve());

            moves.push(s);
        }

        Scramble { moves }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scramble() {
        let scramble = super::Scramble::default();

        assert_eq!(20, scramble.moves.len());
    }
}
