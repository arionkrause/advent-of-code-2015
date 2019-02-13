pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> usize {
        let mut amount_characters_decoded = 0;

        for line in input.lines() {
            let mut index = 1;

            while index < line.chars().count() - 1 {
                if line.chars().skip(index).next().unwrap() == '\\' {
                    if line.chars().skip(index + 1).next().unwrap() == '\\'
                            || line.chars().skip(index + 1).next().unwrap() == '\"' {
                        index += 2;
                    } else if line.chars().skip(index + 1).next().unwrap() == 'x' {
                        index += 4;
                    }
                } else {
                    index += 1;
                }

                amount_characters_decoded += 1;
            }
        }

        (input.len() - input.lines().count() + 1) - amount_characters_decoded
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

        assert_eq!(solve(&input), 12);
    }
}

mod part_2 {
    pub fn solve(input: &str) -> usize {
        let mut amount_characters_encoded = 0;

        for line in input.lines() {
            amount_characters_encoded += 2;

            for character in line.chars() {
                if character == '\\' || character == '\"' {
                    amount_characters_encoded += 2;
                } else {
                    amount_characters_encoded += 1;
                }
            }
        }

        amount_characters_encoded - (input.len() - input.lines().count() + 1)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

        assert_eq!(solve(&input), 19);
    }
}
