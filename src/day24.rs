use std::{cmp::Reverse, str::FromStr};

use regex::Regex;

lazy_static! {
    static ref WEAK_TO_RGX: Regex = Regex::new(r"weak to ((?:\w+,? ?)*)").unwrap();
    static ref IMMUNE_TO_RGX: Regex = Regex::new(r"immune to ((?:\w+,? ?)*)").unwrap();
    static ref RGX: Regex = Regex::new(r"(?P<units>\d+) units each with (?P<hp>\d+) hit points.*? with an attack that does (?P<damage>\d+) (?P<damage_type>\w+) damage at initiative (?P<initiative>\d+)").unwrap();
}

const INPUT: &str = include_str!("../input/day24.txt");

#[derive(Clone, Copy, PartialEq, Debug)]
enum DamageType {
    Bludgeoning,
    Slashing,
    Fire,
    Cold,
    Radiation,
}

impl FromStr for DamageType {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let damage_type = match s {
            "bludgeoning" => DamageType::Bludgeoning,
            "slashing" => DamageType::Slashing,
            "fire" => DamageType::Fire,
            "cold" => DamageType::Cold,
            "radiation" => DamageType::Radiation,
            _ => panic!("Invalid damage type: {}", s),
        };
        Ok(damage_type)
    }
}

#[derive(Clone, PartialEq, Debug)]
struct UnitGroup {
    pub number: usize,
    pub hp: usize,
    pub weak_to: Vec<DamageType>,
    pub immune_to: Vec<DamageType>,
    pub damage_type: DamageType,
    pub damage: usize,
    pub initiative: usize,
}

impl UnitGroup {
    pub fn effective_power(&self) -> usize { self.number * self.damage }

    pub fn calc_damage_to(&self, other: &Self) -> usize {
        if other
            .immune_to
            .iter()
            .any(|&immune_to| immune_to == self.damage_type)
        {
            return 0;
        }
        let double_damage = other
            .weak_to
            .iter()
            .any(|&weak_to| weak_to == self.damage_type);

        if double_damage {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }

    pub fn take_damage(&mut self, attacker: &Self) {
        let damage_taken = attacker.calc_damage_to(self);
        let units_lost = ((damage_taken as f64) / (self.hp as f64)).trunc() as usize;
        self.number = self.number.saturating_sub(units_lost);
    }
}

fn parse_damage_types<'a>(rgx: &'static Regex, s: &str) -> Vec<DamageType> {
    rgx.captures(s)
        .and_then(|cap| cap.get(1))
        .map(|damage_types| -> Vec<DamageType> {
            damage_types
                .as_str()
                .split(", ")
                .map(DamageType::from_str)
                .map(Result::unwrap)
                .collect()
        })
        .unwrap_or_else(Vec::new)
}

impl FromStr for UnitGroup {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let caps = RGX.captures(&s).unwrap();
        let weak_to: Vec<DamageType> = parse_damage_types(&WEAK_TO_RGX, s);
        let immune_to: Vec<DamageType> = parse_damage_types(&IMMUNE_TO_RGX, s);

        let units = UnitGroup {
            number: caps["units"].parse().unwrap(),
            hp: caps["hp"].parse().unwrap(),
            weak_to,
            immune_to,
            damage_type: DamageType::from_str(&caps["damage_type"]).unwrap(),
            damage: caps["damage"].parse().unwrap(),
            initiative: caps["initiative"].parse().unwrap(),
        };
        Ok(units)
    }
}

fn parse_input() -> (Vec<UnitGroup>, Vec<UnitGroup>) {
    let immune_system_units = INPUT
        .lines()
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(UnitGroup::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let infection_units = INPUT
        .lines()
        .skip_while(|&l| l != "Infection:")
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(UnitGroup::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    (immune_system_units, infection_units)
}

fn debug_group(group: &UnitGroup) -> String {
    format!("(damage {}) contains {} units", group.damage, group.number)
}

fn part1(debug: bool) -> usize {
    let (mut immune_system_units, mut infection_units) = parse_input();
    if debug {
        println!("{:?}", (&immune_system_units, &infection_units));
    }

    while !immune_system_units.is_empty() && !infection_units.is_empty() {
        immune_system_units
            .sort_by_key(|group| (Reverse(group.effective_power()), Reverse(group.initiative)));
        infection_units
            .sort_by_key(|group| (Reverse(group.effective_power()), Reverse(group.initiative)));

        if debug {
            println!("\nImmune System: ");
            for (i, group) in immune_system_units.iter().enumerate() {
                println!("Group {} {}", i + 1, debug_group(group));
            }
            println!("Infection: ");
            for (i, group) in infection_units.iter().enumerate() {
                println!("Group {} {}", i + 1, debug_group(group));
            }
            println!();
        }

        let (mut immune_system_targets, mut infection_targets) = (
            Vec::with_capacity(immune_system_units.len()),
            Vec::with_capacity(infection_units.len()),
        );
        while immune_system_targets.len() != immune_system_units.len()
            || infection_targets.len() != infection_units.len()
        {
            let next_immune_to_pick = immune_system_units
                .get(immune_system_targets.len())
                .map(|group| (group.effective_power(), group.initiative))
                .unwrap_or((0, 0));
            let next_infection_to_pick = infection_units
                .get(infection_targets.len())
                .map(|group| (group.effective_power(), group.initiative))
                .unwrap_or((0, 0));

            if next_immune_to_pick > next_infection_to_pick {
                let group = immune_system_units
                    .get(immune_system_targets.len())
                    .unwrap();
                let target_group_opt = infection_units
                    .iter()
                    .enumerate()
                    .filter(|&(target_i, _)| {
                        immune_system_targets
                            .iter()
                            .all(|&existing_target_i_opt| Some(target_i) != existing_target_i_opt)
                    })
                    .max_by_key(|(_i, target_group)| {
                        (
                            group.calc_damage_to(target_group),
                            target_group.effective_power(),
                            target_group.initiative,
                        )
                    });
                immune_system_targets.push(target_group_opt.map(|(i, _)| i));
            } else {
                let group = infection_units.get(infection_targets.len()).unwrap();
                let target_group_opt = immune_system_units
                    .iter()
                    .enumerate()
                    .filter(|&(target_i, _)| {
                        infection_targets
                            .iter()
                            .all(|&existing_target_i_opt| Some(target_i) != existing_target_i_opt)
                    })
                    .max_by_key(|(_i, target_group)| {
                        (
                            group.calc_damage_to(target_group),
                            target_group.effective_power(),
                            target_group.initiative,
                        )
                    });
                infection_targets.push(target_group_opt.map(|(i, _)| i));
            }
        }
        assert_eq!(immune_system_units.len(), immune_system_targets.len());
        assert_eq!(infection_units.len(), infection_targets.len());

        if debug {
            println!(
                "(immune_targets, infection_targets): {:?}",
                (&immune_system_targets, &infection_targets)
            );
        }

        // (is_immune, src_index, target_ix)
        let mut attacking_units: Vec<(bool, usize, usize)> = immune_system_targets
            .into_iter()
            .enumerate()
            .filter_map(|(src_ix, target_ix_opt)| {
                target_ix_opt.map(|target_ix| (true, src_ix, target_ix))
            })
            .chain(infection_targets.into_iter().enumerate().filter_map(
                |(src_ix, target_ix_opt)| target_ix_opt.map(|target_ix| (false, src_ix, target_ix)),
            ))
            .collect();
        attacking_units.sort_by_key(|&(is_immune, src_ix, _)| {
            let src_vec = if is_immune {
                &immune_system_units
            } else {
                &infection_units
            };
            let src_unit = &src_vec[src_ix];
            Reverse(src_unit.initiative)
        });

        for (is_immune, src_ix, target_ix) in attacking_units {
            let (src_vec, target_vec) = if is_immune {
                (&immune_system_units, &mut infection_units)
            } else {
                (&infection_units, &mut immune_system_units)
            };

            let before_units = target_vec[target_ix].number;
            target_vec[target_ix].take_damage(&src_vec[src_ix]);
            if debug {
                println!(
                    "{} Group {} attacks enemy group {}, killing {} units.",
                    if is_immune { "Immune" } else { "Infection" },
                    src_ix + 1,
                    target_ix + 1,
                    before_units - target_vec[target_ix].number
                );
            }
        }

        immune_system_units.retain(|group| group.number > 0);
        infection_units.retain(|group| group.number > 0);
    }

    let winning_army = if immune_system_units.is_empty() {
        infection_units
    } else {
        immune_system_units
    };
    winning_army.into_iter().map(|group| group.number).sum()
}

fn part2(debug: bool) -> usize {
    let mut boost = 0;
    'outer: loop {
        boost += 1;

        let (mut immune_system_units, mut infection_units) = parse_input();
        for group in immune_system_units.iter_mut() {
            group.damage += boost;
        }

        if debug {
            println!("{:?}", (&immune_system_units, &infection_units));
        }

        while !immune_system_units.is_empty() && !infection_units.is_empty() {
            immune_system_units
                .sort_by_key(|group| (Reverse(group.effective_power()), Reverse(group.initiative)));
            infection_units
                .sort_by_key(|group| (Reverse(group.effective_power()), Reverse(group.initiative)));

            if debug {
                println!("\nImmune System: ");
                for (i, group) in immune_system_units.iter().enumerate() {
                    println!("Group {} {}", i + 1, debug_group(group));
                }
                println!("Infection: ");
                for (i, group) in infection_units.iter().enumerate() {
                    println!("Group {} {}", i + 1, debug_group(group));
                }
                println!();
            }

            let (mut immune_system_targets, mut infection_targets) = (
                Vec::with_capacity(immune_system_units.len()),
                Vec::with_capacity(infection_units.len()),
            );
            while immune_system_targets.len() != immune_system_units.len()
                || infection_targets.len() != infection_units.len()
            {
                let next_immune_to_pick = immune_system_units
                    .get(immune_system_targets.len())
                    .map(|group| (group.effective_power(), group.initiative))
                    .unwrap_or((0, 0));
                let next_infection_to_pick = infection_units
                    .get(infection_targets.len())
                    .map(|group| (group.effective_power(), group.initiative))
                    .unwrap_or((0, 0));

                if next_immune_to_pick > next_infection_to_pick {
                    let group = immune_system_units
                        .get(immune_system_targets.len())
                        .unwrap();
                    let target_group_opt = infection_units
                        .iter()
                        .enumerate()
                        .filter(|&(target_i, _)| {
                            immune_system_targets.iter().all(|&existing_target_i_opt| {
                                Some(target_i) != existing_target_i_opt
                            })
                        })
                        .map(|(i, target_group)| {
                            (i, target_group, group.calc_damage_to(target_group))
                        })
                        .filter(|&(_, _, damage)| damage > 0)
                        .max_by_key(|&(i, target_group, damage)| {
                            if debug {
                                println!(
                                    "Immune group {} would deal enemy group {} {} damage.",
                                    infection_targets.len(),
                                    i,
                                    damage
                                );
                            }

                            (
                                damage,
                                target_group.effective_power(),
                                target_group.initiative,
                            )
                        });
                    immune_system_targets.push(target_group_opt.map(|(i, ..)| i));
                } else {
                    let group = infection_units.get(infection_targets.len()).unwrap();
                    let target_group_opt = immune_system_units
                        .iter()
                        .enumerate()
                        .filter(|&(target_i, _)| {
                            infection_targets.iter().all(|&existing_target_i_opt| {
                                Some(target_i) != existing_target_i_opt
                            })
                        })
                        .map(|(i, target_group)| {
                            (i, target_group, group.calc_damage_to(target_group))
                        })
                        .filter(|&(_, _, damage)| damage > 0)
                        .max_by_key(|&(i, target_group, damage)| {
                            if debug {
                                println!(
                                    "Infection group {} would deal enemy group {} {} damage.",
                                    infection_targets.len(),
                                    i,
                                    damage
                                );
                            }

                            (
                                damage,
                                target_group.effective_power(),
                                target_group.initiative,
                            )
                        });
                    infection_targets.push(target_group_opt.map(|(i, ..)| i));
                }
            }

            if debug {
                println!(
                    "(immune_targets, infection_targets): {:?}",
                    (&immune_system_targets, &infection_targets)
                );
            }

            // (is_immune, src_index, target_ix)
            let mut attacking_units: Vec<(bool, usize, usize)> = immune_system_targets
                .into_iter()
                .enumerate()
                .filter_map(|(src_ix, target_ix_opt)| {
                    target_ix_opt.map(|target_ix| (true, src_ix, target_ix))
                })
                .chain(infection_targets.into_iter().enumerate().filter_map(
                    |(src_ix, target_ix_opt)| {
                        target_ix_opt.map(|target_ix| (false, src_ix, target_ix))
                    },
                ))
                .collect();
            attacking_units.sort_by_key(|&(is_immune, src_ix, _)| {
                let src_vec = if is_immune {
                    &immune_system_units
                } else {
                    &infection_units
                };
                let src_unit = &src_vec[src_ix];
                Reverse(src_unit.initiative)
            });

            let mut total_units_killed = 0;
            for (is_immune, src_ix, target_ix) in attacking_units {
                let (src_vec, target_vec) = if is_immune {
                    (&immune_system_units, &mut infection_units)
                } else {
                    (&infection_units, &mut immune_system_units)
                };

                let before_units = target_vec[target_ix].number;
                target_vec[target_ix].take_damage(&src_vec[src_ix]);
                if debug {
                    println!(
                        "{} Group {} (initiative {}) attacks enemy group {}, killing {} units.",
                        if is_immune { "Immune" } else { "Infection" },
                        src_ix + 1,
                        src_vec[src_ix].initiative,
                        target_ix + 1,
                        before_units - target_vec[target_ix].number
                    );
                }
                total_units_killed += before_units - target_vec[target_ix].number;
            }
            if total_units_killed == 0 {
                continue 'outer;
            }

            immune_system_units.retain(|group| group.number > 0);
            infection_units.retain(|group| group.number > 0);
        }

        if !immune_system_units.is_empty() {
            return immune_system_units
                .into_iter()
                .map(|group| group.number)
                .sum();
        }
    }
}

pub fn run() {
    println!("Part 1: {}", part1(false));
    println!("Part 2: {}", part2(false));
}
