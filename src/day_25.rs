pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!();
}

mod part_1 {
    use regex::Regex;

    pub fn solve(input: &str) -> u64 {
        let (row, column) = decode_input(&input);
        let mut number = 20151125;
        let diagonal_size = row as u64 + column as u64 - 1; // Row and column intersect on one number, thus the "- 1" to deduplicate
        let amount_indexes_up_to_diagonal = (diagonal_size.pow(2) + diagonal_size) / 2; // https://en.wikipedia.org/wiki/Triangular_number#Formula
        let iterations = amount_indexes_up_to_diagonal - row as u64; // Each row intersects with one number on the diagonal, so it acts as an offset

        for _ in 0..iterations {
            number *= 252533;
            number %= 33554393;
        }

        number
    }

    fn decode_input(input: &str) -> (u16, u16) {
        let re = Regex::new(r"^To continue, please consult the code grid in the manual\. {2}Enter the code at row (?P<row>\d+), column (?P<column>\d+)\.$").unwrap();
        let captures = re.captures(&input).unwrap();
        (captures.name("row").unwrap().as_str().parse().unwrap(), captures.name("column").unwrap().as_str().parse().unwrap())
    }
}
