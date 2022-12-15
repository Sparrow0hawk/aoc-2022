use std::str::Split;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

pub trait Winner {
    fn winner(&self) -> Choice;
}

impl Winner for Choice {
    fn winner(&self) -> Choice {
        match self {
            Self::Rock => Choice::Paper,
            Self::Paper => Choice::Scissors,
            Self::Scissors => Choice::Rock,
        }
    }
}

impl Choice {
    fn convert(s: &str) -> Result<Self, &'static str> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => return Err("Not a valid choice: {s:?}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    ours: Choice,
    theirs: Choice,
}

impl Game {
    fn find_winner(&self) -> Outcome {
        let (ours_win, theirs_win) = (self.ours.winner(), self.theirs.winner());

        match (ours_win, theirs_win) {
            _ if ours_win == self.theirs => Outcome::Win,
            _ if theirs_win == self.ours => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }
}

pub fn match_hands(mut line: Split<&str>) -> Result<Outcome, &'static str> {
    let p1 = match line.next() {
        Some(line) => line,
        None => return Err("No choice for player 1"),
    };

    let p2 = match line.next() {
        Some(line) => line,
        None => return Err("No choice for player 2"),
    };

    let round = Game {
        ours: Choice::convert(p2)?,
        theirs: Choice::convert(p1)?,
    };

    Ok(round.find_winner())
}
