use std::{cmp::Ordering, env, fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Value(u32),
}

impl FromStr for Packet {
    type Err = fmt::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line_chars = line.chars().collect::<Vec<char>>();
        let mut packet_list: Vec<Packet> = vec![];
        // go one by one until finding ']', if there is a list inside, just take substr and
        // make sure to count enough ]'s so that it takes the entire sublist
        // call parse() on it for recursion
        // add up all the packets into the vector
        let mut i = 1;
        let mut cur_num = String::new();

        loop {
            if line_chars[i] == '[' {
                let mut cur_packet_str = String::from("[");
                let mut bracket_count = 1;

                while bracket_count != 0 {
                    i += 1;
                    if line_chars[i] == '[' {
                        bracket_count += 1;
                    } else if line_chars[i] == ']' {
                        bracket_count -= 1;
                    }

                    cur_packet_str += &line_chars[i].to_string();
                }

                // recursively parses the inner list
                packet_list.push(cur_packet_str.parse().unwrap());

                i += 1;
                continue;
            }

            if line_chars[i].is_numeric() {
                cur_num += &line_chars[i].to_string();
            }

            if line_chars[i] == ',' || line_chars[i] == ']' {
                if !cur_num.is_empty() {
                    packet_list.push(Packet::Value(cur_num.parse().unwrap()));
                }

                cur_num.clear();
            }

            if line_chars[i] == ']' {
                break;
            }

            i += 1;
        }

        return Ok(Packet::List(packet_list));
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match &self {
            Packet::List(packets) => match other {
                Packet::List(other_packets) => {
                    for i in 0..packets.len() {
                        // if the other list ran out, this list is greater than the other one
                        if i >= other_packets.len() {
                            return Some(Ordering::Greater);
                        }

                        let result = packets[i].cmp(&other_packets[i]);
                        if let Ordering::Greater = result {
                            return Some(Ordering::Greater);
                        } else if let Ordering::Less = result {
                            return Some(Ordering::Less);
                        }
                    }

                    // if passed all elements of the current list and
                    // no vital copmarison has been found, then it is less
                    if packets.len() < other_packets.len() {
                        return Some(Ordering::Less);
                    } else {
                        return Some(Ordering::Equal);
                    }
                }
                Packet::Value(other_val) => {
                    let comparison_packet = Packet::List(vec![Packet::Value(*other_val)]);

                    return self.partial_cmp(&comparison_packet);
                }
            },
            Packet::Value(val) => match other {
                Packet::Value(other_val) => val.partial_cmp(other_val),
                _ => {
                    let comparison_packet = Packet::List(vec![Packet::Value(*val)]);

                    return comparison_packet.partial_cmp(&other);
                }
            },
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(&other).unwrap();
    }
}

fn main() {
    let input = std::fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let input: Vec<&str> = input.lines().collect();

    let mut pair_sum: u32 = 0;

    let mut i = 0;
    // always at least one value in the given inputs
    loop {
        let first_packet: Packet = input[i].parse().unwrap();
        let second_packet: Packet = input[i + 1].parse().unwrap();

        /*
        println!(
            "{:?}, {:?}, {:?}",
            first_packet,
            second_packet,
            first_packet.cmp(&second_packet)
        );
        */

        if let Ordering::Less = first_packet.cmp(&second_packet) {
            pair_sum += i as u32 / 3 + 1;
            println!("{}", i as u32 / 3 + 1);
        }

        i += 3;
        if i >= input.len() {
            break;
        }
    }

    println!("indices sum: {}", pair_sum);
}
