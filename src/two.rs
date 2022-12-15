use std::str::Split;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

pub trait Winner {
    fn winner(&self) -> Choice;
}

pub trait Loser {
    fn loser(&self) -> Choice;
}

pub trait Score {
    fn score(&self) -> i64;
}

impl Score for Choice {
    fn score(&self) -> i64 {
        match self {
            Self::Paper => 2,
            Self::Rock => 1,
            Self::Scissors => 3,
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> i64 {
        match self {
            Self::Draw => 3,
            Self::Lose => 0,
            Self::Win => 6,
        }
    }
}

impl Winner for Choice {
    fn winner(&self) -> Choice {
        match self {
            Self::Paper => Choice::Rock,
            Self::Scissors => Choice::Paper,
            Self::Rock => Choice::Scissors,
        }
    }
}

impl Loser for Choice {
    fn loser(&self) -> Choice {
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

impl Outcome {
    fn convert(s: &str) -> Result<Self, &'static str> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => return Err("Not a valid outcome: {s:?}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FixedGame {
    expected_result: Outcome,
    theirs: Choice,
}

impl FixedGame {
    fn pick_winner(&self) -> Choice {
        match self.expected_result {
            Outcome::Win => self.theirs.loser(),
            Outcome::Draw => self.theirs,
            Outcome::Lose => self.theirs.winner(),
        }
    }

    fn get_score(&self) -> i64 {
        self.pick_winner().score() + self.expected_result.score()
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

    fn get_score(&self) -> i64 {
        self.ours.score() + self.find_winner().score()
    }
}

pub fn match_hands(mut line: Split<&str>) -> Result<i64, &'static str> {
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

    Ok(round.get_score())
}

pub fn pick_hands(mut line: Split<&str>) -> Result<i64, &'static str> {
    let p1 = match line.next() {
        Some(line) => line,
        None => return Err("No choice for player 1"),
    };

    let p2 = match line.next() {
        Some(line) => line,
        None => return Err("No choice for player 2"),
    };

    let round = FixedGame {
        expected_result: Outcome::convert(p2)?,
        theirs: Choice::convert(p1)?,
    };

    Ok(round.get_score())
}
