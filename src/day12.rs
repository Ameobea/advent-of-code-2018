use std::iter;

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new("initial state: ([#\\.]+)").unwrap();
}

const INPUT: &str = include_str!("../input/day12.txt");

fn parse_input() -> (Vec<bool>, Vec<([bool; 5], bool)>) {
    let mut lines = INPUT.lines();
    let initial_state = &RGX.captures(lines.next().unwrap()).unwrap()[1];
    let parsed_initial_state = initial_state.chars().map(|c| c == '#');
    let padding = iter::repeat(false).take(PADDING_SIZE);
    let padded_parsed_initial_state = padding.clone().chain(parsed_initial_state).chain(padding).collect();

    let rules = lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let rule = l.chars().take(5).map(|c| c == '#').collect();
            let next = l.ends_with('#');
            (rule, next)
        })
        .map(|(pots, next): (Vec<bool>, bool)| {
            ([pots[0], pots[1], pots[2], pots[3], pots[4]], next)
        })
        .collect();

    (padded_parsed_initial_state, rules)
}

const PADDING_SIZE: usize = 250;

fn solve() -> (isize, i64) {
    let (mut state, rules) = parse_input();

    let mut critical_section = Vec::new();
    let mut critical_section_start = 0;
    let mut critical_section_end = 0;
    let mut found = false;
    let mut next_state = |state: Vec<bool>| -> (Vec<bool>, bool) {
        let mut new_state = vec![false; state.len()];
        let mut new_critical_section = Vec::new();
        let mut critical_section_started = false;

        for i in 2..(state.len() - 2) {
            if !found && state[i] {
                if !critical_section_started {
                    critical_section_started = true;
                    critical_section_start = i;
                }
                critical_section_end = i;
            }

            for (rule, is_pot) in &rules {
                if state[(i - 2)..=(i + 2)] == *rule {
                    new_state[i] = *is_pot;
                }
            }
        }

        let is_first_repeat = if !found {
            for val in &state[critical_section_start..=critical_section_end] {
                new_critical_section.push(*val);
            }
            // this is the first iteration where we just translate to the right.
            let is_first_repeat = new_critical_section == critical_section;
            if is_first_repeat {
                found = true;
            } else {
                critical_section = new_critical_section;
            }
            is_first_repeat
        } else {
            false
        };

        (new_state, is_first_repeat)
    };

    fn enumerate_true_state<'a>(state: &'a [bool]) -> impl Iterator<Item = (usize, bool)> + 'a {
        state.iter().cloned().enumerate().filter(|(_i, b)| *b)
    }

    let mut first_repeat_iter = 0;
    let mut total_for_20 = 0;
    for i in 0..200 {
        if i == 20 {
            let bools = enumerate_true_state(&state);
            total_for_20 = bools.map(|(i, _)| i as isize - PADDING_SIZE as isize).sum();
        }

        let (new_state, is_first_repeat) = next_state(state);
        state = new_state;
        if is_first_repeat {
            first_repeat_iter = i;
        }
    }

    let total_for_50_billion: i64 = enumerate_true_state(&critical_section)
        .map(|(i, _)| {
            i as i64 + 50_000_000_000i64 - first_repeat_iter
                + critical_section_start as i64
                - PADDING_SIZE as i64
        })
        .sum();

    (total_for_20, total_for_50_billion)
}

pub fn run() {
    let (part1, part2) = solve();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
