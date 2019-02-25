use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    player: Player,
    boss: Boss,
    amount_mana_points_spent: u16,
    log: Vec<String>,
}

impl State {
    fn get_initial_state(input: &str, player_hit_points: u8, player_mana_points: u16) -> State {
        let boss = decode_input(&input);

        let player = Player {
            hit_points: player_hit_points,
            mana_points: player_mana_points,
            effects: vec![],
        };

        State {
            player,
            boss,
            amount_mana_points_spent: 0,
            log: vec![],
        }
    }

    fn get_next_states(&self, spells: &Vec<Spell>, effects: &Vec<Effect>, print_log: bool, hard_mode: bool) -> Vec<State> {
        let mut next_states = Vec::new();

        let mut state_clone: State = self.clone().into();

        if hard_mode {
            state_clone.player.hit_points -= 1;

            if state_clone.player.hit_points == 0 {
                return next_states;
            }
        }

        state_clone.apply_effects(print_log);

        if state_clone.player_won() {
            next_states.push(state_clone);
        } else {
            for spell in spells.iter() {
                let mut state_clone: State = state_clone.clone();

                match state_clone.player_turn(&spell, &effects, print_log) {
                    Some(mut state_after_player_turn) => {
                        if print_log {
                            state_after_player_turn.log.push(format!("After Player turn. {:?}", State::get_log(&state_after_player_turn)));
                        }

                        if state_after_player_turn.player_won() {
                            next_states.push(state_after_player_turn);
                        } else {
                            state_after_player_turn.apply_effects(print_log);

                            if state_after_player_turn.player_won() {
                                next_states.push(state_after_player_turn);
                                continue;
                            }

                            match state_after_player_turn.boss_turn(print_log) {
                                Some(mut state_after_boss_turn) => {
                                    if print_log {
                                        state_after_boss_turn.log.push(format!("After Boss turn. {:?}", State::get_log(&state_after_boss_turn)));
                                    }

                                    next_states.push(state_after_boss_turn);
                                }
                                None => {}
                            }
                        }
                    }
                    None => {}
                }
            }
        }

        next_states
    }

    fn apply_effects(&mut self, print_log: bool) {
        if print_log {
            for effect in self.player.effects.iter() {
                match effect.effect_type {
                    EffectType::Recharge { amount } => {
                        self.log.push(format!("Recharge provides {} mana. Player's mana is now {}.", amount, self.player.mana_points + amount as u16))
                    }
                    _ => {}
                }

                if effect.timer == 1 {
                    self.log.push(format!("Player's {:?} has worn off.", effect.effect_type));
                }
            }

            for effect in self.boss.effects.iter() {
                match effect.effect_type {
                    EffectType::Poison { damage } => self.log.push(format!("Boss takes {} poison damage. Boss's HP is now {}.", damage, self.boss.hit_points as i32 - damage as i32)),
                    _ => {}
                }

                if effect.timer == 1 {
                    self.log.push(format!("Boss's {:?} has worn off.", effect.effect_type));
                }
            }
        }

        self.player.apply_effects();
        self.boss.apply_effects();
    }

    fn get_log(state: &State) -> String {
        format!("Player {{ HP: {:?}, Mana: {:?}, {:?} }}, Boss {{ HP: {:?}, {:?} }}, amount_mana_points_spent: {:?}",
                state.player.hit_points,
                state.player.mana_points,
                state.player.effects,
                state.boss.hit_points,
                state.boss.effects,
                state.amount_mana_points_spent
        ).to_string()
    }

    fn player_turn(&mut self, spell: &Spell, effects: &Vec<Effect>, print_log: bool) -> Option<State> {
        match self.player.mana_points.checked_sub(spell.cost as u16) {
            Some(new_value) => {
                self.player.mana_points = new_value;
                self.amount_mana_points_spent += spell.cost as u16;
            }
            None => return None,
        }

        match spell.spell_type {
            SpellType::MagicMissile { damage } => {
                match self.boss.hit_points.checked_sub(damage) {
                    Some(new_value) => self.boss.hit_points = new_value,
                    None => self.boss.hit_points = 0,
                }

                if print_log {
                    self.log.push(format!("Player casts Magic missile. Damage: {}. Boss's HP is now {}.", damage, self.boss.hit_points));
                }
            }
            SpellType::Drain { damage, heal } => {
                match self.boss.hit_points.checked_sub(damage) {
                    Some(new_value) => self.boss.hit_points = new_value,
                    None => self.boss.hit_points = 0,
                }

                self.player.hit_points += heal;

                if print_log {
                    self.log.push(format!("Player casts Drain. Damage: {}. Boss's HP is now {}.", damage, self.boss.hit_points));
                }
            }
            SpellType::Shield => {
                if self.player.effects.iter().any(|effect| {
                    match effect.effect_type {
                        EffectType::Shield { armor_increase: _ } => true,
                        _ => false,
                    }
                }) {
                    return None;
                }

                self.player.effects.push(effects.iter()
                        .filter(|&effect| match effect.effect_type {
                            EffectType::Shield { armor_increase: _ } => true,
                            _ => false,
                        })
                        .collect::<Vec<&Effect>>()
                        .first()
                        .unwrap()
                        .clone()
                        .to_owned());

                if print_log {
                    self.log.push(format!("Player casts Shield."));
                }
            }
            SpellType::Poison => {
                if self.boss.effects.iter().any(|effect| {
                    match effect.effect_type {
                        EffectType::Poison { damage: _ } => true,
                        _ => false,
                    }
                }) {
                    return None;
                }

                self.boss.effects.push(effects.iter()
                        .filter(|effect| match effect.effect_type {
                            EffectType::Poison { damage: _ } => true,
                            _ => false,
                        })
                        .collect::<Vec<&Effect>>()
                        .first()
                        .unwrap()
                        .clone()
                        .to_owned());

                if print_log {
                    self.log.push(format!("Player casts Poison."));
                }
            }
            SpellType::Recharge => {
                if self.player.effects.iter().any(|effect| {
                    match effect.effect_type {
                        EffectType::Recharge { amount: _ } => true,
                        _ => false,
                    }
                }) {
                    return None;
                }

                self.player.effects.push(effects.iter()
                        .filter(|effect| match effect.effect_type {
                            EffectType::Recharge { amount: _ } => true,
                            _ => false,
                        })
                        .collect::<Vec<&Effect>>()
                        .first()
                        .unwrap()
                        .clone()
                        .to_owned());

                if print_log {
                    self.log.push(format!("Player casts Recharge."));
                }
            }
        }

        return Some(self.clone());
    }

    fn boss_turn(&mut self, print_log: bool) -> Option<State> {
        let mut damage = self.boss.damage_score;

        for effect in self.player.effects.iter() {
            match effect.effect_type {
                EffectType::Shield { armor_increase } => damage -= armor_increase,
                _ => {}
            }
        }

        if print_log {
            self.log.push(format!("Boss attacks Player with {} damage. Player's HP is now {}.", damage, self.player.hit_points as i32 - damage as i32));
        }

        match self.player.hit_points.checked_sub(damage) {
            Some(new_hit_points) => {
                if new_hit_points == 0 {
                    return None;
                }

                self.player.hit_points = new_hit_points;
            }
            None => return None,
        }

        return Some(self.clone());
    }

    fn player_won(&self) -> bool {
        self.boss.hit_points == 0
    }

    fn boss_won(&self) -> bool {
        self.player.hit_points == 0
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.amount_mana_points_spent.cmp(&self.amount_mana_points_spent)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&State> for State {
    fn from(other: &State) -> Self {
        State {
            player: other.player.clone(),
            boss: other.boss.clone(),
            amount_mana_points_spent: other.amount_mana_points_spent,
            log: other.log.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Player {
    hit_points: u8,
    mana_points: u16,
    effects: Vec<Effect>,
}

impl Player {
    fn apply_effects(&mut self) {
        for effect in self.effects.iter_mut() {
            match effect.effect_type {
                EffectType::Recharge { amount } => self.mana_points += amount as u16,
                _ => {}
            }

            effect.timer -= 1;
        }

        self.effects.retain(|effect| effect.timer > 0);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Boss {
    hit_points: u8,
    damage_score: u8,
    effects: Vec<Effect>,
}

impl Boss {
    fn apply_effects(&mut self) {
        for effect in self.effects.iter_mut() {
            match effect.effect_type {
                EffectType::Poison { damage } => {
                    match self.hit_points.checked_sub(damage) {
                        Some(new_value) => self.hit_points = new_value,
                        None => self.hit_points = 0,
                    }
                }
                _ => {}
            }

            effect.timer -= 1;
        };

        self.effects.retain(|effect| effect.timer > 0);
    }
}

struct Spell {
    cost: u8,
    spell_type: SpellType,
}

enum SpellType {
    MagicMissile { damage: u8 },
    Drain { damage: u8, heal: u8 },
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Effect {
    timer: u8,
    effect_type: EffectType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum EffectType {
    Shield { armor_increase: u8 },
    Poison { damage: u8 },
    Recharge { amount: u8 },
}

pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 50, 500, false, false));
    println!("Part 2: {}.", part_2::solve(&input, 50, 500, false, true));
    println!();
}

fn decode_input(input: &str) -> Boss {
    let captures = Regex::new(r"^Hit Points: (?P<hit_points>\d+)\nDamage: (?P<damage>\d+)$").unwrap().captures(&input).unwrap();

    Boss {
        hit_points: captures.name("hit_points").unwrap().as_str().parse().unwrap(),
        damage_score: captures.name("damage").unwrap().as_str().parse().unwrap(),
        effects: vec![],
    }
}

fn get_spells() -> Vec<Spell> {
    vec![
        Spell { cost: 53, spell_type: SpellType::MagicMissile { damage: 4 } },
        Spell { cost: 73, spell_type: SpellType::Drain { damage: 2, heal: 2 } },
        Spell { cost: 113, spell_type: SpellType::Shield },
        Spell { cost: 173, spell_type: SpellType::Poison },
        Spell { cost: 229, spell_type: SpellType::Recharge },
    ]
}

fn get_effects() -> Vec<Effect> {
    vec![
        Effect { timer: 6, effect_type: EffectType::Shield { armor_increase: 7 } },
        Effect { timer: 6, effect_type: EffectType::Poison { damage: 3 } },
        Effect { timer: 5, effect_type: EffectType::Recharge { amount: 101 } },
    ]
}

fn get_minimum_amount_mana_points_spent(input: &str, player_hit_points: u8, player_mana_points: u16, print_log: bool, hard_mode: bool) -> u16 {
    let mut initial_state = State::get_initial_state(&input, player_hit_points, player_mana_points);

    if print_log {
        initial_state.log.push(State::get_log(&initial_state));
    }

    let spells = get_spells();
    let effects = get_effects();
    let mut minimum_amount_mana_points_spent = None;
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(initial_state.clone().into());

    while let Some(state) = heap.pop() {
        if match minimum_amount_mana_points_spent {
            Some(amount) => state.amount_mana_points_spent > amount,
            None => false
        } {
            continue;
        }

        if state.player_won() || state.boss_won() {
            if state.player_won() && match minimum_amount_mana_points_spent {
                Some(amount) => state.amount_mana_points_spent < amount,
                None => true,
            } {
                if print_log {
                    println!();

                    for (index, log) in state.log.iter().enumerate() {
                        println!("{}. {}", index + 1, log);
                    }
                }

                minimum_amount_mana_points_spent = Some(state.amount_mana_points_spent);
            }
        } else {
            heap.extend(state.get_next_states(&spells, &effects, print_log, hard_mode));
        }
    }

    minimum_amount_mana_points_spent.unwrap()
}

mod part_1 {
    use crate::day_22::get_minimum_amount_mana_points_spent;

    pub fn solve(input: &str, player_hit_points: u8, player_mana: u16, print_log: bool, hard_mode: bool) -> u16 {
        get_minimum_amount_mana_points_spent(&input, player_hit_points, player_mana, print_log, hard_mode)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "Hit Points: 13
Damage: 8";

        assert_eq!(solve(input, 10, 250, false, false), 226);
    }

    #[cfg(test)]
    #[test]
    fn test_2() {
        let input = "Hit Points: 14
Damage: 8";

        assert_eq!(solve(input, 10, 250, false, false), 641);
    }
}

mod part_2 {
    use crate::day_22::get_minimum_amount_mana_points_spent;

    pub fn solve(input: &str, player_hit_points: u8, player_mana: u16, print_log: bool, hard_mode: bool) -> u16 {
        get_minimum_amount_mana_points_spent(&input, player_hit_points, player_mana, print_log, hard_mode)
    }
}

// Procedural solving alternative
// It's ~32 times faster, but less maintainable
//
//use regex::Regex;
//use std::collections::VecDeque;
//
//#[derive(Clone, Debug)]
//struct State {
//    player_hit_points: i8,
//    player_mana_points: i16,
//    player_shield: bool,
//    player_shield_turns_left: i8,
//    player_recharge: bool,
//    player_recharge_turns_left: i8,
//    boss_hit_points: i8,
//    boss_damage: i8,
//    boss_poison: bool,
//    boss_poison_turns_left: i8,
//    amount_mana_points_spent: u16,
//}
//
//pub fn solve(input: &str) {
//    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
//    println!("Part 1: {}.", part_1::solve(&input));
//    println!("Part 2: {}.", part_2::solve(&input));
//    println!();
//}
//
//fn decode_input(input: &str) -> (i8, i8) {
//    let captures = Regex::new(r"^Hit Points: (?P<hit_points>\d+)\nDamage: (?P<damage>\d+)$").unwrap().captures(&input).unwrap();
//    (captures.name("hit_points").unwrap().as_str().parse().unwrap(), captures.name("damage").unwrap().as_str().parse().unwrap())
//}
//
//fn get_minimum_amount_mana_points_spent(input: &str, hard_mode: bool) -> u16 {
//    let (boss_hit_points, boss_damage) = decode_input(&input);
//    let mut minimum_amount_mana_points_spent = std::u16::MAX;
//    let mut queue: VecDeque<State> = VecDeque::new();
//
//    queue.push_back(State {
//        player_hit_points: 50,
//        player_mana_points: 500,
//        player_shield: false,
//        player_shield_turns_left: 0,
//        player_recharge: false,
//        player_recharge_turns_left: 0,
//        boss_hit_points,
//        boss_damage,
//        boss_poison: false,
//        boss_poison_turns_left: 0,
//        amount_mana_points_spent: 0,
//    });
//
//    while let Some(mut state) = queue.pop_front() {
//        if state.amount_mana_points_spent > minimum_amount_mana_points_spent {
//            continue;
//        }
//
//        if hard_mode {
//            state.player_hit_points -= 1;
//
//            if state.player_hit_points <= 0 {
//                continue;
//            }
//        }
//
//        // Apply effects
//        if state.player_recharge {
//            state.player_mana_points += 101;
//            state.player_recharge_turns_left -= 1;
//
//            if state.player_recharge_turns_left == 0 {
//                state.player_recharge = false;
//            }
//        }
//
//        if state.player_shield {
//            state.player_shield_turns_left -= 1;
//
//            if state.player_shield_turns_left == 0 {
//                state.player_shield = false;
//            }
//        }
//
//        if state.boss_poison {
//            state.boss_hit_points -= 3;
//
//            if state.boss_hit_points <= 0 {
//                if state.amount_mana_points_spent < minimum_amount_mana_points_spent {
//                    minimum_amount_mana_points_spent = state.amount_mana_points_spent;
//                }
//
//                continue;
//            }
//
//            state.boss_poison_turns_left -= 1;
//
//            if state.boss_poison_turns_left == 0 {
//                state.boss_poison = false;
//            }
//        }
//
//        // Player
//        for spell_index in 0..5 {
//            let mut state_clone = state.clone();
//
//            match spell_index {
//                0 => {
//                    state_clone.player_mana_points -= 53;
//
//                    if state_clone.player_mana_points < 0 {
//                        continue;
//                    }
//
//                    state_clone.amount_mana_points_spent += 53;
//                    state_clone.boss_hit_points -= 4;
//
//                    if state_clone.boss_hit_points <= 0 {
//                        if state_clone.amount_mana_points_spent < minimum_amount_mana_points_spent {
//                            minimum_amount_mana_points_spent = state_clone.amount_mana_points_spent;
//                        }
//
//                        continue;
//                    }
//                }
//
//                1 => {
//                    state_clone.player_mana_points -= 73;
//
//                    if state_clone.player_mana_points < 0 {
//                        continue;
//                    }
//
//                    state_clone.amount_mana_points_spent += 73;
//                    state_clone.boss_hit_points -= 2;
//
//                    if state_clone.boss_hit_points <= 0 {
//                        if state_clone.amount_mana_points_spent < minimum_amount_mana_points_spent {
//                            minimum_amount_mana_points_spent = state_clone.amount_mana_points_spent;
//                        }
//
//                        continue;
//                    }
//
//                    state_clone.player_hit_points += 2;
//                }
//
//                2 => {
//                    if state_clone.player_shield {
//                        continue;
//                    }
//
//                    state_clone.player_mana_points -= 113;
//
//                    if state_clone.player_mana_points < 0 {
//                        continue;
//                    }
//
//                    state_clone.amount_mana_points_spent += 113;
//                    state_clone.player_shield = true;
//                    state_clone.player_shield_turns_left = 6;
//                }
//
//                3 => {
//                    if state_clone.boss_poison {
//                        continue;
//                    }
//
//                    state_clone.player_mana_points -= 173;
//
//                    if state_clone.player_mana_points < 0 {
//                        continue;
//                    }
//
//                    state_clone.amount_mana_points_spent += 173;
//                    state_clone.boss_poison = true;
//                    state_clone.boss_poison_turns_left = 6;
//                }
//
//                4 => {
//                    if state_clone.player_recharge {
//                        continue;
//                    }
//
//                    state_clone.player_mana_points -= 229;
//
//                    if state_clone.player_mana_points < 0 {
//                        continue;
//                    }
//
//                    state_clone.amount_mana_points_spent += 229;
//                    state_clone.player_recharge = true;
//                    state_clone.player_recharge_turns_left = 5;
//                }
//
//                _ => {}
//            }
//
//            // Apply effects
//            if state_clone.player_recharge {
//                state_clone.player_mana_points += 101;
//                state_clone.player_recharge_turns_left -= 1;
//
//                if state_clone.player_recharge_turns_left == 0 {
//                    state_clone.player_recharge = false;
//                }
//            }
//
//            if state_clone.player_shield {
//                state_clone.player_shield_turns_left -= 1;
//
//                if state_clone.player_shield_turns_left == 0 {
//                    state_clone.player_shield = false;
//                }
//            }
//
//            if state_clone.boss_poison {
//                state_clone.boss_hit_points -= 3;
//
//                if state_clone.boss_hit_points <= 0 {
//                    if state_clone.amount_mana_points_spent < minimum_amount_mana_points_spent {
//                        minimum_amount_mana_points_spent = state_clone.amount_mana_points_spent;
//                    }
//
//                    continue;
//                }
//
//                state_clone.boss_poison_turns_left -= 1;
//
//                if state_clone.boss_poison_turns_left == 0 {
//                    state_clone.boss_poison = false;
//                }
//            }
//
//            // Boss
//            if state_clone.player_shield {
//                state_clone.player_hit_points -= state_clone.boss_damage - 7;
//            } else {
//                state_clone.player_hit_points -= state_clone.boss_damage;
//            }
//
//            if state_clone.player_hit_points <= 0 {
//                continue;
//            }
//
//            queue.push_back(state_clone);
//        }
//    }
//
//    minimum_amount_mana_points_spent
//}
//
//mod part_1 {
//    use crate::day_22::get_minimum_amount_mana_points_spent;
//
//    pub fn solve(input: &str) -> u16 {
//        get_minimum_amount_mana_points_spent(&input, false)
//    }
//}
//
//mod part_2 {
//    use crate::day_22::get_minimum_amount_mana_points_spent;
//
//    pub fn solve(input: &str) -> u16 {
//        get_minimum_amount_mana_points_spent(&input, true)
//    }
//}
