use std::{
    env,
    fmt::Error,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        if start.x > end.x || start.y > end.y {
            return Line {
                start: end,
                end: start,
            };
        }

        Line { start, end }
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        (point.x >= self.start.x && point.x <= self.end.x && point.y == self.start.y)
            || (point.y >= self.start.y && point.y <= self.end.y && point.x == self.start.x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();

        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();

        Ok(Point::new(x, y))
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        return Point::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}

fn is_blocked(spot: Point, obstacle_lines: &Vec<Line>) -> bool {
    for i in obstacle_lines.iter() {
        if i.contains_point(&spot) {
            return true;
        }
    }

    false
}

#[derive(Debug)]
enum DescendResult {
    Infinity,
    Value(i32),
}

fn get_descent_result(point: &Point, obstacle_lines: &Vec<Line>) -> DescendResult {
    let mut result = DescendResult::Infinity;

    // get the highest y-value obstacle found
    for obstacle in obstacle_lines {
        // if any of the rocks are vertically over the point
        // and the point is within x-distance of that line, then
        // they are bound to hit
        if (obstacle.start.y >= point.y || obstacle.end.y >= point.y)
            && (obstacle.start.x <= point.x && point.x <= obstacle.end.x)
        {
            let top_point = if obstacle.start.y > obstacle.end.y {
                obstacle.end.y
            } else {
                obstacle.start.y
            };

            // top_point is the top_most floor of the point
            let diff = top_point - point.y;
            if let DescendResult::Value(best_y) = result {
                if best_y > diff {
                    result = DescendResult::Value(diff);
                }
            } else if let DescendResult::Infinity = result {
                result = DescendResult::Value(diff);
            }
        }
    }

    result
}

// will return true if it ends up going down forever, otherwise return false
fn simulate_until_rest(point: &mut Point, obstacle_lines: &mut Vec<Line>) -> bool {
    loop {
        let down_move = get_descent_result(&point, &obstacle_lines);

        match down_move {
            DescendResult::Infinity => return true,
            DescendResult::Value(x) => {
                if x == 1 {
                    if !is_blocked(point.clone() + (-1, 1).into(), &obstacle_lines) {
                        *point += (-1, 1).into();
                        continue;
                    } else if !is_blocked(point.clone() + (1, 1).into(), &obstacle_lines) {
                        *point += (1, 1).into();
                    } else {
                        // it is rested now, push it as a singular point obstacle line
                        obstacle_lines.push(Line::new(*point, *point));
                        return false;
                    }
                } else {
                    *point += (0, x - 1).into();
                }
            }
        } // end match
    } // end loop
}

// note in (x, y), +y means downward
// i could use a hash map and figure things out but im afraid the input might be really large
fn main() {
    let input = std::fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let mut obstacle_lines: Vec<Line> = vec![];
    for line in input.lines() {
        let mut points: Vec<Point> = vec![];
        for point in line.split("->").map(|pair| pair.trim()) {
            let point: Point = point.parse().unwrap();
            points.push(point);
        }

        // will have some copies here
        let mut prev_point_index = 0;
        for i in 1..points.len() {
            obstacle_lines.push(Line::new(points[prev_point_index], points[i]));
            prev_point_index = i;
        }
    }

    // start simulation here
    let sand_start = Point::new(500, 0);

    let mut iter: u32 = 0;
    while !simulate_until_rest(&mut sand_start.clone(), &mut obstacle_lines) {
        iter += 1;
    }

    println!("{}", iter);
}
