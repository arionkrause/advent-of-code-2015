use regex::Regex;

struct Instruction {
    command: Command,
    from: Position,
    to: Position,
}

struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Clone, Eq, PartialEq)]
enum State {
    On,
    Off,
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
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

mod part_1 {
    use crate::day_6::Command;
    use crate::day_6::State;
    use crate::day_6::decode_input;

    pub fn solve(input: &str) -> usize {
        let mut grid = vec![vec![State::Off; 1000]; 1000];
        let instructions = decode_input(&input);

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

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("turn on 0,0 through 999,999"), 1_000_000);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("toggle 0,0 through 999,0"), 1_000);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("turn off 499,499 through 500,500"), 0);
    }
}

mod part_2 {
    use crate::day_6::Command;
    use crate::day_6::decode_input;

    pub fn solve(input: &str) -> usize {
        let instructions = decode_input(&input);
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

        grid.iter().map(|row| row.iter().sum::<usize>()).sum()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("turn on 0,0 through 0,0"), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("toggle 0,0 through 999,999"), 2_000_000);
    }
}
