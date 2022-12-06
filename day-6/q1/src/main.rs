use std::{env, fs};

fn is_unique(arr: &Vec<char>) -> bool {
    let mut copy = arr.clone();
    copy.sort();
    for i in 0..copy.len() - 1 {
        if copy[i] == copy[i + 1] {
            return false;
        }
    }

    return true;
}

fn main() {
    let input: Vec<char> = fs::read_to_string(env::args().nth(1).unwrap())
        .unwrap()
        .chars()
        .collect();

    let mut window_start: usize = 0;

    loop {
        if is_unique(&input[window_start..window_start + 4].to_vec()) {
            println!("{}", window_start + 4);
            return;
        }

        window_start += 1;
    }
}
