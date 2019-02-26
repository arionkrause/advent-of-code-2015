pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<u8> {
    input.lines()
            .map(|line| line.parse::<u8>().unwrap())
            .collect()
}

mod part_1 {
    use crate::day_24::decode_input;

    pub fn solve(input: &str) -> u64 {
        let mut weights = decode_input(&input);
        weights.sort_by(|a, b| b.cmp(&a));
        let weight_per_compartment = weights.iter().map(|w| *w as u16).sum::<u16>() / 3;
        let mut valid_combinations = Vec::new();
        let mut minimum_amount_weights_compartment_1 = None;
        let valid_compartment_1_combinations = get_valid_combinations(&weights, weight_per_compartment, 0, vec![]);

        for valid_compartment_1_combination in valid_compartment_1_combinations {
            let mut remaining_weights_after_compartment_1 = weights.clone();
            remaining_weights_after_compartment_1.retain(|w| !valid_compartment_1_combination.contains(&w));

            if exist_valid_combinations(&remaining_weights_after_compartment_1, weight_per_compartment, 0, vec![]) {
                if minimum_amount_weights_compartment_1.is_none() || valid_compartment_1_combination.len() < minimum_amount_weights_compartment_1.unwrap() {
                    minimum_amount_weights_compartment_1 = Some(valid_compartment_1_combination.len());
                    valid_combinations.clear();
                }

                valid_combinations.push(valid_compartment_1_combination.clone());
                continue;
            }
        }

        let minimum_amount_weights_compartment_1 = valid_combinations.iter()
                .map(|compartment_1| compartment_1.len())
                .min()
                .unwrap();

        valid_combinations.iter()
                .filter(|compartment_1| compartment_1.len() == minimum_amount_weights_compartment_1)
                .map(|compartment_1| compartment_1.iter().fold(1, |accumulator, weight| accumulator * *weight as u64))
                .min()
                .unwrap()
    }

    fn get_valid_combinations(weights: &[u8], weight_per_compartment: u16, total_weight_used: u16, weights_already_used: Vec<u8>) -> Vec<Vec<u8>> {
        let mut valid_combinations = Vec::new();

        for index in 0..weights.len() {
            let new_used_weight = total_weight_used + weights[index] as u16;

            if new_used_weight == weight_per_compartment {
                let mut weights_already_used_clone = weights_already_used.clone();
                weights_already_used_clone.push(weights[index]);
                valid_combinations.push(weights_already_used_clone);
                continue;
            } else if new_used_weight < weight_per_compartment {
                let mut weights_already_used_clone = weights_already_used.clone();
                weights_already_used_clone.push(weights[index]);
                valid_combinations.extend(get_valid_combinations(&weights[index + 1..], weight_per_compartment, new_used_weight, weights_already_used_clone))
            }
        }

        valid_combinations
    }

    fn exist_valid_combinations(weights: &[u8], weight_per_compartment: u16, total_weight_used: u16, weights_already_used: Vec<u8>) -> bool {
        for index in 0..weights.len() {
            let new_used_weight = total_weight_used + weights[index] as u16;

            if new_used_weight == weight_per_compartment {
                return true;
            } else if new_used_weight < weight_per_compartment {
                let mut weights_already_used_clone = weights_already_used.clone();
                weights_already_used_clone.push(weights[index]);

                if exist_valid_combinations(&weights[index + 1..], weight_per_compartment, new_used_weight, weights_already_used_clone) {
                    return true;
                }
            }
        }

        false
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

        assert_eq!(solve(&input), 99);
    }
}
