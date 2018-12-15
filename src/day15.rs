extern crate pathfinding;

use std::usize;

use pathfinding::prelude::*;

const INPUT: &str = include_str!("../input/day15.txt");

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Elf(usize),
    Goblin(usize),
    Blank,
    Wall,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Cell::Elf(_) => 'E',
            Cell::Goblin(_) => 'G',
            Cell::Blank => '.',
            Cell::Wall => '#',
        };
        write!(fmt, "{}", c)
    }
}

impl Cell {
    pub fn is_enemy(self, other: Self) -> bool {
        match (self, other) {
            (Cell::Elf(_), Cell::Goblin(_)) | (Cell::Goblin(_), Cell::Elf(_)) => true,
            _ => false,
        }
    }

    pub fn get_attack_power(&self, elf_pow: usize) -> usize {
        match self {
            Cell::Elf(_) => elf_pow,
            Cell::Goblin(_) => GOBLIN_ATTACK_POWER,
            Cell::Blank => 0,
            Cell::Wall => 0,
        }
    }

    pub fn damage(&mut self, damage: usize) -> bool {
        match self {
            Cell::Elf(ref mut hp) | Cell::Goblin(ref mut hp) =>
                if *hp <= damage {
                    true
                } else {
                    *hp -= damage;
                    false
                },
            _ => panic!("Tried to damage non-living entity"),
        }
    }

    pub fn is_traversable(self) -> bool { self == Cell::Blank }
}

fn parse_input() -> impl Iterator<Item = Vec<Cell>> {
    INPUT.lines().map(|line| {
        line.chars()
            .map(|c| match c {
                '#' => Cell::Wall,
                '.' => Cell::Blank,
                'E' => Cell::Elf(200),
                'G' => Cell::Goblin(200),
                _ => unreachable!(),
            })
            .collect()
    })
}

fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let x_diff = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let y_diff = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    x_diff + y_diff
}

const GOBLIN_ATTACK_POWER: usize = 3;
const GRID_SIZE: usize = 32;

fn solve(
    cur_elf_attack_power: usize,
    elves_must_win: bool,
    debug: bool,
    debug_block: bool,
) -> Option<usize> {
    let mut state: Vec<_> = parse_input().collect();
    let mut grid = Grid::new(GRID_SIZE, GRID_SIZE);

    for (y, line) in state.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if cell.is_traversable() {
                grid.add_vertex((x, y));
            }
        }
    }

    let original_elf_count = state
        .iter()
        .flat_map(|l| l.into_iter())
        .filter(|c| c.is_enemy(Cell::Goblin(0)))
        .count();

    let mut rounds = 0;
    'main: loop {
        if debug {
            println!("{}", rounds);
            for l in &state {
                for c in l {
                    print!("{}", c);
                }
                println!()
            }
            println!("\n\n");
            if debug_block {
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }
        }

        let mut moved_entities = Vec::new();
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if moved_entities.iter().find(|n| **n == (x, y)).is_some() {
                    continue;
                }

                let cell = state[y][x];
                match cell {
                    Cell::Elf(_) | Cell::Goblin(_) => (),
                    _ => continue,
                }

                let attack = |cur_x: usize,
                              cur_y: usize,
                              state: &mut [Vec<Cell>],
                              grid: &mut Grid|
                 -> bool {
                    let mut best_adjascent_enemy = None;
                    for (x_diff, y_diff) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let (xa, ya) = (cur_x as isize + x_diff, cur_y as isize + y_diff);
                        let is_enemy = xa >= 0
                            && ya >= 0
                            && xa < (GRID_SIZE as isize)
                            && ya < (GRID_SIZE as isize)
                            && state[ya as usize][xa as usize].is_enemy(cell);
                        if !is_enemy {
                            continue;
                        }
                        let (dst_x, dst_y) = (xa as usize, ya as usize);

                        let hp = match state[dst_y][dst_x] {
                            Cell::Elf(hp) | Cell::Goblin(hp) => hp,
                            _ => unreachable!(),
                        };
                        best_adjascent_enemy = match best_adjascent_enemy {
                            None => Some((hp, (dst_x, dst_y))),
                            Some((best_hp, (best_x, best_y)))
                                if (best_hp > hp)
                                    || (hp == best_hp && (dst_y, dst_x) < (best_y, best_x)) =>
                                Some((hp, (dst_x, dst_y))),
                            _ => best_adjascent_enemy,
                        }
                    }

                    if let Some((_, (target_x, target_y))) = best_adjascent_enemy {
                        // attack after moving
                        let died = state[target_y][target_x]
                            .damage(cell.get_attack_power(cur_elf_attack_power));
                        if died {
                            state[target_y][target_x] = Cell::Blank;
                            grid.add_vertex((target_x, target_y));
                        };
                        true
                    } else {
                        false
                    }
                };

                // check if there is an enemy adjascent
                let did_attack = attack(x, y, &mut state, &mut grid);
                if did_attack {
                    continue;
                }

                // move towards closest targets, stored as (x,y)
                let mut possible_targets: Vec<(usize, usize)> = Vec::new();
                for y2 in 0..GRID_SIZE {
                    for x2 in 0..GRID_SIZE {
                        // find all enemies on the grid
                        let target = state[y2][x2];
                        if !cell.is_enemy(target) {
                            continue;
                        }

                        // valid targets are cells which are adjascent to an enemy and traversable
                        for (x_diff, y_diff) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let (xa, ya) = (x2 as isize + x_diff, y2 as isize + y_diff);
                            let is_invalid = xa < 0
                                || ya < 0
                                || xa >= (GRID_SIZE as isize)
                                || ya >= (GRID_SIZE as isize)
                                || !state[ya as usize][xa as usize].is_traversable();
                            if is_invalid {
                                continue;
                            }

                            possible_targets.push((xa as usize, ya as usize));
                        }
                    }
                }

                if possible_targets.is_empty() {
                    if state
                        .iter()
                        .flat_map(|l| l.iter())
                        .find(|c| c.is_enemy(cell))
                        .is_some()
                    {
                        // there are remaining targets, just none we can attack.
                        continue;
                    }

                    // no more targets to kill; one side has won
                    break 'main;
                }

                possible_targets.sort_unstable_by(|&(x1, y1), &(x2, y2)| {
                    let dst1 = manhattan_distance(x, y, x1, y1);
                    let dst2 = manhattan_distance(x, y, x2, y2);
                    dst1.cmp(&dst2)
                });

                let mut solutions = Vec::new();
                let mut min_solution_len = usize::max_value();
                let (mut min_target_x, mut min_target_y) = (usize::max_value(), usize::max_value());
                grid.add_vertex((x, y));
                for (target_x, target_y) in possible_targets {
                    if let Some((solution, _)) = fringe(
                        &(x, y),
                        |n| grid.neighbours(n).into_iter().map(|n| (n, 1)),
                        |(xt, yt)| manhattan_distance(*xt, *yt, target_x, target_y),
                        |n| *n == (target_x, target_y),
                    ) {
                        let cur_solution_len = solution.len();
                        if cur_solution_len < min_solution_len
                            || ((cur_solution_len == min_solution_len)
                                && ((target_y, target_x) < (min_target_y, min_target_x)))
                        {
                            min_solution_len = cur_solution_len;
                            min_target_x = target_x;
                            min_target_y = target_y;
                        } else {
                            // skip this target completely if it isn't the closest
                            continue;
                        }

                        // We've found a shortest solution, now check to see if there are multiple
                        // first steps that yield optimal paths and pick the first step with the
                        // first step which comes first in reading order
                        let (mut min_next_step_x, mut min_next_step_y) =
                            (usize::max_value(), usize::max_value());

                        for (x_diff, y_diff) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let (xa, ya) = (x as isize + x_diff, y as isize + y_diff);
                            let is_valid_first_step = xa >= 0
                                && ya >= 0
                                && xa < (GRID_SIZE as isize)
                                && ya < (GRID_SIZE as isize)
                                && state[ya as usize][xa as usize].is_traversable();
                            if !is_valid_first_step {
                                continue;
                            }

                            let subsolution = fringe(
                                &(xa as usize, ya as usize),
                                |n| grid.neighbours(n).into_iter().map(|n| (n, 1)),
                                |(xt, yt)| manhattan_distance(*xt, *yt, target_x, target_y),
                                |n| *n == (target_x, target_y),
                            );
                            if let Some((solution, _)) = subsolution {
                                if solution.len() >= min_solution_len {
                                    continue;
                                }

                                if (ya as usize, xa as usize) < (min_next_step_y, min_next_step_x) {
                                    min_next_step_x = xa as usize;
                                    min_next_step_y = ya as usize;
                                }
                            }
                        }

                        solutions.push((
                            cur_solution_len,
                            (target_y, target_x),
                            (min_next_step_y, min_next_step_x),
                        ));
                    }
                }
                grid.remove_vertex(&(x, y));

                let best_dst_opt = solutions.into_iter().min().map(|(_, _, (y, x))| (x, y));
                if let Some((dst_x, dst_y)) = best_dst_opt {
                    // move along the path towards the destination
                    assert_eq!(state[dst_y][dst_x], Cell::Blank);
                    grid.add_vertex((x, y));
                    grid.remove_vertex(&(dst_x, dst_y));
                    state[dst_y][dst_x] = cell;
                    state[y][x] = Cell::Blank;
                    moved_entities.push((dst_x, dst_y));

                    // check if there is an enemy adjascent and attack if there is
                    attack(dst_x, dst_y, &mut state, &mut grid);
                }
            }
        }

        rounds += 1;
    }

    if elves_must_win {
        let after_elf_count = state
            .iter()
            .flat_map(|l| l.into_iter())
            .filter(|c| c.is_enemy(Cell::Goblin(100)))
            .count();
        if after_elf_count != original_elf_count {
            return None;
        }
    }

    let hitpoint_sum: usize = state
        .into_iter()
        .flat_map(|l| l.into_iter())
        .map(|c| match c {
            Cell::Elf(hp) | Cell::Goblin(hp) => hp,
            _ => 0,
        })
        .sum();

    Some(rounds * hitpoint_sum)
}

fn part1() -> usize { solve(3, false, false, false).unwrap() }

fn part2() -> usize {
    let mut cur_elf_attack_power = 3;
    loop {
        cur_elf_attack_power += 1;
        if let Some(res) = solve(cur_elf_attack_power, true, false, false) {
            return res;
        }
    }
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
