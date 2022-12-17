use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

type ValvePtr = Rc<RefCell<Valve>>;

#[derive(Debug, Clone)]
struct Valve {
    connecting_valves: Vec<ValvePtr>,
    label: String,
    flow_rate: i64,
}

impl Valve {
    pub fn new(label: String, connecting_valves: Vec<Rc<RefCell<Valve>>>, flow_rate: i64) -> Valve {
        Valve {
            connecting_valves,
            label,
            flow_rate,
        }
    }

    pub fn from_label_flowrate(label: String, flow_rate: i64) -> Valve {
        Valve {
            connecting_valves: vec![],
            label,
            flow_rate,
        }
    }

    pub fn to_string(self_: ValvePtr) -> String {
        let mut visited: Vec<String> = vec![];
        Valve::to_string_(self_.clone(), &mut visited)
    }

    pub fn to_string_(self_: ValvePtr, visited: &mut Vec<String>) -> String {
        if visited.iter().any(|label| self_.borrow().label == *label) {
            return String::new();
        }

        let mut result: String = String::new();
        visited.push(self_.borrow().label.clone());

        result += &format!("\n{} => ", self_.borrow().label).to_string();

        for i in self_.borrow().connecting_valves.iter() {
            result += &format!("{} ", i.borrow().label).to_string();
        }

        result += &self_
            .borrow()
            .connecting_valves
            .iter()
            .map(|valve| Valve::to_string_(valve.clone(), visited))
            .reduce(|accum, next| accum + &next)
            .unwrap();

        result
    }
}

fn find_all_distances(valves: HashMap<String, ValvePtr>) -> HashMap<(String, String), i64> {
    let mut result = HashMap::new();

    // initailize self and to connecting
    valves.keys().for_each(|label| {
        result.insert((label.clone(), label.clone()), 0);
    });

    valves.iter().for_each(|(label, valve)| {
        valve
            .borrow()
            .connecting_valves
            .iter()
            .for_each(|connecting_valve| {
                result.insert((label.clone(), connecting_valve.borrow().label.clone()), 1);
                result.insert((connecting_valve.borrow().label.clone(), label.clone()), 1);
            });
    });

    for u in valves.keys() {
        for v1 in valves.keys() {
            for v2 in valves.keys() {
                let v1_v2 = (v1.clone(), v2.clone());
                let v1_u = (v1.clone(), u.clone());
                let u_v2 = (u.clone(), v2.clone());

                let v1_u_v2_result = result
                    .get(&v1_u)
                    .copied()
                    .unwrap_or(i64::MAX)
                    .saturating_add(result.get(&u_v2).copied().unwrap_or(i64::MAX));

                if result.get(&v1_v2).copied().unwrap_or(i64::MAX) > v1_u_v2_result {
                    result.insert(v1_v2, v1_u_v2_result);
                }
            }
        }
    }

    for valve in valves.keys() {
        result.remove(&(valve.clone(), valve.clone()));
    }

    result
}

fn find_all_paths(
    start: &String,
    valves: &HashMap<String, ValvePtr>,
    path_used: i64,
    path_so_far: Vec<String>,
    pressure_so_far: i64,
    distances: &HashMap<(String, String), i64>,
    used: &mut HashSet<String>,
    paths: &mut HashMap<Vec<String>, i64>,
    total_time: i64,
) {
    used.insert(start.clone());

    let pressure_relieved =
        (total_time - (path_used + 1)) * valves.get(start).unwrap().borrow().flow_rate;
    paths.insert(path_so_far.clone(), pressure_so_far + pressure_relieved);

    for valve in valves.keys() {
        if !used.contains(valve) {
            let distance = distances
                .get(&(start.clone(), valve.clone()))
                .copied()
                .unwrap();

            if distance + path_used < total_time {
                find_all_paths(
                    valve,
                    valves,
                    distance + path_used + 1,
                    {
                        let mut path = path_so_far.clone();
                        path.push(valve.to_string());
                        path
                    },
                    pressure_so_far + pressure_relieved,
                    distances,
                    &mut used.clone(),
                    paths,
                    total_time,
                );
            }
        }
    }
}

fn main() {
    let file = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file).lines();

    // read once to get all the input, then read again to fill in the connections
    let mut valves_without_connections: Vec<ValvePtr> = vec![];
    let mut connection_labels_in_order: Vec<Vec<String>> = vec![];
    for line in reader.flatten() {
        let line = line.split(' ').collect::<Vec<&str>>();

        let label = line[1];
        let flow_rate = line[4]
            .split_once('=')
            .unwrap()
            .1
            .split(';')
            .nth(0)
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let connected_labels = line[9..]
            .iter()
            .map(|label| label.trim().split(',').nth(0).unwrap().to_string())
            .collect::<Vec<String>>();

        connection_labels_in_order.push(connected_labels);

        valves_without_connections.push(Rc::new(RefCell::new(Valve::from_label_flowrate(
            label.to_string(),
            flow_rate,
        ))));
    }

    let mut all_valves: HashMap<String, ValvePtr> = HashMap::new();

    let mut root: Option<ValvePtr> = None;
    for (i, connections) in connection_labels_in_order.iter_mut().enumerate() {
        let cur_valve = &mut valves_without_connections[i].clone();

        all_valves.insert(cur_valve.borrow().label.clone(), cur_valve.clone());
        for i in connections {
            let connected_valve = valves_without_connections
                .iter()
                .find(|x| x.borrow().label == *i)
                .unwrap()
                .clone();

            cur_valve
                .borrow_mut()
                .connecting_valves
                .push(connected_valve.clone());

            if cur_valve.borrow().label == "AA" {
                root = Some(cur_valve.clone());
            }
        }
    }

    // this contains element with label "AA"
    let root = root.unwrap();

    let mut paths = HashMap::new();
    find_all_paths(
        &root.borrow().label,
        &all_valves,
        0,
        vec![],
        0,
        &find_all_distances(all_valves.clone()),
        &mut HashSet::new(),
        &mut paths,
        26,
    );

    let all_paths: Vec<(HashSet<String>, i64)> = paths
        .iter()
        .map(|(vec, value)| (vec.iter().map(|f| f.clone()).collect(), *value))
        .collect();

    let mut best: i64 = i64::MIN;
    for (path, cost) in all_paths.iter() {
        for (path2, cost2) in all_paths.iter() {
            if path.is_disjoint(path2) {
                best = best.max(cost + cost2);
            }
        }
    }

    println!("{}", best);
}
