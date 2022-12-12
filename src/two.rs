use anyhow;

pub enum GameResult {
    Win(i32),
    Lose(i32),
    Draw(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub fn make_choice(choice: &str) -> Result<Choice, anyhow::Error> {
    let choice_out = match choice {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => panic!("Invalid choice!"),
    };

    Ok(choice_out)
}

pub trait Winner {
    fn winner(&self) -> Self;
}

pub trait Score {
    fn score(&self) -> i32;
}

impl Score for Choice {
    fn score(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3
        }
    }
}

impl Winner for Choice {
    fn winner(&self) -> Self {
        match *self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct RockPaperScissors {
    player1: Choice,
    player2: Choice,
}

impl RockPaperScissors {

    fn new(player1: &str, player2: &str) -> Result<RockPaperScissors, anyhow::Error> {
        let p1 = make_choice(player1)?;
        let p2 = make_choice(player2)?;

        Ok(RockPaperScissors { player1: p1, player2: p2 })
    }

    fn game<'a>(mut split: impl Iterator<Item = &'a str>) -> Result<i32, anyhow::Error> {

        let p1 = match split.next() {
            Some(split) => split,
            None => return anyhow::error::Error
        };

        let p2 = match split.next() {
            Some(split) => split,
            None => return Err("No value for player 2 found.")
        };

        let game = RockPaperScissors::new(p1, p2)?;

        Ok(10)

    }
}

#[cfg(test)]
mod tests {
    use super::Choice::*;
    use super::*;

    #[test]
    fn test_make_choice() {
        assert_eq!(make_choice("A").unwrap(), Rock)
    }

    #[test]
    fn test_new_rockpaperscissors() {

        let game = RockPaperScissors::new("A","Y").unwrap();

        assert_eq!(game, RockPaperScissors{ player1: Choice::Rock, player2: Choice::Paper})
    }
}
