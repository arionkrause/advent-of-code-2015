use regex::Regex;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    player: Player,
    boss: Boss,
    amount_mana_points_spent: u16,
    log: Vec<String>,
}

impl State {
    fn get_initial_state(input: &str, player_hit_points: u8, player_mana: u16) -> State {
        let boss = decode_input(&input);

        let player = Player {
            hit_points: player_hit_points,
            mana_points: player_mana,
            effects: vec![],
        };

        State {
            player,
            boss,
            amount_mana_points_spent: 0,
            log: vec![],
        }
    }

    fn get_next_states(&self, spells: &Vec<Spell>, effects: &Vec<Effect>, print_log: bool) -> Vec<State> {
        let mut next_states = Vec::new();

        let mut state_clone: State = self.clone().into();
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
                    },
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
    println!("Part 1: {}.", part_1::solve(&input, 50, 500, false));
//    println!("Part 2: {}.", part_2::solve(&input));
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

mod part_1 {
    use crate::day_22::Effect;
    use crate::day_22::Spell;
    use std::collections::BinaryHeap;
    use crate::day_22::State;
    use crate::day_22::get_spells;
    use crate::day_22::get_effects;

    pub fn solve(input: &str, player_hit_points: u8, player_mana: u16, print_log: bool) -> u16 {
        let mut initial_state = State::get_initial_state(&input, player_hit_points, player_mana);

        if print_log {
            initial_state.log.push(State::get_log(&initial_state));
        }

        let spells = get_spells();
        let effects = get_effects();
        get_minimum_amount_mana_points_spent(&initial_state, &spells, &effects, print_log)
    }

    fn get_minimum_amount_mana_points_spent(initial_state: &State, spells: &Vec<Spell>, effects: &Vec<Effect>, print_log: bool) -> u16 {
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
                heap.extend(state.get_next_states(&spells, &effects, print_log));
            }
        }

        minimum_amount_mana_points_spent.unwrap()
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "Hit Points: 13
Damage: 8";

        assert_eq!(solve(input, 10, 250, true), 226);
    }

    #[cfg(test)]
    #[test]
    fn test_2() {
        let input = "Hit Points: 14
Damage: 8";

        assert_eq!(solve(input, 10, 250, true), 641);
    }
}
