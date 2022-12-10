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

    // we start "during" the first cycle
    let mut cur_cycle: usize = 1;

    // height of 6
    for _ in 0..6 {
        // width of 40
        for cur_width in 0..40 {
            // sprite string 40 char long, for each range, create the CRT row, display it
            // CRT row draws on where the sprite is
            // go one row at a time
            let sprite_middle = cycle_val_at[cur_cycle - 1];

            // if crt_pos lies in the middle of the sprite, then draw a pixel, otherwise a blank
            if sprite_middle - 1 <= cur_width as i32 && cur_width as i32 <= sprite_middle + 1 {
                print!("#")
            } else {
                print!(".");
            }

            cur_cycle += 1;
        }

        println!();
    }
}
