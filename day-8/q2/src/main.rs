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

    let mut max: u32 = u32::MIN;
    for i in 0..tree_matrix.len() {
        for j in 0..tree_matrix[0].len() {
            let cur = get_scenic_score(&tree_matrix, i, j);
            if max < cur {
                max = cur;
            }
        }
    }

    println!("{}", max);
}

fn get_scenic_score(tree_matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> u32 {
    // go through all directions, see if they are visible
    // left first
    let mut result_score: u32 = 1;

    let mut cur_score: u32 = 0;
    for k in (0..i).rev() {
        cur_score += 1;

        if tree_matrix[i][j] <= tree_matrix[k][j] {
            break;
        }
    }

    result_score *= cur_score;

    let mut cur_score: u32 = 0;
    for k in i + 1..tree_matrix.len() {
        cur_score += 1;

        if tree_matrix[i][j] <= tree_matrix[k][j] {
            break;
        }
    }

    result_score *= cur_score;

    let mut cur_score: u32 = 0;
    for k in (0..j).rev() {
        cur_score += 1;

        if tree_matrix[i][j] <= tree_matrix[i][k] {
            break;
        }
    }

    result_score *= cur_score;

    let mut cur_score: u32 = 0;
    for k in j + 1..tree_matrix.len() {
        cur_score += 1;

        if tree_matrix[i][j] <= tree_matrix[i][k] {
            break;
        }
    }

    result_score *= cur_score;

    result_score
}
