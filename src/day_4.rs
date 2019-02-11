use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn solve(input: &str) -> (Option<i32>, Option<i32>) {
    print!("Day {}", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    let mut part_1 = None;
    let mut part_2 = None;
    let mut md5_hasher = Md5::new();
    let mut number = 1;
    let expected_starting_part_1 = "0".repeat(5);
    let expected_starting_part_2 = "0".repeat(6);

    loop {
        let mut attempt = format!("{}{}", input, number);
        md5_hasher.input_str(&attempt);
        let digest = md5_hasher.result_str();

        if part_1.is_none() && digest.starts_with(&expected_starting_part_1) {
            part_1 = Some(number);
        }

        if digest.starts_with(&expected_starting_part_2) {
            part_2 = Some(number);
            break;
        }

        md5_hasher.reset();
        number += 1;
    }

    if part_1.is_some() {
        print!(" - Part 1: {}.", part_1.unwrap());
    }

    if part_2.is_some() {
        println!(" Part 2: {}.", part_2.unwrap());
    }

    (part_1, part_2)
}

#[cfg(test)]
#[test]
#[ignore]
fn part_1_test_1() {
    assert_eq!(solve("abcdef").0.unwrap(), 609043);
}

#[test]
#[ignore]
fn part_1_test_2() {
    assert_eq!(solve("pqrstuv").0.unwrap(), 1048970);
}

#[test]
#[ignore]
fn part_2_test_1() {}
