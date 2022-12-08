use std::env;

fn main() {
    let file = std::fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    // u8 since it will only be 1 digit at most
    // note: u8 goes from 0 to 255 iirc
    let mut tree_matrix: Vec<Vec<u8>> = vec![];

    for line in file.lines() {
        tree_matrix.push(vec![]);
        for ch in line.chars() {
            tree_matrix.last_mut().unwrap().push(ch as u8 - '0' as u8);
        }
    }

    let mut total: u32 = 0;
    for i in 0..tree_matrix.len() {
        for j in 0..tree_matrix[0].len() {
            if is_visible(&tree_matrix, i, j) {
                total += 1;
            }
        }
    }

    println!("{}", total);
}

fn is_visible(tree_matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    // go through all directions, see if they are visible
    // left first
    for cur_range in vec![0..i, i + 1..tree_matrix.len()] {
        let mut is_visible = true;
        for k in cur_range {
            if tree_matrix[i][j] <= tree_matrix[k][j] {
                is_visible = false;
                break;
            }
        }

        if is_visible {
            return true;
        }
    }

    for cur_range in vec![0..j, j + 1..tree_matrix[0].len()] {
        let mut is_visible = true;
        for k in cur_range {
            if tree_matrix[i][j] <= tree_matrix[i][k] {
                is_visible = false;
                break;
            }
        }

        if is_visible {
            return true;
        }
    }

    false
}
