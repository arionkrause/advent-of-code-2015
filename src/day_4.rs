pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    use crypto::digest::Digest;
    use crypto::md5::Md5;

    pub fn solve(input: &str) -> usize {
        let mut md5_hasher = Md5::new();
        let mut number = 1;
        let expected_starting = "0".repeat(5);

        loop {
            let attempt = format!("{}{}", input, number);
            md5_hasher.input_str(&attempt);
            let digest = md5_hasher.result_str();

            if digest.starts_with(&expected_starting) {
                return number;
            }

            md5_hasher.reset();
            number += 1;
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("abcdef"), 609043);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("pqrstuv"), 1048970);
    }
}

mod part_2 {
    use crypto::digest::Digest;
    use crypto::md5::Md5;

    pub fn solve(input: &str) -> usize {
        let mut md5_hasher = Md5::new();
        let mut number = 1;
        let expected_starting = "0".repeat(6);

        loop {
            let attempt = format!("{}{}", input, number);
            md5_hasher.input_str(&attempt);
            let digest = md5_hasher.result_str();

            if digest.starts_with(&expected_starting) {
                return number;
            }

            md5_hasher.reset();
            number += 1;
        }
    }
}
