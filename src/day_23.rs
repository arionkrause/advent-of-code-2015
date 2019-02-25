use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Hlf { register: char },
    Inc { register: char },
    Jie { register: char, offset: u8 },
    Jio { register: char, offset: u8 },
    Jmp { offset: i8 },
    Tpl { register: char },
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<Instruction> {
    let re_hlf_inc_tpl = Regex::new(r"^(?P<instruction>hlf|inc|tpl) (?P<register>[ab])$").unwrap();
    let re_jie_jio = Regex::new(r"^(?P<instruction>ji[eo]) (?P<register>[ab]), \+(?P<offset>\d+)$").unwrap();
    let re_jmp = Regex::new(r"^jmp (?P<offset>[-+]\d+)$").unwrap();
    let mut instructions = Vec::new();

    for line in input.lines() {
        if let Some(captures) = re_hlf_inc_tpl.captures(&line) {
            let instruction_name = captures.name("instruction").unwrap().as_str();
            let register = captures.name("register").unwrap().as_str().chars().next().unwrap();

            instructions.push(match instruction_name {
                "hlf" => Instruction::Hlf { register },
                "inc" => Instruction::Inc { register },
                "tpl" => Instruction::Tpl { register },
                _ => panic!(),
            });

            continue;
        }

        if let Some(captures) = re_jie_jio.captures(&line) {
            let instruction_name = captures.name("instruction").unwrap().as_str();
            let register = captures.name("register").unwrap().as_str().chars().next().unwrap();
            let offset = captures.name("offset").unwrap().as_str().parse().unwrap();

            instructions.push(match instruction_name {
                "jie" => Instruction::Jie { register, offset },
                "jio" => Instruction::Jio { register, offset },
                _ => panic!(),
            });

            continue;
        }

        if let Some(captures) = re_jmp.captures(&line) {
            let offset = captures.name("offset").unwrap().as_str().parse().unwrap();
            instructions.push(Instruction::Jmp { offset });
            continue;
        }
    }

    instructions
}

mod part_1 {
    use crate::day_23::decode_input;
    use crate::day_23::Instruction;

    pub fn solve(input: &str) -> u32 {
        let instructions = decode_input(&input);
        let mut register_a = 0u32;
        let mut register_b = 0u32;
        let mut program_counter = 0;

        while program_counter < instructions.len() {
            match instructions[program_counter] {
                Instruction::Hlf { register } => {
                    if register == 'a' {
                        register_a /= 2;
                    } else {
                        register_b /= 2;
                    }

                    program_counter += 1;
                }
                Instruction::Inc { register } => {
                    if register == 'a' {
                        register_a += 1;
                    } else {
                        register_b += 1;
                    }

                    program_counter += 1;
                }
                Instruction::Jie { register, offset } => {
                    if (register == 'a' && register_a % 2 == 0)
                            || register == 'b' && register_b % 2 == 0 {
                        program_counter += offset as usize;
                    } else {
                        program_counter += 1;
                    }
                }
                Instruction::Jio { register, offset } => {
                    if (register == 'a' && register_a == 1)
                            || register == 'b' && register_b == 1 {
                        program_counter += offset as usize;
                    } else {
                        program_counter += 1;
                    }
                }
                Instruction::Jmp { offset } => {
                    if (program_counter as isize + offset as isize) < 0 {
                        break;
                    }

                    program_counter = (program_counter as isize + offset as isize) as usize;
                }
                Instruction::Tpl { register } => {
                    if register == 'a' {
                        register_a *= 3;
                    } else {
                        register_b *= 3;
                    }

                    program_counter += 1;
                }
            }
        }

        register_b
    }
}
