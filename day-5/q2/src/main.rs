use std::{
    env,
    error::Error,
    io::{self, BufRead, Seek},
    str::FromStr,
    string::ParseError,
};
#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn new() -> Self {
        Stacks { stacks: vec![] }
    }

    pub fn perform_move(&mut self, move_to_do: Move) {
        self.move_stacks(move_to_do.source, move_to_do.target, move_to_do.count);
    }

    pub fn move_stacks(&mut self, first_index: usize, second_index: usize, num_to_move: u32) {
        let mut vals_to_move = vec![];
        for _ in 0..num_to_move {
            let popped_val = self.stacks[first_index - 1].pop();

            vals_to_move.push(popped_val);
        }

        vals_to_move.reverse();
        for i in vals_to_move {
            self.stacks[second_index - 1].push(i.unwrap());
        }
    }
}

#[derive(Debug)]
struct Move {
    source: usize,
    target: usize,
    count: u32,
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.split(" ").collect::<Vec<&str>>();
        Ok(Move {
            count: line[1].parse().unwrap(),
            source: line[3].parse().unwrap(),
            target: line[5].parse().unwrap(),
        })
    }
}

impl FromStr for Stacks {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\r\n").collect();
        let mut stacks = Stacks::new();

        for line in lines {
            // always will have first value
            let line = line.chars().collect::<Vec<char>>();

            if line[1].is_numeric() {
                break;
            }

            if stacks.stacks.is_empty() {
                stacks.stacks.push(vec![]);
            }

            if !line[1].is_whitespace() {
                stacks.stacks[0].push(line[1]);
            }

            for i in (5..line.len()).step_by(4) {
                if stacks.stacks.len() < (i - 5) / 4 + 2 {
                    stacks.stacks.push(vec![]);
                }

                if !line[i].is_whitespace() {
                    stacks.stacks[(i - 5) / 4 + 1].push(line[i]);
                }
            }
        }

        Ok(stacks)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // maybe read into string, split by line, change into characters, read stuff from there idk
    // todo
    //let mut file = std::fs::File::open(env::args().nth(1).unwrap())?;

    let mut stacks = Stacks::new();

    let mut inputs: Vec<String> = std::fs::read_to_string(env::args().nth(1).unwrap())
        .unwrap()
        .split("\r\n\r\n")
        .map(|s| String::from(s))
        .collect();

    let mut defn = &inputs[0];
    let mut moves = &inputs[1];

    stacks = defn.parse().unwrap();

    for stack in stacks.stacks.iter_mut() {
        stack.reverse();
    }

    moves.split("\r\n").for_each(|cur_move| {
        if !cur_move.is_empty() {
            stacks.perform_move(cur_move.parse().unwrap())
        }
    });

    for stack in stacks.stacks.iter() {
        print!("{}", stack.last().unwrap());
    }

    Ok(())
}
