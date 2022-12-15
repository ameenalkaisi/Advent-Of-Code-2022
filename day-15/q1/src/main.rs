use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    coord: Coord,
    closest_beacon: Beacon,
}

#[derive(Debug)]
struct Beacon {
    coord: Coord,
}

impl Coord {
    pub fn man_distance(&self, other: &Coord) -> u32 {
        return ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32;
    }
}

impl Sensor {
    pub fn distance_to_beacon(&self) -> u32 {
        self.coord.man_distance(&self.closest_beacon.coord)
    }
}

fn get_points_at_y(y_level: i32, sensors: &Vec<Sensor>) -> u32 {
    let mut blocked_points_at_y: HashSet<Coord> = HashSet::new();

    for sensor in sensors.iter() {
        let sensor_dist = sensor.distance_to_beacon();
        let width = 2*(sensor.coord.x - sensor.closest_beacon.coord.x).abs();

        // HashSet.insert only inserts if it's not duplicated

        for i in -width..=width {
            let current_point = Coord {
                x: sensor.coord.x + i as i32,
                y: y_level,
            };

            // don't count if it's a sensor, or the closest beacon
            if current_point != sensor.closest_beacon.coord
                && !sensors.iter().any(|sensor| sensor.coord == current_point)
                && current_point.man_distance(&sensor.coord) <= sensor_dist
            {
                blocked_points_at_y.insert(current_point);
            }
        }
    }

    blocked_points_at_y.len() as u32
}

fn main() {
    let file = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file).lines();

    let mut sensors: Vec<Sensor> = vec![];
    for line in reader.flatten() {
        let line: Vec<&str> = line.split(' ').collect();

        let sensor_coord = Coord {
            x: line[2]
                .split_once('=')
                .unwrap()
                .1
                .split(',')
                .nth(0)
                .unwrap()
                .parse()
                .unwrap(),
            y: line[3]
                .split_once('=')
                .unwrap()
                .1
                .split(':')
                .nth(0)
                .unwrap()
                .parse()
                .unwrap(),
        };

        let beacon_coord = Coord {
            x: line[8]
                .split_once('=')
                .unwrap()
                .1
                .split(',')
                .nth(0)
                .unwrap()
                .parse()
                .unwrap(),
            y: line[9].split_once('=').unwrap().1.parse().unwrap(),
        };

        sensors.push(Sensor {
            coord: sensor_coord,
            closest_beacon: Beacon {
                coord: beacon_coord,
            },
        });
    }

    println!(
        "{}",
        get_points_at_y(env::args().nth(2).unwrap().parse().unwrap(), &sensors)
    );
}
