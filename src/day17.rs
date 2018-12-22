use std::usize;

use regex::Regex;

const INPUT: &str = include_str!("../input/day17.txt");

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
    INPUT.lines().filter(|l| !l.is_empty()).map(parse_line)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Sand,
    Clay,
    Water,
}

#[allow(dead_code)]
fn debug_world(world: &[Vec<Cell>]) {
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
    if (y + 1) >= world.len() {
        return false;
    }

    if world[y + 1][x] == Cell::Water {
        match spread_left(world, x, y + 1, true) {
            SpreadResult::Downspout(x) => {
                world[y + 1][x] = Cell::Sand;
                return false;
            },
            SpreadResult::WaterCollision => return false,
            SpreadResult::WallCollision => (),
        };
        match spread_right(world, x, y + 1, true) {
            SpreadResult::Downspout(x) => {
                world[y + 1][x] = Cell::Sand;
                return false;
            },
            SpreadResult::WaterCollision => return false,
            SpreadResult::WallCollision => (),
        };
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
        match pour_location_right {
            SpreadResult::Downspout(x) => spread = spread_down(world, x, y) && spread,
            SpreadResult::WaterCollision => spread = false,
            _ => (),
        };
        match pour_location_left {
            SpreadResult::Downspout(x) => spread = spread_down(world, x, y) && spread,
            SpreadResult::WaterCollision => spread = false,
            _ => (),
        };

        spread
    } else {
        false
    }
}

enum SpreadResult {
    Downspout(usize),
    WaterCollision,
    WallCollision,
}

/// Returns the x coordinate of the cell at which water will begin to pour downward, unless it hits
/// a wall or the side of amap in which `None` will be returned.
fn spread_left(world: &mut Vec<Vec<Cell>>, x: usize, y: usize, ignore_water: bool) -> SpreadResult {
    if x == 0 || world[y][x - 1] == Cell::Clay {
        return SpreadResult::WallCollision;
    } else if y + 1 < world.len() && world[y + 1][x] == Cell::Sand {
        return SpreadResult::Downspout(x);
    } else if !ignore_water && world[y][x - 1] == Cell::Water {
        return SpreadResult::WaterCollision;
    }

    world[y][x - 1] = Cell::Water;

    spread_left(world, x - 1, y, ignore_water)
}

/// Returns the x coordinate of the cell at which water will begin to pour downward, unless it hits
/// a wall or the side of amap in which `None` will be returned.
fn spread_right(
    world: &mut Vec<Vec<Cell>>,
    x: usize,
    y: usize,
    ignore_water: bool,
) -> SpreadResult {
    if x + 1 == world[0].len() || world[y][x + 1] == Cell::Clay {
        return SpreadResult::WallCollision;
    } else if y + 1 < world.len() && world[y + 1][x] == Cell::Sand {
        return SpreadResult::Downspout(x);
    } else if !ignore_water && world[y][x + 1] == Cell::Water {
        return SpreadResult::WaterCollision;
    }

    world[y][x + 1] = Cell::Water;

    spread_right(world, x + 1, y, ignore_water)
}

fn compute_world() -> (usize, usize, Vec<Vec<Cell>>) {
    let (max_x, max_y, min_y) = parse_input().fold(
        (0, 0, usize::max_value()),
        |(max_x, max_y, min_y), (x, y)| {
            let cur_max_x = (x.0).max((x.1).unwrap_or(0));
            let cur_max_y = (y.0).max((y.1).unwrap_or(0));
            (max_x.max(cur_max_x), max_y.max(cur_max_y), min_y.min(y.0))
        },
    );

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

    (min_y, max_y, world)
}

fn count_water(world: &[Vec<Cell>], min_y: usize, max_y: usize) -> usize {
    world[min_y..=max_y]
        .iter()
        .flat_map(|l| l.iter())
        .filter(|&&c| c == Cell::Water)
        .count()
}

fn part1() -> usize {
    let (min_y, max_y, world) = compute_world();
    count_water(&world, min_y, max_y)
}

fn part2() -> usize {
    let (min_y, max_y, mut world) = compute_world();

    let mut count = count_water(&world, min_y, max_y);
    let row_len = world[0].len();
    loop {
        for x in 0..(row_len) {
            for y in (1..world.len()).rev() {
                if world[y][x] == Cell::Sand && world[y - 1][x] == Cell::Water {
                    world[y - 1][x] = Cell::Sand;
                }
            }
        }

        for row in &mut world {
            for i in 0..(row.len() - 1) {
                if row[i] == Cell::Sand && row[i + 1] == Cell::Water {
                    row[i + 1] = Cell::Sand;
                } else if row[i + 1] == Cell::Sand && row[i] == Cell::Water {
                    row[i] = Cell::Sand;
                }

                let i = (row.len() - i) - 1;
                if row[i] == Cell::Sand && row[i - 1] == Cell::Water {
                    row[i - 1] = Cell::Sand;
                } else if row[i - 1] == Cell::Sand && row[i] == Cell::Water {
                    row[i] = Cell::Sand;
                }
            }
        }

        let new_count = count_water(&world, min_y, max_y);
        if new_count == count {
            // debug_world(&world);
            return new_count;
        } else {
            count = new_count;
        }
    }
}

pub fn run() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}
