use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<Vec<i8>> {
    let re = Regex::new(r"^(?P<person>\w+) would (?P<change>(gain|lose)) (?P<amount>\d+) happiness units by sitting next to (?P<other_person>\w+).$").unwrap();
    let persons_indices = get_persons_indices(&input);
    let mut happiness_changes = vec![vec![0i8; persons_indices.len()]; persons_indices.len()];

    for line in input.lines() {
        let captures = re.captures(&line).unwrap();
        let person_in_word = captures.name("person").unwrap().as_str().to_string();
        let person_index = *persons_indices.get(&person_in_word).unwrap();
        let mut amount: i8 = captures.name("amount").unwrap().as_str().parse().unwrap();

        if captures.name("change").unwrap().as_str() == "lose" {
            amount *= -1;
        }

        let other_person_in_word = captures.name("other_person").unwrap().as_str().to_string();
        let other_person_index = *persons_indices.get(&other_person_in_word).unwrap();
        happiness_changes[person_index][other_person_index] = amount;
    }

    happiness_changes
}

fn get_persons_indices(input: &str) -> HashMap<String, usize> {
    let re = Regex::new(r"^(?P<person>\w+)").unwrap();
    let mut persons_indices = HashMap::new();

    for line in input.lines() {
        let person_in_word = re.captures(&line).unwrap().name("person").unwrap().as_str().to_string();

        if persons_indices.get(&person_in_word).is_none() {
            persons_indices.insert(person_in_word, persons_indices.len());
        };
    }

    persons_indices
}

fn get_optimal_change_in_happiness(happiness_changes: &Vec<Vec<i8>>, seating: Vec<usize>) -> i16 {
    let mut most_happiness = None;

    if seating.len() == happiness_changes.len() {
        return get_total_change_in_happiness(&happiness_changes, &seating);
    }

    for (person_index, _) in happiness_changes.iter().enumerate().filter(|(person_index, _)| !seating.contains(person_index)) {
        let mut new_seating = seating.clone();
        new_seating.push(person_index);
        let happiness = get_optimal_change_in_happiness(&happiness_changes, new_seating);

        if most_happiness.is_none() || happiness > most_happiness.unwrap() {
            most_happiness = Some(happiness);
        }
    }

    most_happiness.unwrap()
}

fn get_total_change_in_happiness(hapinness_changes: &Vec<Vec<i8>>, seating: &Vec<usize>) -> i16 {
    let mut total_change_in_happiness = 0;

    for (seating_index, person_index) in seating.iter().enumerate() {
        let person_side_1_index = if seating_index == 0 {
            seating[seating.len() - 1]
        } else {
            seating[seating_index - 1]
        };

        let person_side_2_index = if seating_index == seating.len() - 1 {
            seating[0]
        } else {
            seating[seating_index + 1]
        };

        total_change_in_happiness += hapinness_changes[*person_index][person_side_1_index] as i16;
        total_change_in_happiness += hapinness_changes[*person_index][person_side_2_index] as i16;
    }

    total_change_in_happiness
}

mod part_1 {
    use crate::day_13::decode_input;
    use crate::day_13::get_optimal_change_in_happiness;

    pub fn solve(input: &str) -> i16 {
        let happiness_changes = decode_input(&input);
        get_optimal_change_in_happiness(&happiness_changes, vec![0])
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."), 330);
    }
}

mod part_2 {
    use crate::day_13::decode_input;
    use crate::day_13::get_optimal_change_in_happiness;

    pub fn solve(input: &str) -> i16 {
        let mut happiness_changes = decode_input(&input);

        for happiness_change in happiness_changes.iter_mut() {
            happiness_change.push(0);
        }

        happiness_changes.push(vec![0; happiness_changes[0].len()]);
        get_optimal_change_in_happiness(&happiness_changes, vec![0])
    }
}
