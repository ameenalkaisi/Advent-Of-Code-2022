use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let file = File::open(filename).expect("file is incorrect");
    let lines = io::BufReader::new(file).lines();

    let mut max_calories = [0, 0, 0];

    let mut current_sum: u32 = 0;
    for line in lines {

        if let Ok(bruh) = line {
            println!("{}", bruh.to_string());

            if bruh.is_empty() {
                current_sum = 0;
                continue;
            }

            current_sum += bruh.trim().parse::<u32>().unwrap();

            for i in 0..3 {
                if current_sum > max_calories[i] {
                    max_calories[i] = current_sum;
                    break;
                }
            }
        }
    }

    println!("{}", max_calories[0] + max_calories[1] + max_calories[2]);
}
