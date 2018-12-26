use std::usize;

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new("pos=<(-?\\d+),(-?\\d+),(-?\\d+)>, r=(\\d+)").unwrap();
}

const INPUT: &str = include_str!("../input/day23.txt");

struct Nanobot {
    /// (X,Y,Z)
    pub pos: (isize, isize, isize),
    pub radius: isize,
}

impl Nanobot {
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

fn part2() -> i64 {
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

    let z3_conf = z3::Config::new();
    let ctx = z3::Context::new(&z3_conf);
    let optimizer = z3::Optimize::new(&ctx);

    let x = ctx.named_int_const("x");
    let y = ctx.named_int_const("y");
    let z = ctx.named_int_const("z");
    optimizer.assert(&x.gt(&ctx.from_i64((min_x - 1) as i64)));
    optimizer.assert(&x.lt(&ctx.from_i64((max_x + 1) as i64)));
    optimizer.assert(&y.gt(&ctx.from_i64((min_y - 1) as i64)));
    optimizer.assert(&y.lt(&ctx.from_i64((max_y + 1) as i64)));
    optimizer.assert(&z.gt(&ctx.from_i64((min_z - 1) as i64)));
    optimizer.assert(&z.lt(&ctx.from_i64((max_z + 1) as i64)));

    fn abs<'ctx, 'a>(ctx: &'ctx z3::Context, x: &'a z3::Ast<'ctx>) -> z3::Ast<'ctx> {
        x.lt(&ctx.from_i64(0)).ite(&ctx.from_i64(-1).mul(&[&x]), &x)
    }

    let mut in_range = ctx.from_i64(0);
    for bot in nanobots {
        let bot_x = ctx.from_i64(bot.pos.0 as i64);
        let bot_y = ctx.from_i64(bot.pos.1 as i64);
        let bot_z = ctx.from_i64(bot.pos.2 as i64);
        let bot_radius_plus_1 = ctx.from_i64(bot.radius as i64 + 1);

        let dist_x = abs(&ctx, &bot_x.sub(&[&x]));
        let dist_y = abs(&ctx, &bot_y.sub(&[&y]));
        let dist_z = abs(&ctx, &bot_z.sub(&[&z]));
        let distance_to_bot = dist_x.add(&[&dist_y, &dist_z]);
        let is_in_range_of_bot = distance_to_bot.lt(&bot_radius_plus_1);
        in_range = in_range.add(&[&is_in_range_of_bot.ite(&ctx.from_i64(1), &ctx.from_i64(0))]);
    }
    optimizer.maximize(&in_range);

    let dist_x = abs(&ctx, &x);
    let dist_y = abs(&ctx, &y);
    let dist_z = abs(&ctx, &z);
    let distance_to_origin = dist_x.add(&[&dist_y, &dist_z]);
    optimizer.minimize(&distance_to_origin);

    optimizer.check();
    let model = optimizer.get_model();
    let res = model.eval(&distance_to_origin).unwrap().as_i64().unwrap();
    res
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
