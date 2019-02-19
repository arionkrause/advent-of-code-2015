pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 100));
    println!("Part 2: {}.", part_2::solve(&input, 100));
    println!();
}

fn decode_input(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    grid.push(vec!['.'; input.lines().next().unwrap().len() + 2]);

    for line in input.lines() {
        let mut row = Vec::with_capacity(line.len());
        row.push('.');

        for tile in line.chars() {
            row.push(tile);
        }

        row.push('.');
        grid.push(row);
    }

    grid.push(vec!['.'; input.lines().next().unwrap().len() + 2]);
    grid
}

fn get_amount_neighbors_on(grid: &Vec<Vec<char>>, y: usize, x: usize) -> u8 {
    let mut amount_neighbors_on = 0;

    if grid[y - 1][x - 1] == '#' { amount_neighbors_on += 1; }
    if grid[y - 1][x] == '#' { amount_neighbors_on += 1; }
    if grid[y - 1][x + 1] == '#' { amount_neighbors_on += 1; }

    if grid[y][x - 1] == '#' { amount_neighbors_on += 1; }
    if grid[y][x + 1] == '#' { amount_neighbors_on += 1; }

    if grid[y + 1][x - 1] == '#' { amount_neighbors_on += 1; }
    if grid[y + 1][x] == '#' { amount_neighbors_on += 1; }
    if grid[y + 1][x + 1] == '#' { amount_neighbors_on += 1; }

    amount_neighbors_on
}

mod part_1 {
    use crate::day_18::decode_input;
    use crate::day_18::get_amount_neighbors_on;

    pub fn solve(input: &str, steps: u8) -> u16 {
        let mut grid = decode_input(&input);
        let mut buffer_grid = grid.clone();

        for _ in 0..steps {
            for (y, row) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
                for (x, tile) in row.iter().enumerate().skip(1).take(row.len() - 2) {
                    let amount_neighbors_on = get_amount_neighbors_on(&grid, y, x);

                    if *tile == '#' && amount_neighbors_on != 2 && amount_neighbors_on != 3 {
                        buffer_grid[y][x] = '.';
                    } else if *tile == '.' && amount_neighbors_on == 3 {
                        buffer_grid[y][x] = '#';
                    } else {
                        buffer_grid[y][x] = *tile;
                    }
                }
            }

            std::mem::swap(&mut grid, &mut buffer_grid);
        }

        grid.iter().map(|row| row.iter().filter(|&tile| *tile == '#').count() as u16).sum()
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

mod part_2 {
    use crate::day_18::decode_input;
    use crate::day_18::get_amount_neighbors_on;

    pub fn solve(input: &str, steps: u8) -> u16 {
        let mut grid = decode_input(&input);
        let last_grid_y_index = grid.len() - 1;
        let last_grid_x_index = grid[0].len() - 1;
        grid[1][1] = '#';
        grid[1][last_grid_x_index - 1] = '#';
        grid[last_grid_y_index - 1][1] = '#';
        grid[last_grid_y_index - 1][last_grid_x_index - 1] = '#';
        let mut buffer_grid = grid.clone();

        for _ in 0..steps {
            for (y, row) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
                for (x, tile) in row.iter().enumerate().skip(1).take(row.len() - 2) {
                    if (y == 1 || y == grid.len() - 2) && (x == 1 || x == row.len() - 2) {
                        buffer_grid[y][x] = *tile;
                        continue;
                    }

                    let amount_neighbors_on = get_amount_neighbors_on(&grid, y, x);

                    if *tile == '#' && amount_neighbors_on != 2 && amount_neighbors_on != 3 {
                        buffer_grid[y][x] = '.';
                    } else if *tile == '.' && amount_neighbors_on == 3 {
                        buffer_grid[y][x] = '#';
                    } else {
                        buffer_grid[y][x] = *tile;
                    }
                }
            }

            std::mem::swap(&mut grid, &mut buffer_grid);
        }

        grid.iter().map(|row| row.iter().filter(|&tile| *tile == '#').count() as u16).sum()
    }
}
