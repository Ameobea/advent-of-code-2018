extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::usize;

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new("pos=<(-?\\d+),(-?\\d+),(-?\\d+)>, r=(\\d+)").unwrap();
}

const INPUT: &str = include_str!("../input.txt");

struct Nanobot {
    /// (X,Y,Z)
    pub pos: (isize, isize, isize),
    pub radius: isize,
}

impl Nanobot {
    pub fn in_range_of(&self, pt: (isize, isize, isize)) -> bool {
        let distance = manhattan_distance(self.pos.0, self.pos.1, self.pos.2, pt.0, pt.1, pt.2);
        distance <= self.radius
    }

    /// (min_x, max_x, min_y, max_y, min_z, max_x) that this nanobot's signal radius reaches
    fn max_extents(&self) -> (isize, isize, isize, isize, isize, isize) {
        (
            self.pos.0 - self.radius,
            self.pos.0 + self.radius,
            self.pos.1 - self.radius,
            self.pos.0 + self.radius,
            self.pos.1 - self.radius,
            self.pos.2 + self.radius,
        )
    }
}

fn parse_input() -> impl Iterator<Item = Nanobot> {
    INPUT.lines().filter(|l| !l.is_empty()).map(|line| {
        let caps = RGX.captures(line).unwrap();
        Nanobot {
            pos: (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            ),
            radius: caps[4].parse().unwrap(),
        }
    })
}

fn manhattan_distance(x1: isize, y1: isize, z1: isize, x2: isize, y2: isize, z2: isize) -> isize {
    let x_diff = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let y_diff = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    let z_diff = if z1 < z2 { z2 - z1 } else { z1 - z2 };
    x_diff + y_diff + z_diff
}

fn part1() -> usize {
    let nanobots = parse_input().collect::<Vec<_>>();
    let strongest_nanobot = nanobots
        .iter()
        .max_by_key(|&Nanobot { radius, .. }| radius)
        .unwrap();

    nanobots
        .iter()
        .filter(|bot| {
            let distance_to_strongest = manhattan_distance(
                strongest_nanobot.pos.0,
                strongest_nanobot.pos.1,
                strongest_nanobot.pos.2,
                bot.pos.0,
                bot.pos.1,
                bot.pos.2,
            );
            distance_to_strongest <= strongest_nanobot.radius
        })
        .count()
}

fn part2() -> usize {
    let nanobots = parse_input().collect::<Vec<_>>();
    let (min_x, max_x, min_y, max_y, min_z, max_z) = nanobots.iter().fold(
        (
            isize::max_value(),
            isize::min_value(),
            isize::max_value(),
            isize::min_value(),
            isize::max_value(),
            isize::min_value(),
        ),
        |(min_x, max_x, min_y, max_y, min_z, max_z), bot| {
            let (cur_min_x, cur_max_x, cur_min_y, cur_max_y, cur_min_z, cur_max_z) =
                bot.max_extents();
            (
                min_x.min(cur_min_x),
                max_x.max(cur_max_x),
                min_y.min(cur_min_y),
                max_y.max(cur_max_y),
                min_z.min(cur_min_z),
                max_z.max(cur_max_z),
            )
        },
    );

    let mut step_size = 2_000_000;
    let x_steps = (max_x - min_x) / step_size;
    let y_steps = (max_y - min_y) / step_size;
    let z_steps = (max_z - min_z) / step_size;
    let probe_point_count = z_steps * y_steps * x_steps;

    println!("probe_point_count: {}", probe_point_count);
    // let mut probe_points

    let count_in_range = |pt: (isize, isize, isize)| -> usize {
        nanobots.iter().filter(|bot| bot.in_range_of(pt)).count()
    };

    let (mut best_in_range, mut best_coord, mut best_distance) = (0, (0, 0, 0), 0);
    for z in 0..=z_steps {
        for y in 0..=y_steps {
            for x in 0..=x_steps {
                let pt = (
                    min_x + (x * step_size),
                    min_y + (y * step_size),
                    min_z + (z * step_size),
                );
                let in_range = count_in_range(pt);
                let distance_to_origin = pt.0 + pt.1 + pt.2;

                if in_range > best_in_range
                    || (in_range == best_in_range && distance_to_origin < best_distance)
                {
                    best_in_range = in_range;
                    best_coord = pt;
                    best_distance = distance_to_origin;
                }
            }
        }
    }

    // now, brute-force search
    while step_size > 1 {
        println!("step_size: {}", step_size);

        // 1_000_000_000 iterations
        for z in -500..=500 {
            for y in -500..=500 {
                for x in -500..=500 {
                    let pt = (
                        best_coord.0 + (x * (step_size / 500)),
                        best_coord.1 + (y * (step_size / 500)),
                        best_coord.2 + (z * (step_size / 500)),
                    );
                    let in_range = count_in_range(pt);
                    let distance_to_origin = pt.0 + pt.1 + pt.2;

                    if in_range > best_in_range
                        || (in_range == best_in_range && distance_to_origin < best_distance)
                    {
                        best_in_range = in_range;
                        best_coord = pt;
                        best_distance = distance_to_origin;
                    }
                }
            }
        }
        println!("best_in_range: {}, coord: {:?}", best_in_range, best_coord);
        step_size /= 100;
    }

    0
    // not 125406946
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
