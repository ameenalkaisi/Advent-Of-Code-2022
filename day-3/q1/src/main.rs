use std::{collections::HashMap, env, fs::File, io::{self, BufRead}};

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let file = File::open(filename).expect("file is incorrect");
    let lines = io::BufReader::new(file).lines();

    let mut total_priority: u32 = 0;
    for input in lines {
        if let Err(_) = input {
            break;
        }

        let input = input.unwrap();

        let len = input.len();

        let mut firstCounter: HashMap<char, u32> = HashMap::new();
        let mut secondCounter: HashMap<char, u32> = HashMap::new();

        for (i, c) in input.chars().enumerate() {
            if i < len / 2 {
                if let Some(val) = firstCounter.get(&c) {
                    firstCounter.insert(c, val + 1);
                } else {
                    firstCounter.insert(c, 1);
                }
            } else {
                if let Some(val) = secondCounter.get(&c) {
                    secondCounter.insert(c, val + 1);
                } else {
                    secondCounter.insert(c, 1);
                }
            }
        }

        for key in firstCounter.keys() {
            if let Some(val) = secondCounter.get(key) {
                total_priority += letter_priority(*key);
            }
        }
    }

    println!("{total_priority}");
}

fn letter_priority(letter: char) -> u32 {
    if letter.is_uppercase() {
        return letter as u32 - 'A' as u32 + 27;
    } else {
        return letter as u32 - 'a' as u32 + 1;
    }
}
