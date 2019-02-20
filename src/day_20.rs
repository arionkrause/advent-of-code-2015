pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    pub fn solve(input: &str) -> u32 {
        let input_as_number = input.parse::<u32>().unwrap();
        let mut houses_presents = vec![0; (input_as_number / 10) as usize + 1];

        for elf_number in 1..houses_presents.len() as usize {
            for house_number in (elf_number..houses_presents.len()).step_by(elf_number) {
                houses_presents[house_number] += elf_number as u32 * 10;
            }
        }

        for house_number in 0..houses_presents.len() {
            if houses_presents[house_number] >= input_as_number {
                return house_number as u32;
            }
        }

        unreachable!()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("10"), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("30"), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("40"), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve("70"), 4);
    }

    #[test]
    fn test_5() {
        assert_eq!(solve("60"), 4);
    }

    #[test]
    fn test_6() {
        assert_eq!(solve("120"), 6);
    }

    #[test]
    fn test_7() {
        assert_eq!(solve("80"), 6);
    }

    #[test]
    fn test_8() {
        assert_eq!(solve("150"), 8);
    }

    #[test]
    fn test_9() {
        assert_eq!(solve("130"), 8);
    }
}

mod part_2 {
    pub fn solve(input: &str) -> u32 {
        let input_as_number = input.parse::<u32>().unwrap();
        let mut houses_presents = vec![0; (input_as_number / 10) as usize + 1];

        for elf_number in 1..houses_presents.len() {
            let mut count = 0;

            for house_number in (elf_number..houses_presents.len()).step_by(elf_number) {
                houses_presents[house_number] += elf_number as u32 * 11;
                count += 1;

                if count == 50 {
                    break;
                }
            }
        }

        for house_number in 0..houses_presents.len() {
            if houses_presents[house_number] >= input_as_number {
                return house_number as u32;
            }
        }

        unreachable!()
    }
}
