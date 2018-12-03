#![feature(core_intrinsics, const_raw_ptr_deref, stdsimd, test)]

extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod day1;
pub mod day2;
pub mod day3;

fn print_day(i: usize) {
    println!("== DAY {} ==", i);
}

const DAYS: &[fn()] = &[day1::run, day2::run, day3::run];

pub fn main() {
    for (i, day) in DAYS.iter().enumerate() {
        print_day(i);
        day();
    }
}
