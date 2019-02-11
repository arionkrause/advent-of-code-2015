use std::collections::HashMap;

pub fn solve(input: &str) -> (Option<i32>, Option<i32>) {
    print!("Day {}", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    let mut part_1 = Some(part_1(&input));
    let mut part_2 = Some(part_2(&input));

    if part_1.is_some() {
        print!(" - Part 1: {}.", part_1.unwrap());
    }

    if part_2.is_some() {
        println!(" Part 2: {}.", part_2.unwrap());
    }

    (part_1, part_2)
}

fn part_1(input: &str) -> i32 {
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

    visited_positions.iter().len() as i32
}

fn part_2(input: &str) -> i32 {
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

    visited_positions.iter().len() as i32
}

#[cfg(test)]
#[test]
#[ignore]
fn part_1_test_1() {
    assert_eq!(solve(">").0.unwrap(), 2);
}

#[test]
#[ignore]
fn part_1_test_2() {
    assert_eq!(solve("^>v<").0.unwrap(), 4);
}

#[test]
#[ignore]
fn part_1_test_3() {
    assert_eq!(solve("^v^v^v^v^v").0.unwrap(), 2);
}

#[test]
#[ignore]
fn part_2_test_1() {
    assert_eq!(solve("^v").1.unwrap(), 3);
}

#[test]
#[ignore]
fn part_2_test_2() {
    assert_eq!(solve("^>v<").1.unwrap(), 3);
}

#[test]
#[ignore]
fn part_2_test_3() {
    assert_eq!(solve("^v^v^v^v^v").1.unwrap(), 11);
}
