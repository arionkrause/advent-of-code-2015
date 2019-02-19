use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn decode_input(input: &str) -> (HashMap<String, Vec<String>>, String) {
    (decode_replacements(&input), decode_starting_molecule(&input))
}

fn decode_replacements(input: &str) -> HashMap<String, Vec<String>> {
    let re_replacements = Regex::new(r"^(?P<from>\w+) => (?P<to>\w+)$").unwrap();
    let mut replacements = HashMap::new();

    for line in input.lines() {
        if let Some(captures) = re_replacements.captures(&line) {
            let from = captures.name("from").unwrap().as_str().to_string();
            let to = captures.name("to").unwrap().as_str().to_string();
            replacements.entry(from).and_modify(|tos: &mut Vec<String>| tos.push(to.clone())).or_insert(vec![to]);
        }
    }

    replacements
}

fn decode_starting_molecule(input: &str) -> String {
    let re_starting_molecule = Regex::new(r"^(?P<starting_molecule>\w+)$").unwrap();
    let mut starting_molecule = None;

    for line in input.lines() {
        if let Some(captures) = re_starting_molecule.captures(&line) {
            starting_molecule = Some(captures.name("starting_molecule").unwrap().as_str().to_string());
        }
    }

    starting_molecule.unwrap()
}

mod part_1 {
    use std::collections::HashSet;
    use crate::day_19::decode_input;

    pub fn solve(input: &str) -> usize {
        let (replacements, starting_molecule) = decode_input(&input);
        let mut distinct_molecules = HashSet::new();

        for (from, tos) in replacements.iter() {
            for (index, _) in starting_molecule.match_indices(from).collect::<Vec<_>>() {
                for to in tos.iter() {
                    let mut new_molecule = starting_molecule.chars().take(index).collect::<String>();
                    new_molecule.push_str(&starting_molecule.chars().skip(index).collect::<String>().replacen(from, to, 1));
                    distinct_molecules.insert(new_molecule);
                }
            }
        }

        distinct_molecules.len()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "H => HO
H => OH
O => HH

HOH";

        assert_eq!(solve(&input), 4);
    }

    #[test]
    fn test_2() {
        let input = "H => HO
H => OH
O => HH

HOHOHO";

        assert_eq!(solve(&input), 7);
    }
}

mod part_2 {
    use crate::day_19::decode_starting_molecule;

    pub fn solve(input: &str) -> usize {
        let molecule = decode_starting_molecule(&input);

        if molecule.match_indices("Rn").collect::<Vec<_>>().len() == 0 {
            return molecule.chars().filter(|c| c.is_uppercase()).count();
        }

        molecule.chars().filter(|c| c.is_uppercase()).count()
                - molecule.match_indices("Rn").collect::<Vec<_>>().len()
                - molecule.match_indices("Ar").collect::<Vec<_>>().len()
                - molecule.match_indices("Y").collect::<Vec<_>>().len() * 2
                - 1
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "e => H
e => O
H => HO
H => OH
O => HH

HOH";

        assert_eq!(solve(&input), 3);
    }

    #[test]
    fn test_2() {
        let input = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO";

        assert_eq!(solve(&input), 6);
    }
}
