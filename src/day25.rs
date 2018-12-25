use std::collections::VecDeque;

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new("\\w*(-?\\d+),(-?\\d+),(-?\\d+),(-?\\d+)").unwrap();
}

const INPUT: &str = include_str!("../input/day25.txt");

fn manhattan_distance(
    (x1, y1, z1, t1): (isize, isize, isize, isize),
    (x2, y2, z2, t2): (isize, isize, isize, isize),
) -> isize {
    let x_diff = (x1 - x2).abs();
    let y_diff = (y1 - y2).abs();
    let z_diff = (z1 - z2).abs();
    let t_diff = (t1 - t2).abs();
    x_diff + y_diff + z_diff + t_diff
}

fn parse_input() -> impl Iterator<Item = (isize, isize, isize, isize)> {
    INPUT.lines().filter(|l| !l.is_empty()).map(|l| {
        let caps = RGX.captures(l).unwrap();
        (
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
        )
    })
}

fn part1() -> usize {
    let mut constellations: VecDeque<Vec<(isize, isize, isize, isize)>> = VecDeque::new();
    for new_star in parse_input() {
        let mut match_found = false;
        for constellation in &mut constellations {
            let mut connects = false;
            for &star in constellation.iter() {
                if manhattan_distance(star, new_star) <= 3 {
                    connects = true;
                    break;
                }
            }

            if connects {
                constellation.push(new_star);
                match_found = true;
                break;
            }
        }
        if !match_found {
            constellations.push_back(vec![new_star]);
        }
    }

    loop {
        let mut cur_has_no_match = false;
        let mut since_last_match = 0;
        while let Some(constellation) = constellations.pop_front() {
            let mut was_merged = false;
            for candidate_constellation in &mut constellations {
                let mut constellations_should_be_merged = false;
                for &star in &constellation {
                    constellations_should_be_merged =
                        candidate_constellation.iter().any(|&candidate_star| {
                            let dst = manhattan_distance(candidate_star, star) <= 3;
                            // println!("{:?} - {:?}: {}", star, candidate_star, dst);
                            dst
                        });
                    if constellations_should_be_merged {
                        break;
                    }
                }

                if constellations_should_be_merged {
                    candidate_constellation.extend(constellation.iter().cloned());
                    was_merged = true;
                    break;
                }
            }

            if !was_merged {
                if !cur_has_no_match {
                    cur_has_no_match = true;
                    since_last_match = 0;
                } else {
                    since_last_match += 1;
                }
                constellations.push_back(constellation);

                if since_last_match == constellations.len() {
                    // println!("{:?}", constellations);
                    return constellations.len();
                }
            } else {
                cur_has_no_match = false;
            }
        }
    }
    // 566: too high
    // 352: too low
}

fn part2() -> usize { 0 }

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
