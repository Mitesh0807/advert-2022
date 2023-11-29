use std::fs::File;
use std::io::Read;
#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}
impl Move {
    fn value(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}
impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[derive(Debug, Clone)]
enum Expectation {
    Win,
    Draw,
    Lose,
}
impl Expectation {
    fn value(&self, other: &Move) -> u32 {
        match self {
            Expectation::Win => match other {
                Move::Rock => 6,
                Move::Paper => 0,
                Move::Scissors => 3,
            },
            Expectation::Draw => match other {
                Move::Rock => 3,
                Move::Paper => 1,
                Move::Scissors => 2,
            },
            Expectation::Lose => match other {
                Move::Rock => 0,
                Move::Paper => 2,
                Move::Scissors => 1,
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Round {
    RockPaper,
    RockScissors,
    PaperScissors,
    PaperRock,
    ScissorsRock,
    ScissorsPaper,
}
impl Round {
    fn value(&self) -> u32 {
        match self {
            Round::RockPaper => 1 + 6,
            Round::RockScissors => 1 + 0,
            Round::PaperScissors => 2 + 6,
            Round::PaperRock => 2 + 0,
            Round::ScissorsRock => 3 + 0,
            Round::ScissorsPaper => 3 + 6,
        }
    }
}
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut total_score = 0;
    let mut total_value_future = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let my_move = parts[1];
        let oppent_move = parts[0];
        let my_move2: Move = match my_move {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("invalid move"),
        };
        let my_move_value = my_move2.value();
        let my_move_future = match my_move {
            "X" => Expectation::Lose,
            "Y" => Expectation::Draw,
            "Z" => Expectation::Win,
            _ => panic!("invalid move"),
        };
        let oppent_move: Move = match oppent_move {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("invalid move"),
        };
        let my_move_value_future = my_move_future.value(&oppent_move);
        total_value_future += my_move_value_future;
        let outcome = match (my_move2, oppent_move) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Lose,
            (Move::Paper, Move::Scissors) => Outcome::Lose,
            (Move::Scissors, Move::Rock) => Outcome::Lose,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Win,
        };
        // let outcome_new_str = match
        total_score += outcome.value() + my_move_value;
    }
    println!(" total score: {}", total_score);
    println!(" total value future: {}", total_value_future);
}
