use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
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

fn find_distress_beacon(max_coord_value: u32, sensors: &Vec<Sensor>) -> i64 {
    for y in 0 as i64..max_coord_value as i64 {
        let mut x: i64 = 0;

        while x <= max_coord_value as i64 {
            let coord = Coord {
                x: x as i64,
                y: y as i64,
            };

            // if you find a sensor such that the distance between the current point and the sensor
            // is less than or equal the sensor's range, then that sensor is blocking the current
            // location, and so we must skip this sensor
            // otherwise, if no sensor can sense our current location, then we've found the spot
            // and should report it
            if let Some(sensor) = sensors
                .iter()
                .find(|sensor| sensor.coord.man_distance(&coord) <= sensor.distance_to_beacon())
            {
                // skip the current sensor's range

                // this is using the c^2 = a^2 + b^2, except in manhatten distance
                // in manhatten distance, this would be |c| = |a| + |b|
                // so I did, |b| = |c| - |a|, then that would be the EDGE of the current diamond
                // that's formed, go 1 over to completely skip it
                x = (sensor.distance_to_beacon() - (y - sensor.coord.y).abs() as u32
                    + sensor.coord.x as u32
                    + 1) as i64;
            } else {
                return x as i64 * 4_000_000 + y as i64;
            }
        }
    }

    0
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
        find_distress_beacon(env::args().nth(2).unwrap().parse().unwrap(), &sensors)
    );
}
