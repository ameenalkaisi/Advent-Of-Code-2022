use std::{
    env,
    fs::File,
    io::BufRead,
    ops::{Add, AddAssign, Neg},
    str::FromStr,
};

#[derive(Clone)]
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

/*
#[derive(Clone)]
struct RopePair {
    head: Point,
    tail: Point,
}
*/

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

    pub fn move_to(&mut self, p: &Point) {
        if self == p {
            return;
        }

        if (p.y - self.y).abs() <= 1 && (p.x - self.x).abs() <= 1 {
            return;
        } else if (p.y - self.y).abs() == 2 && (p.x - self.x).abs() == 2 {
            // this is the case where an element went diagonally
            // must go into it diagonally
            // based on diagonal direction, go through them to find out which way it went
            if p.y > self.y && p.x > self.x {
                *self += Point::new(1, 1);
            } else if p.y > self.y && p.x < self.x {
                *self += Point::new(-1, 1);
            } else if p.y < self.y && p.x > self.x {
                *self += Point::new(1, -1);
            } else {
                *self += Point::new(-1, -1);
            }
        } else if (p.y - self.y).abs() > 1 {
            if p.y - self.y > 1 {
                self.x = p.x;
                self.y = p.y - 1;
            } else {
                self.x = p.x;
                self.y = p.y + 1;
            }
        } else {
            // (p.x - self.x).abs() > 1
            if p.x - self.x > 1 {
                self.x = p.x - 1;
                self.y = p.y;
            } else {
                self.x = p.x + 1;
                self.y = p.y;
            }
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
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

/*impl RopePair {
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
}*/

struct Rope {
    // head is ropes[0].head, tail is ropes.last()?.tail
    knots: Vec<Point>,
}

impl Rope {
    pub fn new(knots: usize) -> Self {
        let mut knots_vec = Vec::new();
        (0..knots).for_each(|_| knots_vec.push(Point::new(0, 0)));

        Self { knots: knots_vec }
    }

    pub fn move_head(&mut self, dir: Direction, count: u32) -> Vec<Point> {
        let mut history: Vec<Point> = vec![];
        for _ in 0..count {
            // note: knots[0] is head
            self.knots[0] += match dir {
                Direction::Up => Point::new(0, 1),
                Direction::Down => Point::new(0, -1),
                Direction::Left => Point::new(-1, 0),
                Direction::Right => Point::new(1, 0),
            };

            let ropes_len = self.knots.len();
            for i in 1..ropes_len {
                let prev_knot = self.knots.get(i - 1).unwrap().clone();
                let cur_knot = self.knots.get_mut(i).unwrap();

                // head must move toward last rope's tail
                cur_knot.move_to(&prev_knot);

                // if it's self.ropes.len() - 1, add to history
                if i == ropes_len - 1 && !history.contains(&cur_knot) {
                    history.push(cur_knot.clone());
                }
            }

            //dbg!(&self.knots);
        }

        history
    }
}

//todo: idea is to have like 9 ropes, when head moves, call 1's move_head to the tail of head and
//so on
// should probably be array of 9 n stuff
fn main() {
    let file = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut visited: Vec<Point> = vec![Point::new(0, 0)];
    let mut rope = Rope::new(10);

    for line in reader.lines() {
        if let Err(_) = line {
            break;
        }

        let line = line.unwrap();
        let (dir, count) = line.split_once(" ").unwrap();
        for i in rope
            .move_head(dir.parse().unwrap(), count.parse().unwrap())
            .iter()
        {
            if !visited.contains(i) {
                visited.push(i.clone());
            }
        }
    }

    println!("{}", visited.len());
}
