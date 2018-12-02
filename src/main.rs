#![feature(core_intrinsics, const_raw_ptr_deref, stdsimd, test)]

pub mod day1;
pub mod day2;

fn print_day(i: usize) {
    println!("== DAY {} ==", i);
}

const DAYS: [fn(); 2] = [day1::run, day2::run];

pub fn main() {
    for (i, day) in DAYS.iter().enumerate() {
        print_day(i);
        day();
    }
}
