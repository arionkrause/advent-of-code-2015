use regex::Regex;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

struct Wire {
    gate: Gate,
    input_value: Option<u16>,
    input_wire_first: Option<String>,
    input_wire_second: Option<String>,
    value: Option<u16>,
}

enum Gate {
    Andi,
    Andr,
    Lshift,
    Not,
    Ori,
    Orr,
    Rshift,
    Seti,
    Setr,
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, "a"));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn decode_input(input: &str) -> HashMap<String, Rc<RefCell<Wire>>> {
    let re_seti = Regex::new(r"^(?P<input_value>\d+) -> (?P<id>[a-z]+)$").unwrap();
    let re_setr = Regex::new(r"^(?P<input_wire_first>[a-z]+) -> (?P<id>[a-z]+)$").unwrap();
    let re_andi_ori = Regex::new(r"^(?P<input_value>\d+) (?P<gate>AND|OR) (?P<input_wire_first>[a-z]+) -> (?P<id>[a-z]+)$").unwrap();
    let re_andr_orr = Regex::new(r"^(?P<input_wire_first>[a-z]+) (?P<gate>AND|OR) (?P<input_wire_second>[a-z]+) -> (?P<id>[a-z]+)$").unwrap();
    let re_lshift_rshift = Regex::new(r"^(?P<input_wire_first>[a-z]+) (?P<gate>[LR]SHIFT) (?P<input_value>\d+) -> (?P<id>[a-z]+)$").unwrap();
    let re_not = Regex::new(r"^NOT (?P<input_wire_first>[a-z]+) -> (?P<id>[a-z]+)$").unwrap();
    let mut wires = HashMap::new();

    for line in input.lines() {
        if let Some(captures) = re_seti.captures(&line) {
            let input_value = captures.name("input_value").unwrap().as_str().parse().unwrap();
            let id = captures.name("id").unwrap().as_str().to_string();

            wires.insert(id, Rc::new(RefCell::new(Wire {
                gate: Gate::Seti,
                input_value: Some(input_value),
                input_wire_first: None,
                input_wire_second: None,
                value: None,
            })));

            continue;
        }

        if let Some(captures) = re_setr.captures(&line) {
            let input_wire_first = captures.name("input_wire_first").unwrap().as_str().to_string();
            let id = captures.name("id").unwrap().as_str().to_string();

            wires.insert(id, Rc::new(RefCell::new(Wire {
                gate: Gate::Setr,
                input_value: None,
                input_wire_first: Some(input_wire_first),
                input_wire_second: None,
                value: None,
            })));

            continue;
        }

        if let Some(captures) = re_andi_ori.captures(&line) {
            let input_value = captures.name("input_value").unwrap().as_str().parse().unwrap();
            let input_wire_first = captures.name("input_wire_first").unwrap().as_str().to_string();
            let gate_in_word = captures.name("gate").unwrap().as_str();
            let id = captures.name("id").unwrap().as_str().to_string();

            let gate = match gate_in_word {
                "AND" => Gate::Andi,
                "OR" => Gate::Ori,
                _ => panic!(),
            };

            wires.insert(id, Rc::new(RefCell::new(Wire {
                gate,
                input_value: Some(input_value),
                input_wire_first: Some(input_wire_first),
                input_wire_second: None,
                value: None,
            })));

            continue;
        }

        if let Some(captures) = re_andr_orr.captures(&line) {
            let input_wire_first = captures.name("input_wire_first").unwrap().as_str().to_string();
            let input_wire_second = captures.name("input_wire_second").unwrap().as_str().to_string();
            let gate_in_word = captures.name("gate").unwrap().as_str();
            let id = captures.name("id").unwrap().as_str().to_string();

            let gate = match gate_in_word {
                "AND" => Gate::Andr,
                "OR" => Gate::Orr,
                _ => panic!(),
            };

            wires.insert(id, Rc::new(RefCell::new(Wire {
                gate,
                input_value: None,
                input_wire_first: Some(input_wire_first),
                input_wire_second: Some(input_wire_second),
                value: None,
            })));

            continue;
        }

        if let Some(captures) = re_lshift_rshift.captures(&line) {
            let input_wire_first = captures.name("input_wire_first").unwrap().as_str().to_string();
            let input_value = captures.name("input_value").unwrap().as_str().parse().unwrap();
            let gate_in_word = captures.name("gate").unwrap().as_str();
            let id = captures.name("id").unwrap().as_str().to_string();

            let gate = match gate_in_word {
                "LSHIFT" => Gate::Lshift,
                "RSHIFT" => Gate::Rshift,
                _ => panic!(),
            };

            wires.insert(id, Rc::new(RefCell::new(Wire {
                gate,
                input_value: Some(input_value),
                input_wire_first: Some(input_wire_first),
                input_wire_second: None,
                value: None,
            })));

            continue;
        }

        if let Some(captures) = re_not.captures(&line) {
            let input_wire_first = captures.name("input_wire_first").unwrap().as_str().to_string();
            let id = captures.name("id").unwrap().as_str().to_string();

            wires.insert(id, Rc::new(RefCell::new(Wire {
                gate: Gate::Not,
                input_value: None,
                input_wire_first: Some(input_wire_first),
                input_wire_second: None,
                value: None,
            })));

            continue;
        }

        panic!();
    }

    wires
}

fn get_value(wires: &HashMap<String, Rc<RefCell<Wire>>>, id: &str) -> Option<u16> {
    if let Some(wire) = wires.get(id) {
        if wire.borrow().value.is_none() {
            let new_value = match wire.borrow().gate {
                Gate::Andi => {
                    Some(wire.borrow().input_value.unwrap() & get_value(&wires, &wire.borrow().input_wire_first.clone().unwrap()).unwrap())
                }
                Gate::Andr => {
                    Some(get_value(&wires, &wire.borrow().input_wire_first.clone().unwrap()).unwrap() & get_value(&wires, &wire.borrow().input_wire_second.clone().unwrap()).unwrap())
                }
                Gate::Lshift => {
                    Some(get_value(&wires, &wire.borrow().input_wire_first.clone().unwrap()).unwrap() << wire.borrow().input_value.clone().unwrap())
                }
                Gate::Not => {
                    Some(!(get_value(&wires, &wire.borrow().input_wire_first.clone().unwrap()).unwrap()))
                }
                Gate::Ori => {
                    Some(wire.borrow().input_value.unwrap() | get_value(&wires, &wire.borrow().input_wire_first.clone().unwrap()).unwrap())
                }
                Gate::Orr => {
                    Some(get_value(&wires, &wire.borrow().input_wire_first.clone().unwrap()).unwrap() | get_value(&wires, &wire.borrow().input_wire_second.clone().unwrap()).unwrap())
                }
                Gate::Rshift => {
                    Some(get_value(&wires, &wire.borrow().input_wire_first.clone().unwrap()).unwrap() >> wire.borrow().input_value.clone().unwrap())
                }
                Gate::Seti => {
                    wire.borrow().input_value
                }
                Gate::Setr => {
                    Some(get_value(&wires, &wire.borrow().input_wire_first.clone().unwrap()).unwrap())
                }
            };

            wire.borrow_mut().value = new_value;
        }

        wire.borrow().value
    } else {
        None
    }
}

mod part_1 {
    use crate::day_7::decode_input;
    use crate::day_7::get_value;

    pub fn solve(input: &str, id: &str) -> u16 {
        let wires = decode_input(&input);
        get_value(&wires, id).unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

        assert_eq!(solve(&input, "d"), 72);
        assert_eq!(solve(&input, "e"), 507);
        assert_eq!(solve(&input, "f"), 492);
        assert_eq!(solve(&input, "g"), 114);
        assert_eq!(solve(&input, "h"), 65412);
        assert_eq!(solve(&input, "i"), 65079);
        assert_eq!(solve(&input, "x"), 123);
        assert_eq!(solve(&input, "y"), 456);
    }
}

mod part_2 {
    use crate::day_7::decode_input;
    use crate::day_7::get_value;

    pub fn solve(input: &str) -> u16 {
        let wires = decode_input(&input);
        let value_a = get_value(&wires, "a");
        wires.values().for_each(|w| w.borrow_mut().value = None);
        wires.get("b").unwrap().borrow_mut().value = value_a;
        get_value(&wires, "a").unwrap()
    }
}
