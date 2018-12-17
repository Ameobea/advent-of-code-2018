extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::usize;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

lazy_static! {
    static ref X_RGX: Regex = Regex::new("x=(\\d+)(?:\\.\\.(\\d+))?").unwrap();
    static ref Y_RGX: Regex = Regex::new("y=(\\d+)(?:\\.\\.(\\d+))?").unwrap();
}

fn parse_line(line: &str) -> ((usize, Option<usize>), (usize, Option<usize>)) {
    let x_caps = X_RGX.captures(line).unwrap();
    let x = (
        x_caps[1].parse().unwrap(),
        x_caps.get(2).map(|s| s.as_str().parse().unwrap()),
    );
    let y_caps = Y_RGX.captures(line).unwrap();
    let y = (
        y_caps[1].parse().unwrap(),
        y_caps.get(2).map(|s| s.as_str().parse().unwrap()),
    );

    (x, y)
}

fn parse_input() -> impl Iterator<Item = ((usize, Option<usize>), (usize, Option<usize>))> {
    let parsed_lines = INPUT.lines().filter(|l| !l.is_empty()).map(parse_line);

    parsed_lines
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Sand,
    Clay,
    Water,
}

fn debug_world(world: &[Vec<Cell>]) {
    // return;
    for row in world {
        for c in &row[300..] {
            print!(
                "{}",
                match c {
                    Cell::Clay => '#',
                    Cell::Water => '~',
                    Cell::Sand => '.',
                }
            );
        }
        println!();
    }
    println!("\n");
}

fn spread_down(world: &mut Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    // println!("down: {:?}", (x, y));
    // debug_world(world);
    if (y + 1) >= world.len() {
        return false;
    }

    if world[y + 1][x] == Cell::Water {
        if let Some(x_opt) = spread_left(world, x, y + 1, true) {
            if let Some(x) = x_opt {
                world[y + 1][x] = Cell::Sand;
            }
            return false;
        }
        if let Some(x_opt) = spread_right(world, x, y + 1, true) {
            if let Some(x) = x_opt {
                world[y + 1][x] = Cell::Sand;
            }
            return false;
        }
    }

    let can_move_down = world[y + 1][x] == Cell::Sand;
    let spread = if can_move_down {
        world[y + 1][x] = Cell::Water;
        spread_down(world, x, y + 1)
    } else {
        true
    };

    if spread {
        let pour_location_left = spread_left(world, x, y, world[y][x - 1] == Cell::Water);
        let pour_location_right = spread_right(world, x, y, world[y][x + 1] == Cell::Water);

        let mut spread = true;
        if let Some(Some(x)) = pour_location_right {
            spread = spread_down(world, x, y) && spread;
        } else if let Some(None) = pour_location_right {
            spread = false;
        }

        if let Some(Some(x)) = pour_location_left {
            spread = spread_down(world, x, y) && spread;
        } else if let Some(None) = pour_location_left {
            spread = false;
        }

        spread
    } else {
        false
    }
}

/// Returns the x coordinate of the cell at which water will begin to pour downward, unless it hits a wall or the side of amap in which `None` will be returned.
fn spread_left(
    world: &mut Vec<Vec<Cell>>,
    x: usize,
    y: usize,
    ignore_water: bool,
) -> Option<Option<usize>> {
    // println!("left: {:?}", (x, y));
    // debug_world(world);

    if x == 0 || world[y][x - 1] == Cell::Clay {
        return None;
    }

    if y + 1 < world.len() && world[y + 1][x] == Cell::Sand {
        return Some(Some(x));
    }

    if !ignore_water && world[y][x - 1] == Cell::Water {
        return Some(None);
    }

    world[y][x - 1] = Cell::Water;

    spread_left(world, x - 1, y, ignore_water)
}

/// Returns the x coordinate of the cell at which water will begin to pour downward, unless it hits a wall or the side of amap in which `None` will be returned.
fn spread_right(
    world: &mut Vec<Vec<Cell>>,
    x: usize,
    y: usize,
    ignore_water: bool,
) -> Option<Option<usize>> {
    // println!("left: {:?}", (x, y));
    // debug_world(world);

    if x + 1 == world[0].len() || world[y][x + 1] == Cell::Clay {
        return None;
    }

    if y + 1 < world.len() && world[y + 1][x] == Cell::Sand {
        return Some(Some(x));
    }

    if !ignore_water && world[y][x + 1] == Cell::Water {
        return Some(None);
    }

    world[y][x + 1] = Cell::Water;

    spread_right(world, x + 1, y, ignore_water)
}

fn part1() -> usize {
    let (max_x, max_y, min_y) = parse_input().fold(
        (0, 0, usize::max_value()),
        |(max_x, max_y, min_y), (x, y)| {
            let cur_max_x = (x.0).max((x.1).unwrap_or(0));
            let cur_max_y = (y.0).max((y.1).unwrap_or(0));
            (max_x.max(cur_max_x), max_y.max(cur_max_y), min_y.min(y.0))
        },
    );

    println!("{:?}", (max_x, max_y));

    let mut world = vec![vec![Cell::Sand; max_x + 2]; max_y + 1];
    for (x, y) in parse_input() {
        if let Some(max_x) = x.1 {
            for x in (x.0)..=max_x {
                world[y.0][x] = Cell::Clay;
            }
        } else if let Some(max_y) = y.1 {
            for y in (y.0)..=max_y {
                world[y][x.0] = Cell::Clay;
            }
        }
    }
    world[0][500] = Cell::Water;

    spread_down(&mut world, 500, 0);

    debug_world(&world);

    world[min_y..=max_y]
        .iter()
        .flat_map(|l| l.iter())
        .filter(|&&c| c == Cell::Water)
        .count()

    // not 2172
    // not 132176
    // not 132158
    // not 31902
    // not 31863
    // not 31864
    // not 31860
}

fn part2() -> usize {
    0
}

pub fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}

