use anyhow;
use aoc_2022::read_lines;

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

impl Winner for Choice {
    fn winner(&self) -> Self {
        match *self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }
}

pub struct RockPaperScissors {
    player1: String,
    player2: String,
}

impl RockPaperScissors {
    fn new(player1: String, player2: String) -> RockPaperScissors {
        RockPaperScissors { player1, player2 }
    }
}

fn play(game: RockPaperScissors)

#[cfg(test)]
mod tests {
    use super::Choice::*;
    use super::*;

    #[test]
    fn test_make_choice() {
        assert_eq!(make_choice("A").unwrap(), Rock)
    }
}
