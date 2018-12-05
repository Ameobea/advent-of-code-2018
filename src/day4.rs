extern crate chrono;

use std::cmp::Ordering;

use regex::Regex;

lazy_static! {
    static ref RGX: Regex =
        Regex::new("\\[(\\d+)-(\\d+)-(\\d+) (\\d\\d):(\\d\\d)\\] (.*)").unwrap();
    static ref GUARD_RGX: Regex = Regex::new("Guard #(\\d+) begins shift").unwrap();
}

const INPUT: &str = include_str!("../input/day4.txt");

#[derive(Debug, PartialEq, Eq)]
enum Action {
    StartShift(usize),
    FallAsleep,
    WakeUp,
}

impl Action {
    pub fn guard_id(&self) -> Option<usize> {
        match self {
            Action::StartShift(guard_id) => Some(*guard_id),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Event {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub action: Action,
}

impl Event {
    pub fn timestamp(&self) -> i64 {
        let dt = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd(self.year as i32, self.month as u32, self.day as u32),
            chrono::NaiveTime::from_hms(self.hour as u32, self.minute as u32, 0),
        );
        dt.timestamp()
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering { self.timestamp().cmp(&other.timestamp()) }
}

fn parse_action(action: &str) -> Action {
    if action == "falls asleep" {
        Action::FallAsleep
    } else if action == "wakes up" {
        Action::WakeUp
    } else {
        let guard_id: usize = GUARD_RGX.captures(action).unwrap()[1].parse().unwrap();
        Action::StartShift(guard_id)
    }
}

fn parse_input() -> impl Iterator<Item = Event> {
    let mut events: Vec<Event> = RGX
        .captures_iter(INPUT)
        .map(|cap| Event {
            year: cap[1].parse().unwrap(),
            month: cap[2].parse().unwrap(),
            day: cap[3].parse().unwrap(),
            hour: cap[4].parse().unwrap(),
            minute: cap[5].parse().unwrap(),
            action: parse_action(&cap[6]),
        })
        .collect();

    events.sort_unstable();
    events.into_iter()
}

fn get_guard_sleep_totals() -> Vec<[usize; 60]> {
    let mut guard_sleep_totals: Vec<[usize; 60]> = vec![[0; 60]; 4000];

    let mut input_iter = parse_input();
    let first_evt = input_iter.next().unwrap();
    let first_guard_id = first_evt.action.guard_id().unwrap();
    input_iter.fold(
        (first_guard_id, first_evt.timestamp(), 0),
        |(cur_guard_id, last_ts, sleep_start_minute), evt| match evt.action {
            Action::FallAsleep => (cur_guard_id, evt.timestamp(), evt.minute),
            Action::StartShift(id) => (id, last_ts, sleep_start_minute),
            Action::WakeUp => {
                let seconds_asleep = evt.timestamp() - last_ts;
                let minutes_asleep = seconds_asleep / 60;

                for i in sleep_start_minute..(sleep_start_minute + minutes_asleep as usize) {
                    guard_sleep_totals[cur_guard_id][i % 60] += 1;
                }

                (cur_guard_id, last_ts, 61)
            },
        },
    );

    guard_sleep_totals
}

fn find_max_ix_by<T, B: PartialEq + Ord>(
    by: fn(T) -> B,
    initial: B,
    items: impl Iterator<Item = T>,
) -> (usize, B) {
    items
        .enumerate()
        .fold((0, initial), |(max_ix, max_val), (i, val)| {
            let transformed: B = by(val);
            if transformed > max_val {
                (i, transformed)
            } else {
                (max_ix, max_val)
            }
        })
}

fn part1() -> usize {
    let guard_sleep_totals: Vec<[usize; 60]> = get_guard_sleep_totals();

    let (best_guard_id, _): (usize, usize) = find_max_ix_by(
        |sleep_times| sleep_times.iter().sum(),
        0,
        guard_sleep_totals.iter(),
    );
    let sleep_times = &guard_sleep_totals[best_guard_id];

    let (best_minute, _) = find_max_ix_by(|time| *time, 0, sleep_times.iter());

    best_guard_id * best_minute
}

pub fn part2() -> impl ::std::fmt::Display {
    let guard_sleep_totals: Vec<[usize; 60]> = get_guard_sleep_totals();

    let (most_slept_minute_guard_id, (_, most_slept_minute)) = find_max_ix_by(
        |sleep_times| -> (usize, usize) {
            let (cur_most_slept_minute, cur_most_slept_minutes) =
                find_max_ix_by(|m| *m, 0, sleep_times.iter());

            (cur_most_slept_minutes, cur_most_slept_minute)
        },
        (0, 0),
        guard_sleep_totals.into_iter(),
    );

    most_slept_minute_guard_id * most_slept_minute
}

#[cfg(test)]
mod test {
    extern crate test;

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) { b.iter(super::part2) }
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
