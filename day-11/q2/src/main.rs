use std::env;

#[derive(Debug)]
enum OperationValue {
    Value(u64),
    Old,
}

#[derive(Debug)]
enum Operation {
    Plus,
    Times,
}

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<u64>,
    operation_type: Operation,
    operation_val: OperationValue,
    divisble_by: u32,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: u64,
}

struct ThrownItem {
    monkey_index: usize,
    item: u64,
}

// todo, throwing mechanics

impl Monkey {
    pub fn inspect_and_get_monkey_order(&mut self, common_multiple: u64) -> Vec<ThrownItem> {
        let mut thrown_items: Vec<ThrownItem> = vec![];

        let mut starting_items = self.starting_items.clone();
        self.starting_items.clear();

        for i in starting_items.iter_mut() {
            self.inspect_count += 1;
            *i = self.exec_operation(*i);

            *i = *i % common_multiple;

            thrown_items.push(ThrownItem {
                monkey_index: self.which_monkey(*i),
                item: *i,
            });
        }

        thrown_items
    }

    fn exec_operation(&self, old: u64) -> u64 {
        let operation_val = match self.operation_val {
            OperationValue::Value(x) => x,
            OperationValue::Old => old,
        };

        match self.operation_type {
            Operation::Plus => operation_val + old,
            Operation::Times => operation_val * old,
        }
    }

    fn which_monkey(&self, item: u64) -> usize {
        if item % (self.divisble_by as u64) == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

fn parse_monkeys(lines: Vec<&str>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    let mut i = 0;
    loop {
        let mut starting_items: Vec<u64> = vec![];
        let mut operation_type: Operation;
        let mut divisble_by: u32;
        let mut operation_val: OperationValue;
        let mut true_monkey: usize;
        let mut false_monkey: usize;

        let first = lines[i + 1].trim().split_once(':').unwrap();

        starting_items = first
            .1
            .split(',')
            .map(|bruh| bruh.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let second = lines[i + 2].trim().split(' ').collect::<Vec<&str>>();
        operation_type = match second[4] {
            "*" => Operation::Times,
            "+" => Operation::Plus,
            _ => panic!("should only be times or plus"),
        };

        if second[5] == "old" {
            operation_val = OperationValue::Old;
        } else {
            operation_val = OperationValue::Value(second[5].parse().unwrap());
        }

        divisble_by = lines[i + 3]
            .trim()
            .split(' ')
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();

        true_monkey = lines[i + 4]
            .trim()
            .split(' ')
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();
        false_monkey = lines[i + 5]
            .trim()
            .split(' ')
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();

        monkeys.push(Monkey {
            starting_items,
            operation_type,
            operation_val,
            divisble_by,
            true_monkey,
            false_monkey,
            inspect_count: 0,
        });

        i += 7;
        if i >= lines.len() {
            break;
        }
    }

    monkeys
}

fn simulate_round(monkeys: &mut Vec<Monkey>, common_multiple: u64) {
    for i in 0..monkeys.len() {
        let cur_monkey = &mut monkeys[i];

        let thrown_monkeys = cur_monkey.inspect_and_get_monkey_order(common_multiple);

        // num of items thrown equals num of items inspected

        for ThrownItem { monkey_index, item } in thrown_monkeys.iter() {
            monkeys[*monkey_index].starting_items.push(*item);
        }
    }
}

fn main() {
    let input = std::fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    // parsing the monkeys

    let mut monkeys = parse_monkeys(lines);
    let common_multiple: u64 = monkeys.iter().map(|monkey| monkey.divisble_by as u64).product();

    for _ in 0..10_000 {
        simulate_round(&mut monkeys, common_multiple);
    }

    let mut top_two = (0, 0);
    for monkey in monkeys {
        let items_inspected = monkey.inspect_count;
        if items_inspected > top_two.0 {
            top_two.0 = items_inspected;
        } else if items_inspected > top_two.1 {
            top_two.1 = items_inspected;
        }

        if top_two.0 > top_two.1 {
            std::mem::swap(&mut top_two.1, &mut top_two.0);
        }
    }

    println!("{}", top_two.0 * top_two.1);
}
