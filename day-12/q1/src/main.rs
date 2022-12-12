use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    env,
    rc::Rc,
};

fn main() {
    let input = std::fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let mut start_point: Option<(usize, usize)> = None;
    let matrix: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let chars = line.chars();
            let result = chars.clone();
            if let None = start_point {
                chars.enumerate().for_each(|(j, val)| {
                    if val == 'S' {
                        start_point = Some((i as usize, j as usize));
                    }
                });
            }

            result.collect::<Vec<char>>()
        })
        .collect();

    let path = find_shortest_path(&matrix, start_point.unwrap(), 'E');

    // don't show starting point as a move
    println!("{}", path.len() - 1);
}

struct SearchNode {
    parent: Option<Rc<RefCell<SearchNode>>>,
    point: (usize, usize),
}

// BFS of the spot
fn find_shortest_path(
    matrix: &Vec<Vec<char>>,
    start_point: (usize, usize),
    final_val: char,
) -> Vec<(usize, usize)> {
    let mut history: Vec<(i32, i32)> = vec![];

    let mut queue = VecDeque::new();
    queue.push_back(start_point);

    let mut index_node: HashMap<(usize, usize), Rc<RefCell<SearchNode>>> = HashMap::new();
    index_node.insert(
        start_point,
        Rc::new(RefCell::new(SearchNode {
            parent: None,
            point: start_point,
        })),
    );

    let moves_to_check: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    while !queue.is_empty() {
        let cur_point = queue.pop_front().unwrap();
        let cur_point_i32 = (cur_point.0 as i32, cur_point.1 as i32);

        // add to some kind of history tracking thing maybe search tree structure
        history.push(cur_point_i32);

        // check for final node
        if matrix[cur_point.0 as usize][cur_point.1 as usize] == final_val {
            // get path from the tree structure
            let mut result_path: Vec<(usize, usize)> = vec![];

            let mut cur_node = index_node.get(&cur_point).unwrap().clone();

            loop {
                result_path.push(cur_node.borrow().point);

                if let None = &cur_node.borrow().parent {
                    break;
                }

                let parent = &cur_node.borrow().parent.as_ref().unwrap().clone();

                cur_node = parent.clone();
            }

            result_path.reverse();
            return result_path;
        }

        for (i, j) in moves_to_check.iter() {
            let moved_point = (cur_point_i32.0 + i, cur_point_i32.1 + j);
            if moved_point.0 >= 0
                && moved_point.1 >= 0
                && moved_point.0 < *&matrix.len() as i32
                && moved_point.1 < *&matrix[0].len() as i32
                && !history.contains(&moved_point)
            {
                let moved_point_usize = (moved_point.0 as usize, moved_point.1 as usize);

                let mut parent_val = matrix[cur_point.0][cur_point.1];
                let mut cur_value = matrix[moved_point_usize.0][moved_point_usize.1];

                if cur_value == 'E' {
                    cur_value = 'z';
                }

                if parent_val == 'S' {
                    parent_val = 'a';
                }

                if (cur_value != 'E' && parent_val != 'S' && cur_value as i8 - parent_val as i8 > 1)
                    || queue.contains(&moved_point_usize)
                {
                    continue;
                }

                let parent_node = index_node
                    .get(&cur_point)
                    .expect(&format!("{}, {}", cur_point.0, cur_point.1))
                    .clone();

                let new_node = Rc::new(RefCell::new(SearchNode {
                    parent: Some(parent_node),
                    point: moved_point_usize,
                }));

                index_node.insert(moved_point_usize, new_node);

                queue.push_back(moved_point_usize);
            }
        }
    }

    vec![]
}
