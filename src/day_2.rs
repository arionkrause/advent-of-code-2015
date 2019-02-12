struct PresentBox {
    length: u32,
    width: u32,
    height: u32,
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<PresentBox> {
    input.lines().map(|line| {
        let dimensions: Vec<u32> = line.split('x').map(|c| c.parse().unwrap()).collect();

        PresentBox {
            length: dimensions[0],
            width: dimensions[1],
            height: dimensions[2],
        }
    }).collect()
}

mod part_1 {
    use crate::day_2::decode_input;

    pub fn solve(input: &str) -> u32 {
        let boxes = decode_input(&input);

        boxes.iter().map(|b| {
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
        }).sum()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("2x3x4"), 58);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("1x1x10"), 43);
    }
}

mod part_2 {
    use crate::day_2::decode_input;

    pub fn solve(input: &str) -> u32 {
        let boxes = decode_input(&input);

        boxes.iter().map(|b| {
            let mut length_needed = b.length * b.width * b.height;

            if b.length >= b.width && b.length >= b.height {
                length_needed += 2 * b.width + 2 * b.height;
            } else if b.width >= b.length && b.width >= b.height {
                length_needed += 2 * b.length + 2 * b.height;
            } else {
                length_needed += 2 * b.length + 2 * b.width;
            }

            length_needed
        }).sum()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("2x3x4"), 34);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("1x1x10"), 14);
    }
}
