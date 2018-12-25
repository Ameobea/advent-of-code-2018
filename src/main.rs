#![feature(
    box_syntax,
    core_intrinsics,
    const_raw_ptr_deref,
    nll,
    stdsimd,
    test,
    thread_local,
    never_type,
    slice_patterns
)]
#![allow(clippy::needless_range_loop, clippy::type_complexity)]

extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate slab;
extern crate structopt;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate cached;
extern crate z3;

use structopt::StructOpt;

pub mod asm_common;
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn print_day(i: usize) {
    println!("== DAY {} ==", i);
}

const DAYS: &[fn()] = &[
    day1::run,
    day2::run,
    day3::run,
    day4::run,
    day5::run,
    day6::run,
    day7::run,
    day8::run,
    day9::run,
    day10::run,
    day11::run,
    day12::run,
    day13::run,
    day14::run,
    day15::run,
    day16::run,
    day17::run,
    day18::run,
    day19::run,
    day20::run,
    day21::run,
    day22::run,
    day23::run,
    day24::run,
    day25::run,
];

#[derive(StructOpt)]
struct Args {
    #[structopt(short = "d", long = "day")]
    pub days: Vec<usize>,
}

pub fn main() {
    let opt = Args::from_args();
    let days_iterator: Box<Iterator<Item = (usize, &'static fn())>> = if opt.days.is_empty() {
        box DAYS.iter().enumerate()
    } else {
        box opt.days.into_iter().map(|i| (i - 1, &DAYS[i - 1]))
    };

    for (i, day) in days_iterator {
        print_day(i + 1);
        day();
    }
}
