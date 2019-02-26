pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    use regex::Regex;

    pub fn solve(input: &str) -> u64 {
        let (target_row, target_column) = decode_input(&input);
        let mut row = 2;
        let mut previous_number = 20151125;

        loop {
            for column in 1..=row {
                let result = previous_number * 252533 % 33554393;

                if row - column + 1 == target_row
                        && column == target_column {
                    return result;
                }

                previous_number = result;
            }

            row += 1;
        }
    }

    fn decode_input(input: &str) -> (u16, u16) {
        let re = Regex::new(r"^To continue, please consult the code grid in the manual\. {2}Enter the code at row (?P<row>\d+), column (?P<column>\d+)\.$").unwrap();
        let captures = re.captures(&input).unwrap();
        (captures.name("row").unwrap().as_str().parse().unwrap(), captures.name("column").unwrap().as_str().parse().unwrap())
    }
}
