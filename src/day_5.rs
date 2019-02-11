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
    let vowels = String::from("aeiou");

    input.lines().filter(|line| {
        if line.contains("ab")
                || line.contains("cd")
                || line.contains("pq")
                || line.contains("xy") {
            return false;
        }

        let mut has_repeated_character = false;
        let mut last_character = None;

        for character in line.chars() {
            if last_character.is_none() {
                last_character = Some(character);
                continue;
            }

            if character == last_character.unwrap() {
                has_repeated_character = true;
                break;
            }

            last_character = Some(character);
        }

        if !has_repeated_character {
            return false;
        }

        if line.chars().filter(|c| vowels.contains(*c)).count() < 3 {
            return false;
        }

        true
    }).count() as i32
}

fn part_2(input: &str) -> i32 {
    input.lines().filter(|line| {
        let mut characters_pairs_indices = HashMap::new();

        for (index, character) in line.chars().enumerate().take(line.chars().count() - 1) {
            let mut character_pair = character.to_string();
             character_pair.push(line.chars().skip(index + 1).take(1).next().unwrap());

            characters_pairs_indices.entry(character_pair)
                    .and_modify(|indices: &mut Vec<usize>| indices.push(index))
                    .or_insert(vec![index]);
        }

        if !characters_pairs_indices.iter().any(|(_, repeated_indices)| {
            if repeated_indices.len() < 2 {
                return false;
            }

            for (i, repeated_index) in repeated_indices.iter().enumerate().take(repeated_indices.len() - 1) {
                for (another_i, another_repeated_index) in repeated_indices.iter().enumerate() {
                    if i == another_i {
                        continue;
                    }

                    if repeated_indices[another_i] - repeated_index > 1 {
                        return true;
                    }
                }
            }

            return false;
        }) {
            return false;
        }

        if !line.chars().enumerate().take(line.chars().count() - 2).any(|(index, character)| {
            character == line.chars().skip(index + 2).take(1).next().unwrap()
        }) {
            return false;
        }

        true
    }).count() as i32
}

#[cfg(test)]
#[test]
#[ignore]
fn part_1_test_1() {
    assert_eq!(solve("ugknbfddgicrmopn").0.unwrap(), 1);
}

#[test]
#[ignore]
fn part_1_test_2() {
    assert_eq!(solve("aaa").0.unwrap(), 1);
}

#[test]
#[ignore]
fn part_1_test_3() {
    assert_eq!(solve("jchzalrnumimnmhp").0.unwrap(), 0);
}

#[test]
#[ignore]
fn part_1_test_4() {
    assert_eq!(solve("haegwjzuvuyypxyu").0.unwrap(), 0);
}

#[test]
#[ignore]
fn part_1_test_5() {
    assert_eq!(solve("dvszwmarrgswjxmb").0.unwrap(), 0);
}

#[test]
#[ignore]
fn part_2_test_1() {
    assert_eq!(solve("xyxy").1.unwrap(), 1);
}

#[test]
#[ignore]
fn part_2_test_2() {
    assert_eq!(solve("aabcdefgaa").1.unwrap(), 0);
}

#[test]
#[ignore]
fn part_2_test_3() {
    assert_eq!(solve("aaa").1.unwrap(), 0);
}

#[test]
#[ignore]
fn part_2_test_4() {
    assert_eq!(solve("xyx").1.unwrap(), 0);
}

#[test]
#[ignore]
fn part_2_test_5() {
    assert_eq!(solve("abcdefeghi").1.unwrap(), 0);
}

#[test]
#[ignore]
fn part_2_test_6() {
    assert_eq!(solve("aaa").1.unwrap(), 0);
}

#[test]
#[ignore]
fn part_2_test_7() {
    assert_eq!(solve("qjhvhtzxzqqjkmpb").1.unwrap(), 1);
}

#[test]
#[ignore]
fn part_2_test_8() {
    assert_eq!(solve("xxyxx").1.unwrap(), 1);
}

#[test]
#[ignore]
fn part_2_test_9() {
    assert_eq!(solve("uurcxstgmygtbstg").1.unwrap(), 0);
}

#[test]
#[ignore]
fn part_2_test_10() {
    assert_eq!(solve("ieodomkazucvgmuy").1.unwrap(), 0);
}

#[test]
#[ignore]
fn part_2_test_11() {
    assert_eq!(solve("rxexcbwhiywwwwnu").1.unwrap(), 1);
}
