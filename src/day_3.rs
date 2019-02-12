pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    use std::collections::HashMap;

    pub fn solve(input: &str) -> usize {
        let mut position = (0, 0);
        let mut visited_positions = HashMap::new();
        visited_positions.insert(position.clone(), 1);

        input.chars().for_each(|c| {
            match c {
                '^' => position.1 -= 1,
                '>' => position.0 += 1,
                'v' => position.1 += 1,
                '<' => position.0 -= 1,
                _ => panic!(),
            };

            visited_positions.entry(position.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
        });

        visited_positions.iter().len()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve(">"), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("^>v<"), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("^v^v^v^v^v"), 2);
    }
}

mod part_2 {
    use std::collections::HashMap;

    pub fn solve(input: &str) -> usize {
        let mut position_santa = (0, 0);
        let mut position_robo_santa = (0, 0);
        let mut visited_positions = HashMap::new();
        visited_positions.insert(position_santa.clone(), 2);

        input.chars().enumerate().for_each(|(index, c)| {
            let mut position = if index % 2 == 0 {
                &mut position_santa
            } else {
                &mut position_robo_santa
            };

            match c {
                '^' => position.1 -= 1,
                '>' => position.0 += 1,
                'v' => position.1 += 1,
                '<' => position.0 -= 1,
                _ => panic!(),
            };

            visited_positions.entry(position.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
        });

        visited_positions.iter().len()
    }

    #[cfg(test)]
    #[test]
    fn part_2_test_1() {
        assert_eq!(solve("^v"), 3);
    }

    #[test]
    fn part_2_test_2() {
        assert_eq!(solve("^>v<"), 3);
    }

    #[test]
    fn part_2_test_3() {
        assert_eq!(solve("^v^v^v^v^v"), 11);
    }
}
