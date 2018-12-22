use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    usize,
};

use cached::UnboundCache;
use pathfinding::prelude::*;
use regex::Regex;

lazy_static! {
    static ref DEPTH_REGEX: Regex = Regex::new("depth: (\\d+)").unwrap();
    static ref TARGET_REGEX: Regex = Regex::new("target: (\\d+),(\\d+)").unwrap();
}

const INPUT: &str = include_str!("../input/day22.txt");

#[derive(Clone, Copy, Debug)]
pub enum Region {
    Rocky,
    Wet,
    Narrow,
}

impl Display for Region {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(
            fmt,
            "{}",
            match self {
                Region::Rocky => '.',
                Region::Wet => '=',
                Region::Narrow => '|',
            }
        )
    }
}

impl Region {
    pub fn valid_tools(self) -> impl Iterator<Item = Option<Tool>> {
        match self {
            Region::Rocky => &[Some(Tool::ClimbingGear), Some(Tool::Torch)],
            Region::Wet => &[Some(Tool::ClimbingGear), None],
            Region::Narrow => &[Some(Tool::Torch), None],
        }
        .iter()
        .cloned()
    }
}

fn parse_input() -> ((isize, isize), isize) {
    let mut lines = INPUT.lines();
    let cap1 = DEPTH_REGEX.captures(lines.next().unwrap()).unwrap();
    let cap2 = TARGET_REGEX.captures(lines.next().unwrap()).unwrap();

    (
        (cap2[1].parse().unwrap(), cap2[2].parse().unwrap()),
        cap1[1].parse().unwrap(),
    )
}

fn get_geologic_index(x: isize, y: isize, target_x: isize, target_y: isize, depth: isize) -> isize {
    if (x, y) == (target_x, target_y) {
        0
    } else if y == 0 {
        x * 16807
    } else if x == 0 {
        y * 48271
    } else {
        erosion_level(x - 1, y, target_x, target_y, depth)
            * erosion_level(x, y - 1, target_x, target_y, depth)
    }
}

cached_key! {
    EROSION: UnboundCache<(isize, isize, isize, isize, isize), isize> = UnboundCache::new();
    Key = { (x, y, target_x, target_y, depth) };

    fn erosion_level(x: isize, y: isize, target_x: isize, target_y: isize, depth: isize) -> isize = {
        (get_geologic_index(x, y, target_x, target_y ,depth) + depth) % 20183
    }
}

cached_key! {
    REGION: UnboundCache<(isize, isize, isize, isize, isize), Region> = UnboundCache::new();
    Key = { (x, y, target_x, target_y, depth) };

    fn get_region_type(x: isize, y: isize, target_x: isize, target_y: isize, depth: isize) -> Region = {
        match erosion_level(x, y, target_x, target_y, depth) % 3 {
            0 => Region::Rocky,
            1 => Region::Wet,
            2 => Region::Narrow,
            _ => unreachable!(),
        }
    }
}

fn get_risk_level(x: isize, y: isize, target_x: isize, target_y: isize, depth: isize) -> usize {
    let mut risk = 0;
    for y in 0..=y {
        for x in 0..=x {
            let region = get_region_type(x, y, target_x, target_y, depth);
            risk += region as usize;
        }
    }

    risk
}

fn part1() -> usize {
    let ((x, y), depth) = parse_input();
    get_risk_level(x, y, x, y, depth)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tool {
    Torch,
    ClimbingGear,
}

fn iter_neighbors<'a>(
    x: isize,
    y: isize,
    target_x: isize,
    target_y: isize,
    cur_tool: Option<Tool>,
    depth: isize,
) -> impl Iterator<Item = ((isize, isize, Option<Tool>), usize)> + 'a {
    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .iter()
        .map(move |(x_diff, y_diff)| (x as isize + x_diff, y as isize + y_diff))
        .filter(move |&(xa, ya)| xa >= 0 && ya >= 0)
        .filter(move |&(xa, ya)| {
            let dst_region = get_region_type(xa, ya, target_x, target_y, depth);
            dst_region.valid_tools().any(|t2| t2 == cur_tool)
        })
        .map(move |(xa, ya)| ((xa, ya, cur_tool), 1))
        .chain(
            get_region_type(x, y, target_x, target_y, depth)
                .valid_tools()
                .filter(move |&t2| t2 != cur_tool)
                .map(move |t2| ((x, y, t2), 7)),
        )
}

fn manhattan_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    let x_diff = (x2 - x1).abs();
    let y_diff = (y2 - y1).abs();
    x_diff + y_diff
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Coord(isize, isize, Option<Tool>);

fn part2() -> usize {
    let ((target_x, target_y), depth) = parse_input();

    astar(
        &(0, 0, Some(Tool::Torch)),
        |&(x, y, tool)| iter_neighbors(x, y, target_x, target_y, tool, depth),
        |&(x, y, _tool)| manhattan_distance(x, y, target_x, target_y) as usize,
        |&(x, y, tool)| x == target_x && y == target_y && tool == Some(Tool::Torch),
    )
    .unwrap()
    .1
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
