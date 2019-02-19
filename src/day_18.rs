pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 100));
    println!();
}

mod part_1 {
    pub fn solve(input: &str, steps: u8) -> u16 {
        let mut grid = decode_input(&input);

        for _ in 0..steps {
            let mut new_grid = Vec::with_capacity(grid.len());

            for (y, row) in grid.iter().enumerate() {
                let mut new_row = Vec::with_capacity(row.len());

                for (x, tile) in row.iter().enumerate() {
                    let amount_neighbors_on = get_amount_neighbors_on(&grid, y, x);

                    if *tile == '#' && amount_neighbors_on != 2 && amount_neighbors_on != 3 {
                        new_row.push('.');
                    } else if *tile == '.' && amount_neighbors_on == 3 {
                        new_row.push('#');
                    } else {
                        new_row.push(*tile);
                    }
                }

                new_grid.push(new_row);
            }

            grid = new_grid;
        }

        grid.iter().map(|row| row.iter().filter(|&tile| *tile == '#').count() as u16).sum()
    }

    fn decode_input(input: &str) -> Vec<Vec<char>> {
        let mut grid = Vec::new();

        for line in input.lines() {
            let mut row = Vec::with_capacity(line.len());

            for tile in line.chars() {
                row.push(tile);
            }

            grid.push(row);
        }

        grid
    }

    fn get_amount_neighbors_on(grid: &Vec<Vec<char>>, y: usize, x: usize) -> u8 {
        let mut amount_neighbors_on = 0;

        if y > 0 && x > 0 && grid[y - 1][x - 1] == '#' { amount_neighbors_on += 1; }
        if y > 0 && grid[y - 1][x] == '#' { amount_neighbors_on += 1; }
        if y > 0 && x < grid[y].len() - 1 && grid[y - 1][x + 1] == '#' { amount_neighbors_on += 1; }

        if x > 0 && grid[y][x - 1] == '#' { amount_neighbors_on += 1; }
        if x < grid[y].len() - 1 && grid[y][x + 1] == '#' { amount_neighbors_on += 1; }

        if y < grid.len() - 1 && x > 0 && grid[y + 1][x - 1] == '#' { amount_neighbors_on += 1; }
        if y < grid.len() - 1 && grid[y + 1][x] == '#' { amount_neighbors_on += 1; }
        if y < grid.len() - 1 && x < grid[0].len() - 1 && grid[y + 1][x + 1] == '#' { amount_neighbors_on += 1; }

        amount_neighbors_on
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

        assert_eq!(solve(&input, 4), 4);
    }
}
