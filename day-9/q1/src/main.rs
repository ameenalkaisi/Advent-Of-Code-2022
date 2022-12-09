use std::{
    env,
    fs::File,
    io::BufRead,
    ops::{Add, AddAssign},
    str::FromStr,
};

enum Direction {
    Up,
    Left,
    Down,
    Right,
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct RopePair {
    head: Point,
    tail: Point,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        let result: Direction;
        match s {
            "R" => result = Right,
            "L" => result = Left,
            "U" => result = Up,
            "D" => result = Down,
            _ => panic!("not possible"),
        }

        Ok(result)
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl RopePair {
    pub fn new() -> Self {
        RopePair {
            head: Point::new(0, 0),
            tail: Point::new(0, 0),
        }
    }

    pub fn move_head(&mut self, dir: Direction, count: i32) -> Vec<Point> {
        let mut history: Vec<Point> = vec![];
        // iterate from 0 to count, going one at a time

        for _ in 0..count {
            let added_point: Point;
            match dir {
                Direction::Up => added_point = Point::new(0, 1),
                Direction::Left => added_point = Point::new(-1, 0),
                Direction::Down => added_point = Point::new(0, -1),
                Direction::Right => added_point = Point::new(1, 0),
            }

            self.head += added_point;

                if (self.head.x - self.tail.x).abs() > 1 || (self.head.y - self.tail.y).abs() > 1 {
                // move tail so that it goes towards head
                match dir {
                    Direction::Up => self.tail = Point::new(0, -1) + self.head.clone(),
                    Direction::Left => self.tail = Point::new(1, 0) + self.head.clone(),
                    Direction::Down => self.tail = Point::new(0, 1) + self.head.clone(),
                    Direction::Right => self.tail = Point::new(-1, 0) + self.head.clone(),
                }

                history.push(self.tail.clone());
            }
        }

        history
    }
}

fn main() {
    let file = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut visited: Vec<Point> = vec![Point::new(0, 0)];
    let mut rope = RopePair::new();

    for line in reader.lines() {
        if let Err(_) = line {
            break;
        }

        let line = line.unwrap();
        let (dir, count) = line.split_once(" ").unwrap();
        for i in rope.move_head(dir.parse().unwrap(), count.parse().unwrap()).iter() {
            if !visited.contains(i) {
                visited.push(i.clone());
            }
        }
    }

    println!("{}", visited.len());
}
