pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    use std::collections::HashMap;
    use regex::Regex;
    use std::collections::HashSet;

    pub fn solve(input: &str) -> usize {
        let (replacements, starting_molecule) = decode_input(&input);
        let mut distinct_molecules = HashSet::new();

        for (from, tos) in replacements {
            for (index, _) in starting_molecule.match_indices(&from).collect::<Vec<_>>() {
                for to in tos.iter() {
                    let mut new_molecule = starting_molecule.chars().take(index).collect::<String>();
                    new_molecule.push_str(&starting_molecule.chars().skip(index).collect::<String>().replacen(&from, to, 1));
                    distinct_molecules.insert(new_molecule);
                }
            }
        }

        distinct_molecules.len()
    }

    fn decode_input(input: &str) -> (HashMap<String, Vec<String>>, String) {
        let re_replacements = Regex::new(r"^(?P<from>\w+) => (?P<to>\w+)$").unwrap();
        let re_starting_molecule = Regex::new(r"^(?P<starting_molecule>\w+)$").unwrap();
        let mut replacements = HashMap::new();
        let mut starting_molecule = None;

        for line in input.lines() {
            if let Some(captures) = re_replacements.captures(&line) {
                let from = captures.name("from").unwrap().as_str().to_string();
                let to = captures.name("to").unwrap().as_str().to_string();
                replacements.entry(from).and_modify(|tos: &mut Vec<String>| tos.push(to.clone())).or_insert(vec![to]);
            }

            if let Some(captures) = re_starting_molecule.captures(&line) {
                starting_molecule = Some(captures.name("starting_molecule").unwrap().as_str().to_string());
            }
        }

        (replacements, starting_molecule.unwrap())
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

//    #[test]
//    fn test_2() {
//        let input = "H => HO
//H => OH
//O => HH
//
//HOHOHO";
//
//        assert_eq!(solve(&input), 7);
//    }
}
