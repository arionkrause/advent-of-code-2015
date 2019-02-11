#[derive(Debug)]
struct Box {
    length: u32,
    width: u32,
    height: u32,
}

pub fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    print!("Day {}", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    let mut part_1 = None;
    let mut part_2 = None;

    let boxes = decode_input(&input);

    part_1 = Some(boxes.iter().map(|b| {
        let mut area_needed = 2 * b.length * b.width
                + 2 * b.width * b.height
                + 2 * b.height * b.length;

        if b.length >= b.width && b.length >= b.height {
            area_needed += b.width * b.height;
        } else if b.width >= b.length && b.width >= b.height {
            area_needed += b.length * b.height;
        } else {
            area_needed += b.length * b.width;
        }

        area_needed
    }).sum());

    part_2 = Some(boxes.iter().map(|b| {
        let mut length_needed = b.length * b.width * b.height;

        if b.length >= b.width && b.length >= b.height {
            length_needed += 2 * b.width + 2 * b.height;
        } else if b.width >= b.length && b.width >= b.height {
            length_needed += 2 * b.length + 2 * b.height;
        } else {
            length_needed += 2 * b.length + 2 * b.width;
        }

        length_needed
    }).sum());

    if part_1.is_some() {
        print!(" - Part 1: {}.", part_1.unwrap());
    }

    if part_2.is_some() {
        println!(" Part 2: {}.", part_2.unwrap());
    }

    (part_1, part_2)
}

fn decode_input(input: &str) -> Vec<Box> {
    input.lines().map(|line| {
        let dimensions: Vec<u32> = line.split('x').map(|c| c.parse().unwrap()).collect();

        Box {
            length: dimensions[0],
            width: dimensions[1],
            height: dimensions[2],
        }
    }).collect()
}

#[cfg(test)]
#[test]
#[ignore]
fn part_1_test_1() {
    assert_eq!(solve("2x3x4").0.unwrap(), 58);
}

#[test]
#[ignore]
fn part_1_test_2() {
    assert_eq!(solve("1x1x10").0.unwrap(), 43);
}

#[test]
#[ignore]
fn part_2_test_1() {
    assert_eq!(solve("2x3x4").1.unwrap(), 34);
}

#[test]
#[ignore]
fn part_2_test_2() {
    assert_eq!(solve("1x1x10").1.unwrap(), 14);
}
