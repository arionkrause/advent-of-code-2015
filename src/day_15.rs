use regex::Regex;

struct Ingredient {
    capacity: i8,
    durability: i8,
    flavor: i8,
    texture: i8,
    calories: u8,
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn decode_input(input: &str) -> Vec<Ingredient> {
    let re = Regex::new(r"^\w+: capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)$").unwrap();
    let mut ingredients = Vec::new();

    for line in input.lines() {
        let captures = re.captures(&line).unwrap();

        ingredients.push(Ingredient {
            capacity: captures.name("capacity").unwrap().as_str().parse().unwrap(),
            durability: captures.name("durability").unwrap().as_str().parse().unwrap(),
            flavor: captures.name("flavor").unwrap().as_str().parse().unwrap(),
            texture: captures.name("texture").unwrap().as_str().parse().unwrap(),
            calories: captures.name("calories").unwrap().as_str().parse().unwrap(),
        })
    }

    ingredients
}

fn get_score(ingredients: &Vec<Ingredient>, amounts: &Vec<u8>) -> u32 {
    let mut total_capacity = 0;
    let mut total_durability = 0;
    let mut total_flavor = 0;
    let mut total_texture = 0;

    for (index, ingredient) in ingredients.iter().enumerate() {
        total_capacity += amounts[index] as i32 * ingredient.capacity as i32;
        total_durability += amounts[index] as i32 * ingredient.durability as i32;
        total_flavor += amounts[index] as i32 * ingredient.flavor as i32;
        total_texture += amounts[index] as i32 * ingredient.texture as i32;
    }

    if total_capacity < 0
            || total_durability < 0
            || total_flavor < 0
            || total_texture < 0 {
        return 0;
    }

    total_capacity as u32 * total_durability as u32 * total_flavor as u32 * total_texture as u32
}

mod part_1 {
    use crate::day_15::decode_input;
    use crate::day_15::get_score;

    pub fn solve(input: &str) -> u32 {
        let ingredients = decode_input(&input);
        let mut best_score = None;
        let mut amounts = vec![0; ingredients.len()];

        loop {
            let last_index = amounts.len() - 1;
            amounts[last_index] += 1;

            if amounts[amounts.len() - 1] > 100 {
                let mut index = amounts.len() - 1;
                amounts[index] = 0;
                index -= 1;

                loop {
                    amounts[index] += 1;

                    if amounts[index] > 100 {
                        if index == 0 {
                            return best_score.unwrap();
                        }

                        amounts[index] = 0;
                        index -= 1;
                        continue;
                    }

                    break;
                }
            }

            if amounts.iter().map(|a| *a as u16 ).sum::<u16>() == 100 {
                let score = get_score(&ingredients, &amounts);

                if best_score.is_none() || score > best_score.unwrap() {
                    best_score = Some(score);
                }
            }
        }
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"), 62842880);
    }
}

mod part_2 {
    use crate::day_15::Ingredient;
    use crate::day_15::decode_input;
    use crate::day_15::get_score;

    pub fn solve(input: &str) -> u32 {
        let ingredients = decode_input(&input);
        let mut best_score = None;
        let mut amounts = vec![0; ingredients.len()];

        loop {
            let last_index = amounts.len() - 1;
            amounts[last_index] += 1;

            if amounts[amounts.len() - 1] > 100 {
                let mut index = amounts.len() - 1;
                amounts[index] = 0;
                index -= 1;

                loop {
                    amounts[index] += 1;

                    if amounts[index] > 100 {
                        if index == 0 {
                            return best_score.unwrap();
                        }

                        amounts[index] = 0;
                        index -= 1;
                        continue;
                    }

                    break;
                }
            }

            if amounts.iter().map(|a| *a as u16 ).sum::<u16>() == 100 {
                let score = get_score(&ingredients, &amounts);

                if (best_score.is_none() || score > best_score.unwrap())
                        && get_calories_amount(&ingredients, &amounts) == 500 {
                    best_score = Some(score);
                }
            }
        }
    }

    fn get_calories_amount(ingredients: &Vec<Ingredient>, amounts: &Vec<u8>) -> u16 {
        ingredients.iter()
                .enumerate()
                .map(|(index, ingredient)| amounts[index] as u16 * ingredient.calories as u16)
                .sum()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"), 57600000);
    }
}
