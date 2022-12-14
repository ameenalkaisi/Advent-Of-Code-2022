use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let file = File::open(filename).expect("file is incorrect");
    let lines = io::BufReader::new(file).lines();

    let mut total_priority: u32 = 0;
    let mut counter: u32 = 0;
    let mut group_char_counter: HashMap<char, u32> = HashMap::new();
    for input in lines {
        // if it's already read 3 lines,
        // add any found badges into the total and
        // reset the map

        if let Err(_) = input.as_ref() {
            break;
        }

        // will add unique letters of this line into the map
        let mut visited: Vec<char> = Vec::new();
        for letter in input.unwrap().chars() {
            if !visited.contains(&letter) {
                if let Some(val) = group_char_counter.get(&letter) {
                    group_char_counter.insert(letter, val + 1);
                } else {
                    group_char_counter.insert(letter, 1);
                }

                visited.push(letter);
            }
        }

        counter += 1;
        if !group_char_counter.is_empty() && counter % 3 == 0 {
            for (key, val) in group_char_counter.iter() {
                if *val == 3 {
                    total_priority += letter_priority(*key);
                }
            }

            group_char_counter.clear();
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
