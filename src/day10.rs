use std::isize;

use regex::Regex;

lazy_static! {
    static ref RGX: Regex =
        Regex::new("position=< *(-?\\d+), +(-?\\d+)> velocity=< *(-?\\d+), +(-?\\d+)>").unwrap();
}

const INPUT: &str = include_str!("../input/day10.txt");

struct Line {
    pub pos_x: isize,
    pub pos_y: isize,
    pub velocity_x: isize,
    pub velocity_y: isize,
}

fn parse_input() -> impl Iterator<Item = Line> {
    RGX.captures_iter(INPUT).map(|cap| Line {
        pos_x: cap[1].parse().unwrap(),
        pos_y: cap[2].parse().unwrap(),
        velocity_x: cap[3].parse().unwrap(),
        velocity_y: cap[4].parse().unwrap(),
    })
}

fn solve() -> (String, usize) {
    let mut points = parse_input().collect::<Vec<_>>();

    let mut min_y_range = isize::max_value();
    let mut i = 0;
    loop {
        let mut min_x = isize::max_value();
        let mut max_x = isize::min_value();
        let mut min_y = isize::max_value();
        let mut max_y = isize::min_value();

        for line in &mut points {
            line.pos_x += line.velocity_x;
            line.pos_y += line.velocity_y;
            min_x = min_x.min(line.pos_x);
            max_x = max_x.max(line.pos_x);
            min_y = min_y.min(line.pos_y);
            max_y = max_y.max(line.pos_y);
        }

        let x_range = max_x - min_x;
        let y_range = max_y - min_y;
        if min_y_range <= y_range {
            let mut grid = vec![vec![' '; x_range as usize]; y_range as usize];
            for line in &points {
                grid[(line.pos_y - line.velocity_y - min_y) as usize][(line.pos_x - line.velocity_x - min_x) as usize] = '#';
            }

            let message = grid
                .into_iter()
                .map(|line| line.into_iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n");
            return (message, i);
        }

        i += 1;
        min_y_range = y_range;
    }
}

pub fn run() {
    let (part1, part2) = solve();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
