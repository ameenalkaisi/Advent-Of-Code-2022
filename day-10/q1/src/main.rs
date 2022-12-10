use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    // will use up a lot of memory, could probably write something that uses less
    // e.g., some wy to indicate the range of cycles where the val is at
    let mut cycle_val_at: Vec<i32> = vec![1];
    for line in reader.lines() {
        if let Err(_) = line {
            break;
        }

        let line = line.unwrap();

        let args: Vec<&str> = line.split(" ").collect();

        let last_val = cycle_val_at.last().unwrap().clone();
        match args[0] {
            "noop" => {
                cycle_val_at.push(last_val);
            }
            "addx" => {
                let new_val = args[1].parse::<i32>().unwrap();
                cycle_val_at.push(last_val);
                cycle_val_at.push(last_val + new_val);
            }
            _ => panic!("shouldn't happen"),
        }
    }

    // print sum of 20, 60, ..., 220
    // note that "during" the 20th means value at the "19th" cycle
    // since after the 20th means its now at the 21st cycle
    let mut sum: i32 = 0;
    for i in (20..=220).step_by(40) {
        sum += cycle_val_at[i - 1] * (i as i32);
    }

    println!("{sum}");
}
