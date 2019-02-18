pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 150));
    println!();
}

mod part_1 {
    pub fn solve(input: &str, amount_eggnog: u16) -> u16 {
        let mut containers = decode_input(&input);
        containers.sort();
        containers.reverse();
        get_valid_combinations_amount(&containers, amount_eggnog as i16)
    }

    fn get_valid_combinations_amount(containers: &[u8], amount_eggnog: i16) -> u16 {
        let mut amount_valid_combinations = 0;

        for i in 0..containers.len() {
            if containers[i] as i16 == amount_eggnog {
                amount_valid_combinations += 1;
            } else if (containers[i] as i16) < amount_eggnog {
                amount_valid_combinations += get_valid_combinations_amount(&containers[i + 1..], amount_eggnog - containers[i] as i16);
            }
        }

        amount_valid_combinations
    }

    fn decode_input(input: &str) -> Vec<u8> {
        let mut containers = Vec::new();

        for line in input.lines() {
            containers.push(line.parse::<u8>().unwrap());
        }

        containers
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "20
15
10
5
5";

        assert_eq!(solve(&input, 25), 4);
    }
}
