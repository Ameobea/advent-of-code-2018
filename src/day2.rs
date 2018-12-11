extern crate libc;
extern crate packed_simd;
extern crate test;

use std::{collections::HashMap, hint::unreachable_unchecked, intrinsics::likely, mem, slice};

use packed_simd::{m8, m8x32, u8x32};

const INPUT: &str = include_str!("../input/day2.txt");
// We include 5 bytes of *undefined* memory in order to have the thing padded to 27-byte windows,
// which is necessary for the way that we read them into SIMD registers later.
const INPUT_BYTES: [u8; 6750] = *include_bytes!("../input/day2.txt");
const LINE_COUNT: usize = 250;
const LINE_LENGTH_WITH_NEWLINE: usize = 27;

fn parse_input() -> impl Iterator<Item = &'static [u8]> {
    INPUT.lines().map(|line| line.as_bytes())
}

fn part1() -> usize {
    let mut seen_twice = 0usize;
    let mut seen_three_times = 0usize;
    let mut seen_letters: HashMap<u8, usize> = HashMap::new();

    for line in parse_input() {
        for c in line {
            seen_letters.entry(*c).and_modify(|i| *i += 1).or_insert(1);
        }

        let mut has_two = false;
        let mut has_three = false;
        for (_, n) in seen_letters.drain() {
            if n == 2 && !has_two {
                seen_twice += 1;
                has_two = true
            } else if n == 3 && !has_three {
                seen_three_times += 1;
                has_three = true;
            }
        }
    }

    seen_twice * seen_three_times
}

#[inline(always)]
fn get_shift_mask() -> m8x32 {
    [
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(false),
        m8::new(false),
        m8::new(false),
        m8::new(false),
        m8::new(false),
    ]
    .into()
}

#[inline(never)]
fn part2() {
    let shift_mask: m8x32 = get_shift_mask();

    let mut parsed_input: [u8x32; LINE_COUNT] = unsafe { mem::uninitialized() };
    for (i, slot) in parsed_input.iter_mut().enumerate() {
        let input_offset = i * LINE_LENGTH_WITH_NEWLINE;
        let p = slot as *mut packed_simd::Simd<[u8; 32]>;

        let line_slice =
            unsafe { slice::from_raw_parts((&INPUT_BYTES as *const u8).add(input_offset), 32) };
        let packed_line = unsafe { u8x32::from_slice_unaligned_unchecked(line_slice) };
        // replace the last 5 bits with zeroes
        let packed_shifted_line = shift_mask.select(packed_line, u8x32::splat(0));

        unsafe { p.write(packed_shifted_line) };
    }

    let zeroes = u8x32::splat(0u8);
    let ones = u8x32::splat(1u8);

    for (i, line) in parsed_input.iter().enumerate() {
        for line2 in parsed_input[i + 1..].iter() {
            let equality_mask: m8x32 = (*line).eq(*line2);
            let equality_bytes: u8x32 = equality_mask.select(zeroes, ones);
            // Elements that are equal are now `0usize` and ones that aren't are `1usize`
            let differing_chars = equality_bytes.wrapping_sum();
            // println!("{:?}\n{:?}\n  {:?}", line, line2, equality_bytes);

            if differing_chars == 1 {
                let line1_chars: [u8; 32] = (*line).into();
                let line2_chars: [u8; 32] = (*line2).into();

                for (i, c) in line1_chars[0..27].iter().enumerate() {
                    if unsafe { likely(*c == line2_chars[i]) } {
                        #[allow(clippy::cast_lossless)]
                        unsafe { libc::putchar(*c as libc::c_int) };
                    }
                }
                return;
            }
        }
    }

    unsafe { unreachable_unchecked() }
}

#[bench]
fn bench_part2(b: &mut test::Bencher) { b.iter(part2) }

pub fn run() {
    println!("Part 1: {}", part1());

    print!("Part 2:");
    print!("\n");
    part2();
}
