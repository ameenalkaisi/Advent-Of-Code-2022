use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let file = File::open(filename).expect("file is incorrect");
    let lines = io::BufReader::new(file).lines();

    let mut max_calories: u32 = 0;

    let mut current_sum: u32 = 0;
    for line in lines {

        if let Ok(bruh) = line {
            println!("{}", bruh.to_string());

            if bruh.is_empty() {
                current_sum = 0;
                continue;
            }

            current_sum += bruh.trim().parse::<u32>().unwrap();

            if current_sum > max_calories {
                max_calories = current_sum;
            } // keep updating in case this is the last one
              // not much overhead anyways
        }
    }

    println!("{max_calories}");
}
