pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> isize {
        (input.chars().filter(|&c| c == '(').count() as isize
                - input.chars().filter(|&c| c == ')').count() as isize)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("(())"), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("()()"), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("((("), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("(()(()("), 3);
    }

    #[test]
    fn test_5() {
        assert_eq!(solve("))((((("), 3);
    }

    #[test]
    fn test_6() {
        assert_eq!(solve("())"), -1);
    }

    #[test]
    fn test_7() {
        assert_eq!(solve("))("), -1);
    }

    #[test]
    fn test_8() {
        assert_eq!(solve(")))"), -3);
    }

    #[test]
    fn test_9() {
        assert_eq!(solve(")())())"), -3);
    }
}

mod part_2 {
    pub fn solve(input: &str) -> usize {
        let mut floor = 0;

        for (index, direction) in input.chars().enumerate() {
            match direction {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!(),
            };

            if floor < 0 {
                return index + 1;
            }
        }

        panic!()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve(")"), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("()())"), 5);
    }
}
