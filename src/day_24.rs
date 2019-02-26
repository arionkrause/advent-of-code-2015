pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 3));
    println!("Part 2: {}.", part_2::solve(&input, 4));
    println!();
}

fn decode_input(input: &str) -> Vec<u8> {
    input.lines()
            .map(|line| line.parse::<u8>().unwrap())
            .collect()
}

fn get_minimum_quantum_entanglement(input: &str, amount_compartments: u8) -> u64 {
    let weights = decode_input(&input);
    let weight_per_compartment = weights.iter().map(|w| *w as u16).sum::<u16>() / amount_compartments as u16;

    for group_size in 1..weights.len() {
        match get_first_valid_combination(&weights, weight_per_compartment, group_size as u8, vec![]) {
            Some(combination) => return combination.iter().fold(1, |accumulator, weight| accumulator * *weight as u64),
            None => {}
        }
    }

    unreachable!()
}

fn get_first_valid_combination(weights: &[u8], weight_per_compartment: u16, group_size: u8, weights_already_used: Vec<u8>) -> Option<Vec<u8>> {
    if (weights_already_used.len() as u8) < group_size {
        for index in 0..weights.len() {
            let mut weights_already_used_clone = weights_already_used.clone();
            weights_already_used_clone.push(weights[index]);
            let total_used_weight = weights_already_used_clone.iter().map(|w| *w as u16).sum::<u16>();

            if total_used_weight == weight_per_compartment {
                return Some(weights_already_used_clone);
            } else if total_used_weight < weight_per_compartment {
                if let Some(combination) = get_first_valid_combination(&weights[index + 1..], weight_per_compartment, group_size, weights_already_used_clone) {
                    return Some(combination);
                }
            }
        }
    }

    None
}

mod part_1 {
    use crate::day_24::get_minimum_quantum_entanglement;

    pub fn solve(input: &str, amount_compartments: u8) -> u64 {
        get_minimum_quantum_entanglement(&input, amount_compartments)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "1
2
3
4
5
7
8
9
10
11";

        assert_eq!(solve(&input, 3), 99);
    }
}

mod part_2 {
    use crate::day_24::get_minimum_quantum_entanglement;

    pub fn solve(input: &str, amount_compartments: u8) -> u64 {
        get_minimum_quantum_entanglement(&input, amount_compartments)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "1
2
3
4
5
7
8
9
10
11";

        assert_eq!(solve(&input, 4), 44);
    }
}
