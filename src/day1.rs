use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day1.txt");

fn parse_input() -> impl Iterator<Item = isize> + Clone {
    INPUT
        .split('\n')
        .map(|n| n.parse().map_err(|_| ()))
        .filter(Result::is_ok)
        .map(Result::unwrap)
}

fn part1() -> isize { parse_input().sum() }

fn part2() -> isize {
    let mut seen_frequencies: HashSet<isize> = HashSet::new();
    seen_frequencies.insert(0);

    let mut cur_num = 0;
    for num in parse_input().cycle() {
        cur_num += num;
        if !seen_frequencies.insert(cur_num) {
            return cur_num;
        }
    }

    unreachable!();
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
