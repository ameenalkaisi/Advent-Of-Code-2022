use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let file = File::open(filename).expect("file is incorrect");
    let lines = io::BufReader::new(file).lines();

    let mut contained_pair_count: u32 = 0;

    for line in lines {
        if let Err(_) = line {
            break;
        }

        let line = line.unwrap();

        // get the ranges
        let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        for cap in regex.captures_iter(&line) {
            let first_begin: u32 = cap[1].parse().unwrap();
            let first_end: u32 = cap[2].parse().unwrap();
            let second_begin: u32 = cap[3].parse().unwrap();
            let second_end: u32 = cap[4].parse().unwrap();

            if (first_begin <= second_begin && first_end >= second_end)
                || (second_begin <= first_begin && second_end >= first_end)
            {
                contained_pair_count += 1;
            }
        }
    }
    println!("{contained_pair_count}");
}
