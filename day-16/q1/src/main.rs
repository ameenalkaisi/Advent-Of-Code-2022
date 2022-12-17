use std::{
    cell::RefCell,
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

type ValvePtr = Rc<RefCell<Valve>>;

#[derive(Debug, Clone)]
struct Valve {
    is_open: bool,
    connecting_valves: Vec<ValvePtr>,
    label: String,
    flow_rate: i64,
}

impl Valve {
    pub fn new(label: String, connecting_valves: Vec<Rc<RefCell<Valve>>>, flow_rate: i64) -> Valve {
        Valve {
            connecting_valves,
            label,
            is_open: false,
            flow_rate,
        }
    }

    pub fn from_label_flowrate(label: String, flow_rate: i64) -> Valve {
        Valve {
            connecting_valves: vec![],
            label,
            is_open: false,
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

    let mut root: Option<ValvePtr> = None;
    for (i, connections) in connection_labels_in_order.iter_mut().enumerate() {
        let cur_valve = &mut valves_without_connections[i].clone();

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
    let mut path = vec![];

    println!("{}", Valve::to_string(root.clone()));
    println!(
        "{}",
        find_max_flow(root, 30, &mut HashMap::new(), &mut path)
    );
}

fn find_max_flow(
    valve_root: ValvePtr,
    minutes_left: i64,
    best_max_at: &mut HashMap<(String, Vec<String>, i64), i64>,
    path: &mut Vec<String>,
) -> i64 {
    if minutes_left <= 0 {
        return 0;
    }

    let cur_valve = valve_root.borrow();

    if let Some(&val) = best_max_at.get(&(cur_valve.label.clone(), path.clone(), minutes_left)) {
        return val;
    }

    // actions are 1. open current valve, 2. move into other valve, or 3. do nothing

    // when opening current, you could still move to other valve, or do nothing
    // also if you open current one, you can make moves in the next iteration
    let mut best: i64 = i64::MIN;
    for i in cur_valve.connecting_valves.iter() {
        if cur_valve.flow_rate > 0 && !path.contains(&cur_valve.label) {
            path.push(cur_valve.label.clone());

            let result = find_max_flow(i.clone(), minutes_left - 2, best_max_at, path);
            best = best.max(result + cur_valve.flow_rate * (minutes_left - 1));

            path.pop();
        }

        let result = find_max_flow(i.clone(), minutes_left - 1, best_max_at, path);
        best = best.max(result);
    }

    best_max_at.insert((cur_valve.label.clone(), path.clone(), minutes_left), best);
    best
}
