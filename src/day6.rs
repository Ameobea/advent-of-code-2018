use std::{collections::HashSet, usize};

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new("(\\d+), (\\d+)").unwrap();
}

const INPUT: &str = include_str!("../input/day6.txt");

fn parse_input() -> impl Iterator<Item = (usize, usize)> {
    RGX.captures_iter(INPUT)
        .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
}

#[derive(Clone, Copy, Debug)]
enum GridPoint {
    Origin(usize),
    Closest(Option<usize>),
}

fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let x_diff = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let y_diff = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    x_diff + y_diff
}

fn part1() -> usize {
    let input: Vec<(usize, usize)> = parse_input().collect();
    let (max_x, max_y) = input.iter().fold(
        (usize::min_value(), usize::min_value()),
        |(max_x, max_y), (x, y)| (max_x.max(*x), max_y.max(*y)),
    );

    let find_closest_point = |x: usize, y: usize| -> GridPoint {
        let (mut best_point, mut min_distance, mut multiple_best): (usize, usize, bool) =
            (0, usize::max_value(), false);
        for (i, (px, py)) in input.iter().enumerate() {
            let distance = manhattan_distance(*px, *py, x, y);
            if distance < min_distance {
                best_point = i;
                min_distance = distance;
                multiple_best = false;
            } else if distance == min_distance {
                multiple_best = true;
            }
        }

        if multiple_best {
            GridPoint::Closest(None)
        } else if min_distance == 0 {
            GridPoint::Origin(best_point)
        } else {
            GridPoint::Closest(Some(best_point))
        }
    };

    let mut grid = vec![vec![GridPoint::Origin(0); max_x]; max_y];
    let mut largest_areas = vec![1; input.len()];
    let mut infinite_areas: HashSet<usize> = HashSet::new();
    for y in 0..max_y {
        for x in 0..max_x {
            let res = find_closest_point(x, y);
            grid[y][x] = res;

            if let GridPoint::Closest(Some(i)) = res {
                if x == 0 || y == 0 || x == max_x - 1 || y == max_y - 1 {
                    infinite_areas.insert(i);
                } else {
                    largest_areas[i] += 1;
                }
            }
        }
    }

    largest_areas
        .into_iter()
        .enumerate()
        .filter(|(i, _count)| infinite_areas.get(&i).is_none())
        .map(|(_i, count)| count)
        .max()
        .unwrap()
}

fn part2() -> usize {
    let input: Vec<(usize, usize)> = parse_input().collect();
    let (max_x, max_y) = input.iter().fold(
        (usize::min_value(), usize::min_value()),
        |(max_x, max_y), (x, y)| (max_x.max(*x), max_y.max(*y)),
    );

    let mut count = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            let distance_sum: usize = input
                .iter()
                .map(|(px, py)| manhattan_distance(*px, *py, x, y))
                .sum();
            if distance_sum < 10000 {
                count += 1;
            }
        }
    }

    count
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
