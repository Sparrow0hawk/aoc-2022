use std::str::Split;

#[derive(Debug, PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
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

pub fn match_hands(mut line: Split<&str>) -> Result<Game, &'static str> {
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

    Ok(round)
}
