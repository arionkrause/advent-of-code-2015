pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn next_password(input: &str) -> String {
    let mut ascii_encoded_password = input.chars().map(|c| c as u8).collect::<Vec<u8>>();

    loop {
        ascii_encoded_password[7] += 1;

        for i in (1..8).rev() {
            if ascii_encoded_password[i] == 123 {
                ascii_encoded_password[i] = 97;
                ascii_encoded_password[i - 1] += 1;
            }
        }

        if ascii_encoded_password.iter().any(|&l| l == 105 || l == 108 || l == 111) {
            continue;
        }

        let mut has_three_letters_increasing = false;

        for i in 0..6 {
            let i_2 = i + 1;
            let i_3 = i + 2;

            if ascii_encoded_password[i_3] as isize - ascii_encoded_password[i_2] as isize == 1
                    && ascii_encoded_password[i_2] as isize - ascii_encoded_password[i] as isize == 1 {
                has_three_letters_increasing = true;
                break;
            }
        }

        if !has_three_letters_increasing {
            continue;
        }

        let mut index_first_pair_of_letters = None;

        for i in 0..5 {
            if ascii_encoded_password[i] == ascii_encoded_password[i + 1] {
                index_first_pair_of_letters = Some(i);
                break;
            }
        }

        if index_first_pair_of_letters.is_none() {
            continue;
        }

        let mut second_pair_of_letters = None;

        for i in index_first_pair_of_letters.unwrap() + 2..7 {
            if ascii_encoded_password[i] == ascii_encoded_password[i + 1]
                    && ascii_encoded_password[i] != ascii_encoded_password[index_first_pair_of_letters.unwrap()] {
                second_pair_of_letters = Some(ascii_encoded_password[i]);
                break;
            }
        }

        if second_pair_of_letters.is_none() {
            continue;
        }

        break;
    }

    ascii_encoded_password.iter().map(|c| *c as char).collect()
}

mod part_1 {
    use crate::day_11::next_password;

    pub fn solve(input: &str) -> String {
        next_password(&input)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("abcdefgh"), "abcdffaa");
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("ghijklmn"), "ghjaabcc");
    }
}

mod part_2 {
    use crate::day_11::next_password;

    pub fn solve(input: &str) -> String {
        next_password(&next_password(&input))
    }
}
