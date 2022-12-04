extern crate core;

use crate::input::{example_input, my_input};
use crate::Move::{PAPER, ROCK, SCISSORS};
use crate::Result::{DRAW, LOSS, VICTORY};

mod input;

fn main() {
    let input = my_input();
    let mut running_score_1 = 0;
    let mut running_score_2 = 0;

    for mut line in input.lines() {
        line = line.trim();

        if line.is_empty() {
            continue;
        }

        let parts = line.split_whitespace().collect::<Vec<&str>>();

        if parts.len() != 2 {
            panic!("Invalid input line: {}", line)
        }

        running_score_1 += get_score(parts[0], parts[1]);
        running_score_2 += get_score_2(parts[0], parts[1]);
    }

    println!("My score when I was figuring it out myself is {}.", running_score_1);
    println!("My score after the Elf told me what the strategy actually means will be {}.", running_score_2);
}

fn get_score(op_move: &str, my_move: &str) -> i32 {
    Result::from_moves(
        Move::from_str(op_move),
        Move::from_str(my_move)
    ).score() + Move::from_str(my_move).score()
}

fn get_score_2(op_move: &str, required_result: &str) -> i32 {
    let parsed_required_result = Result::from_str(required_result);
    let my_move = parsed_required_result.get_required_move(Move::from_str(op_move));

    return Result::from_moves(
        Move::from_str(op_move),
        my_move
    ).score() + my_move.score();
}

#[derive(PartialEq, Eq)]
enum Result {
    VICTORY,
    LOSS,
    DRAW
}

impl Result {
    pub fn from_moves(op_move: Move, my_move: Move) -> Result
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

    pub fn from_str(str: &str) -> Result {
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