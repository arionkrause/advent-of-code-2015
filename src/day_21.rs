use regex::Regex;

struct Player {
    hit_points: u8,
    damage_score: u8,
    armor_score: u8,
    amount_gold_spent: u16,
}

impl Player {
    fn new(weapon: &Weapon, armor: &Option<Armor>, ring_1: &Option<Ring>, ring_2: &Option<Ring>) -> Player {
        Player {
            hit_points: 100,
            damage_score: Player::get_damage_score(&weapon, &ring_1, &ring_2),
            armor_score: Player::get_armor_score(&armor, &ring_1, &ring_2),
            amount_gold_spent: Player::get_amount_gold_spent(&weapon, &armor, &ring_1, &ring_2),
        }
    }

    fn get_damage_score(weapon: &Weapon, ring_1: &Option<Ring>, ring_2: &Option<Ring>) -> u8 {
        let mut damage_score = 0;
        damage_score += weapon.damage;

        match ring_1 {
            Some(ring) => match ring.damage {
                Some(damage) => damage_score += damage,
                None => {}
            },
            None => {}
        }

        match ring_2 {
            Some(ring) => match ring.damage {
                Some(damage) => damage_score += damage,
                None => {}
            },
            None => {}
        }

        damage_score
    }

    fn get_armor_score(armor: &Option<Armor>, ring_1: &Option<Ring>, ring_2: &Option<Ring>) -> u8 {
        let mut armor_score = 0;

        match armor {
            Some(armor) => armor_score += armor.armor,
            None => {}
        }

        match ring_1 {
            Some(ring) => match ring.armor {
                Some(armor) => armor_score += armor,
                None => {}
            },
            None => {}
        }

        match ring_2 {
            Some(ring) => match ring.armor {
                Some(armor) => armor_score += armor,
                None => {}
            },
            None => {}
        }

        armor_score
    }

    fn get_amount_gold_spent(weapon: &Weapon, armor: &Option<Armor>, ring_1: &Option<Ring>, ring_2: &Option<Ring>) -> u16 {
        let mut amount_gold_spent = 0;
        amount_gold_spent += weapon.cost as u16;

        match armor {
            Some(armor) => amount_gold_spent += armor.cost as u16,
            None => {}
        }

        match ring_1 {
            Some(ring) => amount_gold_spent += ring.cost as u16,
            None => {}
        }

        match ring_2 {
            Some(ring) => amount_gold_spent += ring.cost as u16,
            None => {}
        }

        amount_gold_spent
    }
}

struct Weapon {
    cost: u8,
    damage: u8,
}

struct Armor {
    cost: u8,
    armor: u8,
}

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

struct Boss {
    hit_points: u8,
    damage_score: u8,
    armor_score: u8,
}

fn player_wins_the_battle(player: &Player, boss: &Boss) -> bool {
    let player_damage = match player.damage_score.checked_sub(boss.armor_score) {
        Some(damage) => damage,
        None => 1,
    };

    let boss_damage = match boss.damage_score.checked_sub(player.armor_score) {
        Some(damage) => damage,
        None => 1,
    };

    (player.hit_points as f32 / boss_damage as f32).ceil() >= (boss.hit_points as f32 / player_damage as f32).ceil()
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

fn get_players() -> Vec<Player> {
    let weapons = get_weapons();
    let armors = get_armors();
    let rings = get_rings();

    let mut possible_armors = armors.into_iter().map(|a| Some(a)).collect::<Vec<Option<Armor>>>();
    possible_armors.push(None);

    let mut possible_rings = rings.into_iter().map(|r| Some(r)).collect::<Vec<Option<Ring>>>();
    possible_rings.push(None);

    let mut players = Vec::new();

    for weapon in weapons.iter() {
        for armor in possible_armors.iter() {
            for ring_1 in possible_rings.iter() {
                players.push(Player::new(&weapon, &armor, &ring_1, &None));

                if ring_1.is_some() {
                    for ring_2 in possible_rings.iter() {
                        if ring_2.is_none() || ring_1 == ring_2 {
                            continue;
                        }

                        players.push(Player::new(&weapon, &armor, &ring_1, &ring_2));
                    }
                }
            }
        }
    }

    players
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
            cost: 8,
            damage: 4,
        },
        Weapon {
            cost: 10,
            damage: 5,
        },
        Weapon {
            cost: 25,
            damage: 6,
        },
        Weapon {
            cost: 40,
            damage: 7,
        },
        Weapon {
            cost: 74,
            damage: 8,
        },
    ]
}

fn get_armors() -> Vec<Armor> {
    vec![
        Armor {
            cost: 13,
            armor: 1,
        },
        Armor {
            cost: 31,
            armor: 2,
        },
        Armor {
            cost: 53,
            armor: 3,
        },
        Armor {
            cost: 75,
            armor: 4,
        },
        Armor {
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
    use crate::day_21::decode_input;
    use crate::day_21::get_players;
    use crate::day_21::player_wins_the_battle;

    pub fn solve(input: &str) -> u16 {
        let boss = decode_input(&input);
        let mut players = get_players();
        players.sort_by(|a, b| a.amount_gold_spent.cmp(&b.amount_gold_spent));

        for player in players.iter() {
            if player_wins_the_battle(&player, &boss) {
                return player.amount_gold_spent;
            }
        }

        unreachable!()
    }
}

mod part_2 {
    use crate::day_21::decode_input;
    use crate::day_21::get_players;
    use crate::day_21::player_wins_the_battle;

    pub fn solve(input: &str) -> u16 {
        let boss = decode_input(&input);
        let mut players = get_players();
        players.sort_by(|a, b| a.amount_gold_spent.cmp(&b.amount_gold_spent).reverse());

        for player in players.iter() {
            if !player_wins_the_battle(&player, &boss) {
                return player.amount_gold_spent;
            }
        }

        unreachable!()
    }
}
