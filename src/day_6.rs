use regex::Regex;

#[derive(Debug)]
struct Instruction {
    command: Command,
    from: Position,
    to: Position,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum State {
    On,
    Off,
}

pub fn solve(input: &str) -> (Option<usize>, Option<i32>) {
    print!("Day {}", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    let instructions = decode_input(&input);
    let mut part_1 = Some(part_1(&instructions));
    let mut part_2 = Some(part_2(&instructions));

    if part_1.is_some() {
        print!(" - Part 1: {}.", part_1.unwrap());
    }

    if part_2.is_some() {
        println!(" Part 2: {}.", part_2.unwrap());
    }

    (part_1, part_2)
}

fn part_1(instructions: &Vec<Instruction>) -> usize {
    let mut grid = vec![vec![State::Off; 1000]; 1000];

    for instruction in instructions {
        for y in instruction.from.y..=instruction.to.y {
            for x in instruction.from.x..=instruction.to.x {
                grid[y][x] = match instruction.command {
                    Command::TurnOn => State::On,
                    Command::TurnOff => State::Off,
                    Command::Toggle => {
                        match grid[y][x] {
                            State::On => State::Off,
                            State::Off => State::On,
                        }
                    }
                };
            }
        }
    }

    grid.iter().map(|row| {
        row.iter().filter(|&state| state == &State::On).count()
    }).sum()
}

fn part_2(instructions: &Vec<Instruction>) -> i32 {
    let mut grid = vec![vec![0; 1000]; 1000];

    for instruction in instructions {
        for y in instruction.from.y..=instruction.to.y {
            for x in instruction.from.x..=instruction.to.x {
                match instruction.command {
                    Command::TurnOn => grid[y][x] += 1,
                    Command::TurnOff => {
                        if grid[y][x] > 0 {
                            grid[y][x] -= 1;
                        }
                    }
                    Command::Toggle => grid[y][x] += 2,
                };
            }
        }
    }

    grid.iter().map(|row| {
        row.iter().sum::<i32>()
    }).sum()
}

fn decode_input(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(?P<command>(turn (on|off)|toggle)) (?P<from_x>\d+),(?P<from_y>\d+) through (?P<to_x>\d+),(?P<to_y>\d+)").unwrap();
    let mut instructions = Vec::new();

    for line in input.lines() {
        let captures = re.captures(&line).unwrap();

        let command = match captures.name("command").unwrap().as_str() {
            "turn on" => Command::TurnOn,
            "turn off" => Command::TurnOff,
            "toggle" => Command::Toggle,
            _ => panic!(),
        };

        let from = Position {
            x: captures.name("from_x").unwrap().as_str().parse::<usize>().unwrap(),
            y: captures.name("from_y").unwrap().as_str().parse::<usize>().unwrap(),
        };

        let to = Position {
            x: captures.name("to_x").unwrap().as_str().parse::<usize>().unwrap(),
            y: captures.name("to_y").unwrap().as_str().parse::<usize>().unwrap(),
        };

        instructions.push(Instruction {
            command,
            from,
            to,
        });
    }

    instructions
}

#[cfg(test)]
#[test]
#[ignore]
fn part_1_test_1() {
    assert_eq!(solve("turn on 0,0 through 999,999").0.unwrap(), 1_000_000);
}

#[test]
#[ignore]
fn part_1_test_2() {
    assert_eq!(solve("toggle 0,0 through 999,0").0.unwrap(), 1_000);
}

#[test]
#[ignore]
fn part_1_test_3() {
    assert_eq!(solve("turn off 499,499 through 500,500").0.unwrap(), 0);
}

#[test]
#[ignore]
fn part_2_test_1() {
    assert_eq!(solve("turn on 0,0 through 0,0").1.unwrap(), 1);
}

#[test]
#[ignore]
fn part_2_test_2() {
    assert_eq!(solve("toggle 0,0 through 999,999").1.unwrap(), 2_000_000);
}
