use std::collections::HashMap;

struct Sue {
    id: u16,
    compounds: HashMap<Compound, u8>,
}

#[derive(Eq, Hash, PartialEq)]
enum Compound {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<Sue> {
    let re = regex::Regex::new(r"^Sue (?P<id>\d+): (?P<compound_1>\w+): (?P<compound_1_amount>\d+), (?P<compound_2>\w+): (?P<compound_2_amount>\d+), (?P<compound_3>\w+): (?P<compound_3_amount>\d+)$").unwrap();
    let mut sues = Vec::new();

    for line in input.lines() {
        let captures = re.captures(&line).unwrap();
        let id = captures.name("id").unwrap().as_str().parse::<u16>().unwrap();
        let compound_1_in_words = captures.name("compound_1").unwrap().as_str();
        let compound_2_in_words = captures.name("compound_2").unwrap().as_str();
        let compound_3_in_words = captures.name("compound_3").unwrap().as_str();
        let compound_1_amount = captures.name("compound_1_amount").unwrap().as_str().parse::<u8>().unwrap();
        let compound_2_amount = captures.name("compound_2_amount").unwrap().as_str().parse::<u8>().unwrap();
        let compound_3_amount = captures.name("compound_3_amount").unwrap().as_str().parse::<u8>().unwrap();
        let mut compounds = HashMap::new();
        compounds.insert(decode_compound(compound_1_in_words), compound_1_amount);
        compounds.insert(decode_compound(compound_2_in_words), compound_2_amount);
        compounds.insert(decode_compound(compound_3_in_words), compound_3_amount);

        sues.push(Sue {
            id,
            compounds,
        });
    }

    sues
}

fn decode_compound(compound_in_word: &str) -> Compound {
    match compound_in_word {
        "children" => Compound::Children,
        "cats" => Compound::Cats,
        "samoyeds" => Compound::Samoyeds,
        "pomeranians" => Compound::Pomeranians,
        "akitas" => Compound::Akitas,
        "vizslas" => Compound::Vizslas,
        "goldfish" => Compound::Goldfish,
        "trees" => Compound::Trees,
        "cars" => Compound::Cars,
        "perfumes" => Compound::Perfumes,
        _ => panic!(),
    }
}

fn get_mfcsam_output() -> HashMap<Compound, u8> {
    let mut mfcsam_output = HashMap::new();
    mfcsam_output.insert(Compound::Children, 3);
    mfcsam_output.insert(Compound::Cats, 7);
    mfcsam_output.insert(Compound::Samoyeds, 2);
    mfcsam_output.insert(Compound::Pomeranians, 3);
    mfcsam_output.insert(Compound::Akitas, 0);
    mfcsam_output.insert(Compound::Vizslas, 0);
    mfcsam_output.insert(Compound::Goldfish, 5);
    mfcsam_output.insert(Compound::Trees, 3);
    mfcsam_output.insert(Compound::Cars, 2);
    mfcsam_output.insert(Compound::Perfumes, 1);
    mfcsam_output
}

mod part_1 {
    use crate::day_16::decode_input;
    use crate::day_16::get_mfcsam_output;

    pub fn solve(input: &str) -> u16 {
        let sues = decode_input(&input);
        let mfcsam_output = get_mfcsam_output();

        sues.iter().find(|sue| {
            !sue.compounds.iter().any(|(compound, amount)| amount != mfcsam_output.get(&compound).unwrap())
        }).unwrap().id
    }
}

mod part_2 {
    use crate::day_16::decode_input;
    use crate::day_16::get_mfcsam_output;
    use crate::day_16::Compound;

    pub fn solve(input: &str) -> u16 {
        let sues = decode_input(&input);
        let mfcsam_output = get_mfcsam_output();

        sues.iter().find(|sue| {
            !sue.compounds.iter().any(|(compound, amount)| {
                match compound {
                    Compound::Cats | Compound::Trees => amount <= mfcsam_output.get(&compound).unwrap(),
                    Compound::Pomeranians | Compound::Goldfish => amount >= mfcsam_output.get(&compound).unwrap(),
                    _ => amount != mfcsam_output.get(&compound).unwrap(),
                }
            })
        }).unwrap().id
    }
}
