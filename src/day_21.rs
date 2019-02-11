pub fn solve(input: &str) -> (Option<i32>, Option<i32>) {
    print!("Day {}", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    let mut part_1 = None;
    let mut part_2 = None;

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
fn part_1_test_1() {}

#[test]
#[ignore]
fn part_2_test_1() {}
