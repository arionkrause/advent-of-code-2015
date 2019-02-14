pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn look_and_say(input: &str, iterations: usize) -> usize {
    let mut sequence = input.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();

    for _ in 0..iterations {
        let mut new_sequence = Vec::with_capacity(sequence.len());
        let mut count = 0;
        let mut index = 0;

        while index < sequence.len() {
            let digit = sequence[index];
            count += 1;

            while index + 1 < sequence.len()
                    && sequence[index + 1] == digit {
                count += 1;
                index += 1;
            }

            new_sequence.push(count);
            new_sequence.push(digit);
            count = 0;
            index += 1;
        }

        sequence = new_sequence;
    }

    sequence.len()
}

mod part_1 {
    use crate::day_10::look_and_say;

    pub fn solve(input: &str) -> usize {
        look_and_say(&input, 40)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(look_and_say("1", 1), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(look_and_say("11", 1), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(look_and_say("21", 1), 4);
    }

    #[test]
    fn test_4() {
        assert_eq!(look_and_say("1211", 1), 6);
    }

    #[test]
    fn test_5() {
        assert_eq!(look_and_say("111221", 1), 6);
    }
}

mod part_2 {
    use crate::day_10::look_and_say;

    pub fn solve(input: &str) -> usize {
        look_and_say(&input, 50)
    }
}
