use regex::Regex;

struct Player {
    hit_points: u8,
    damage_score: u8,
    armor_score: u8,
//    weapon: Weapon,
//    armor: Option<Armor>,
//    ring_1: Option<Ring>,
//    ring_2: Option<Ring>,
    gold_spent: u16,
}

impl Player {
    fn new(weapon: &Weapon, armor: &Option<Armor>, ring_1: &Option<Ring>, ring_2: &Option<Ring>) -> Player {
        Player {
            hit_points: 100,
            damage_score: Player::get_damage_score(&weapon, &ring_1, &ring_2),
            armor_score: Player::get_armor_score(&armor, &ring_1, &ring_2),
//            weapon: weapon.clone(),
//            armor: armor.clone(),
//            ring_1: ring_1.clone(),
//            ring_2: ring_2.clone(),
            gold_spent: Player::get_gold_spent(&weapon, &armor, &ring_1, &ring_2),
        }
    }

    fn get_damage_score(weapon: &Weapon, ring_1: &Option<Ring>, ring_2: &Option<Ring>) -> u8 {
        let mut damage_score = 0;
        damage_score += weapon.damage;

        if ring_1.is_some() {
            let ring_1_clone = ring_1.clone().unwrap();

            if ring_1_clone.damage.is_some() {
                damage_score += ring_1_clone.damage.unwrap();
            }
        }

        if ring_2.is_some() {
            let ring_2_clone = ring_2.clone().unwrap();

            if ring_2_clone.damage.is_some() {
                damage_score += ring_2_clone.damage.unwrap();
            }
        }

        damage_score
    }

    fn get_armor_score(armor: &Option<Armor>, ring_1: &Option<Ring>, ring_2: &Option<Ring>) -> u8 {
        let mut armor_score = 0;

        if armor.is_some() {
            armor_score += armor.clone().unwrap().armor;
        }

        if ring_1.is_some() {
            let ring_1_clone = ring_1.clone().unwrap();

            if ring_1_clone.armor.is_some() {
                armor_score += ring_1_clone.armor.unwrap();
            }
        }

        if ring_2.is_some() {
            let ring_2_clone = ring_2.clone().unwrap();

            if ring_2_clone.armor.is_some() {
                armor_score += ring_2_clone.armor.unwrap();
            }
        }

        armor_score
    }

    fn get_gold_spent(weapon: &Weapon, armor: &Option<Armor>, ring_1: &Option<Ring>, ring_2: &Option<Ring>) -> u16 {
        let mut gold_spent = 0;
        gold_spent += weapon.cost as u16;

        if armor.is_some() {
            gold_spent += armor.clone().unwrap().cost as u16;
        }

        if ring_1.is_some() {
            gold_spent += ring_1.clone().unwrap().cost as u16;
        }

        if ring_2.is_some() {
            gold_spent += ring_2.clone().unwrap().cost as u16;
        }

        gold_spent
    }
}

#[derive(Clone)]
struct Weapon {
    name: String,
    cost: u8,
    damage: u8,
}

#[derive(Clone)]
struct Armor {
    name: String,
    cost: u8,
    armor: u8,
}

#[derive(Clone)]
struct Ring {
    name: String,
    cost: u8,
    damage: Option<u8>,
    armor: Option<u8>,
}

impl PartialEq for Ring {
    fn eq(&self, other: &Ring) -> bool {
        self.name == other.name
    }
}

#[derive(Clone)]
struct Boss {
    hit_points: u8,
    damage_score: u8,
    armor_score: u8,
}

struct Battle {
    player: Player,
    boss: Boss,
}

impl Battle {
    fn new(player: Player, boss: Boss) -> Battle {
        Battle {
            player,
            boss,
        }
    }

    fn fight_until_battle_ends_and_return_true_if_player_has_won(&mut self) -> bool {
        loop {
            let damage = match self.player.damage_score.checked_sub(self.boss.armor_score) {
                Some(damage) => damage,
                None => 1,
            };

            self.boss.hit_points = match self.boss.hit_points.checked_sub(damage) {
                Some(hit_points) => hit_points,
                None => return true,
            };

            let damage = match self.boss.damage_score.checked_sub(self.player.armor_score) {
                Some(damage) => damage,
                None => 1,
            };

            self.player.hit_points = match self.player.hit_points.checked_sub(damage) {
                Some(hit_points) => hit_points,
                None => return false,
            };
        }
    }
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn get_battles(input: &str) -> Vec<Battle> {
    let boss = decode_input(&input);
    let weapons = get_weapons();
    let armors = get_armors();
    let rings = get_rings();

    let mut possible_armors = armors.into_iter().map(|a| Some(a)).collect::<Vec<Option<Armor>>>();
    possible_armors.push(None);

    let mut possible_rings = rings.into_iter().map(|r| Some(r)).collect::<Vec<Option<Ring>>>();
    possible_rings.push(None);

    let mut battles = Vec::new();

    for weapon in weapons.iter() {
        for armor in possible_armors.iter() {
            for ring_1 in possible_rings.iter() {
                battles.push(Battle::new(Player::new(&weapon, &armor, &ring_1, &None), boss.clone()));

                if ring_1.is_some() {
                    for ring_2 in possible_rings.iter() {
                        if ring_2.is_none() || ring_1 == ring_2 {
                            continue;
                        }

                        battles.push(Battle::new(Player::new(&weapon, &armor, &ring_1, &ring_2), boss.clone()));
                    }
                }
            }
        }
    }

    battles
}

fn decode_input(input: &str) -> Boss {
    let captures = Regex::new(r"^Hit Points: (?P<hit_points>\d+)\nDamage: (?P<damage>\d+)\nArmor: (?P<armor>\d+)$").unwrap().captures(&input).unwrap();

    Boss {
        hit_points: captures.name("hit_points").unwrap().as_str().parse().unwrap(),
        damage_score: captures.name("damage").unwrap().as_str().parse().unwrap(),
        armor_score: captures.name("armor").unwrap().as_str().parse().unwrap(),
    }
}

fn get_weapons() -> Vec<Weapon> {
    vec![
        Weapon {
            name: "Dagger".to_string(),
            cost: 8,
            damage: 4,
        },
        Weapon {
            name: "Shortsword".to_string(),
            cost: 10,
            damage: 5,
        },
        Weapon {
            name: "Warhammer".to_string(),
            cost: 25,
            damage: 6,
        },
        Weapon {
            name: "Longsword".to_string(),
            cost: 40,
            damage: 7,
        },
        Weapon {
            name: "Greataxe".to_string(),
            cost: 74,
            damage: 8,
        },
    ]
}

fn get_armors() -> Vec<Armor> {
    vec![
        Armor {
            name: "Leather".to_string(),
            cost: 13,
            armor: 1,
        },
        Armor {
            name: "Chainmail".to_string(),
            cost: 31,
            armor: 2,
        },
        Armor {
            name: "Splintmail".to_string(),
            cost: 53,
            armor: 3,
        },
        Armor {
            name: "Bandedmail".to_string(),
            cost: 75,
            armor: 4,
        },
        Armor {
            name: "Platemail".to_string(),
            cost: 102,
            armor: 5,
        },
    ]
}

fn get_rings() -> Vec<Ring> {
    vec![
        Ring {
            name: "Damage +1".to_string(),
            cost: 25,
            damage: Some(1),
            armor: None,
        },
        Ring {
            name: "Damage +2".to_string(),
            cost: 50,
            damage: Some(2),
            armor: None,
        },
        Ring {
            name: "Damage +3".to_string(),
            cost: 100,
            damage: Some(3),
            armor: None,
        },
        Ring {
            name: "Defense +1".to_string(),
            cost: 20,
            damage: None,
            armor: Some(1),
        },
        Ring {
            name: "Defense +2".to_string(),
            cost: 40,
            damage: None,
            armor: Some(2),
        },
        Ring {
            name: "Defense +3".to_string(),
            cost: 80,
            damage: None,
            armor: Some(3),
        },
    ]
}

mod part_1 {
    use rayon::prelude::*;
    use crate::day_21::get_battles;

    pub fn solve(input: &str) -> u16 {
        *get_battles(&input).par_iter_mut().map(|battle| {
            if battle.fight_until_battle_ends_and_return_true_if_player_has_won() {
                Some(battle.player.gold_spent)
            } else {
                None
            }
        }).collect::<Vec<Option<u16>>>()
                .iter()
                .filter(|result| result.is_some())
                .map(|result| result.unwrap())
                .collect::<Vec<u16>>()
                .iter()
                .min()
                .unwrap()
    }
}

mod part_2 {
    use rayon::prelude::*;
    use crate::day_21::get_battles;

    pub fn solve(input: &str) -> u16 {
        *get_battles(&input).par_iter_mut().map(|battle| {
            if battle.fight_until_battle_ends_and_return_true_if_player_has_won() {
                None
            } else {
                Some(battle.player.gold_spent)
            }
        }).collect::<Vec<Option<u16>>>()
                .iter()
                .filter(|result| result.is_some())
                .map(|result| result.unwrap())
                .collect::<Vec<u16>>()
                .iter()
                .max()
                .unwrap()
    }
}
