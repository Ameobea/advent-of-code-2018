#![feature(box_syntax, core_intrinsics, const_raw_ptr_deref, nll, stdsimd, test)]
#![allow(clippy::needless_range_loop, clippy::type_complexity)]

extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate slab;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
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
];

pub fn main() {
    for (i, day) in DAYS.iter().enumerate() {
        print_day(i + 1);
        day();
    }
}
