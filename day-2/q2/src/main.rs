use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, PartialOrd, Clone)]
enum Hand {
    Rock,
    Papers,
    Scissors,
}

impl Ord for Hand {
    // probably can do this a better way
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        match self {
            Hand::Rock => match other {
                Hand::Scissors => Ordering::Greater,
                Hand::Papers => Ordering::Less,
                Hand::Rock => Ordering::Equal,
            },
            Hand::Papers => match other {
                Hand::Scissors => Ordering::Less,
                Hand::Papers => Ordering::Equal,
                Hand::Rock => Ordering::Greater,
            },
            Hand::Scissors => match other {
                Hand::Scissors => Ordering::Equal,
                Hand::Papers => Ordering::Greater,
                Hand::Rock => Ordering::Less,
            },
        }
    }
}

impl Hand {
    pub fn new(input: String) -> Hand {
        match input.as_str() {
            "A" => Hand::Rock,
            "B" => Hand::Papers,
            "C" => Hand::Scissors,
            _ => panic!("input was {input}"),
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Papers => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn hand_to(&self, action: &str) -> Hand {
        // return hand to do <action>
        // "X" to lose, "Y" to draw, "Z" to win
        match action {
            // must lose
            "X" => match self {
                Hand::Rock => Hand::Scissors,
                Hand::Papers => Hand::Rock,
                Hand::Scissors => Hand::Papers,
            },
            // always give self back to draw
            "Y" => self.clone(),
            // must win
            "Z" => match self {
                Hand::Rock => Hand::Papers,
                Hand::Papers => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
            _ => panic!("incorrect thing read"),
        }
    }
}

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let file = File::open(filename).expect("file is incorrect");
    let lines = io::BufReader::new(file).lines();

    let mut total_score: u32 = 0;
    for line in lines {
        if let Ok(line) = line {
            let opp = &line[0..1];
            let action = &line[2..3];

            let opp = Hand::new(opp.to_string());
            let should_choose = opp.hand_to(action);

            total_score += should_choose.score();

            match should_choose.cmp(&opp) {
                Ordering::Greater => total_score += 6,
                Ordering::Equal => total_score += 3,
                _ => (),
            }
        }
    }

    println!("{total_score}");
}
