extern crate core;

use crate::input::{get_input};
use crate::Move::{PAPER, ROCK, SCISSORS};
use crate::Outcome::{DRAW, LOSS, VICTORY};

mod input;

fn main() {
    println!("My score when I was figuring it out myself is {}.", my_guess(get_input()));
    println!("My score after the Elf told me what the strategy actually means will be {}.", actually(get_input()));
}

fn my_guess(input: &str) -> i32 {
    let mut running_score = 0;

    for line in input.lines() {
        let (op_move, my_move) = match parse_line(line) {
            Ok(res) => res,
            Err(_) => continue
        };

        running_score += get_score(op_move, my_move);
    }

    return running_score
}

fn actually(input: &str) -> i32 {
    let mut running_score = 0;

    for line in input.lines() {
        let (op_move, required_outcome) = match parse_line(line) {
            Ok(res) => res,
            Err(_) => continue
        };

        running_score += get_score_2(op_move, required_outcome);
    }

    return running_score
}

fn parse_line(mut input_line: &str) -> Result<(&str, &str), &str> {
    input_line = input_line.trim();

    if input_line.is_empty() {
        return Err("Empty line.")
    }

    let parts = input_line.split_whitespace().collect::<Vec<&str>>();

    if parts.len() != 2 {
        return Err("Invalid line.")
    }

    Ok((parts[0], parts[1]))
}

fn get_score(op_move: &str, my_move: &str) -> i32 {
    Outcome::from_moves(
        Move::from_str(op_move),
        Move::from_str(my_move)
    ).score() + Move::from_str(my_move).score()
}

fn get_score_2(op_move: &str, required_outcome: &str) -> i32 {
    let parse_require_outcome = Outcome::from_str(required_outcome);
    let my_move = parse_require_outcome.get_required_move(Move::from_str(op_move));

    return Outcome::from_moves(
        Move::from_str(op_move),
        my_move
    ).score() + my_move.score();
}

#[derive(PartialEq, Eq)]
enum Outcome {
    VICTORY,
    LOSS,
    DRAW
}

impl Outcome {
    pub fn from_moves(op_move: Move, my_move: Move) -> Outcome
    {
        if op_move == my_move {
            return DRAW;
        }

        if
            op_move == ROCK && my_move == PAPER ||
            op_move == PAPER && my_move == SCISSORS ||
            op_move == SCISSORS && my_move == ROCK
        {
            return VICTORY
        }

        return LOSS
    }

    pub fn get_required_move(&self, op_move: Move) -> Move {
        return match *self {
            DRAW => op_move,
            VICTORY => match op_move {
                ROCK => PAPER,
                PAPER => SCISSORS,
                SCISSORS => ROCK
            }
            LOSS => match op_move
            {
                ROCK => SCISSORS,
                PAPER => ROCK,
                SCISSORS => PAPER,
            }
        }
    }

    pub fn score(&self) -> i32 {
        return match *self {
            VICTORY => 6,
            DRAW => 3,
            LOSS => 0,
        }
    }

    pub fn from_str(str: &str) -> Outcome {
        match str {
            "X" => LOSS,
            "Y" => DRAW,
            "Z" => VICTORY,
            &_ => panic!("{} is not a valid result string.", str)
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS
}

impl Move {
    pub fn from_str(str_move: &str) -> Move {
        match str_move {
            "A" | "X" => return ROCK,
            "B" | "Y" => return PAPER,
            "C" | "Z" => return SCISSORS,
            _ => panic!("Invalid move: {}", str_move)
        }
    }

    pub fn score(&self) -> i32 {
        return match *self {
            ROCK => 1,
            PAPER => 2,
            SCISSORS => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{actually, my_guess};

    const TEST_INPUT: &str = "
        A Y
        B X
        C Z
    ";

    #[test]
    fn test_my_guess() {
        assert_eq!(my_guess(TEST_INPUT), 15)
    }

    #[test]
    fn test_actually() {
        assert_eq!(actually(TEST_INPUT), 12)
    }
}