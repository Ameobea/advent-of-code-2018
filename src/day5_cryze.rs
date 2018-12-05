use arrayvec::ArrayVec;
use rayon::prelude::*;
use std::hint::unreachable_unchecked;

fn collapse(input: impl IntoIterator<Item = u8>, buf: &mut ArrayVec<[u8; 16384]>) {
    let mut top_byte = b'\0';
    for byte in input {
        if top_byte ^ 0x20 == byte {
            buf.pop();
            top_byte = buf.last().cloned().unwrap_or_default();
        } else {
            top_byte = byte;
            buf.try_push(byte)
                .unwrap_or_else(|_| unsafe { unreachable_unchecked() });
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut buf = ArrayVec::new();
    collapse(input.bytes(), &mut buf);
    buf.len()
}

pub fn part2(input: &str) -> usize {
    let mut buf = ArrayVec::new();
    (b'A'..=b'Z')
        .map(|upper| {
            buf.clear();
            let lower = upper.to_ascii_lowercase();
            let filtered = input.bytes().filter(|&b| b != upper && b != lower);
            collapse(filtered, &mut buf);
            buf.len()
        })
        .min()
        .unwrap()
}

pub fn part2_rayon(input: &str) -> usize {
    (b'A'..b'Z' + 1)
        .into_par_iter()
        .map(|upper| {
            let mut buf = ArrayVec::new();
            let lower = upper.to_ascii_lowercase();
            let filtered = input.bytes().filter(|&b| b != upper && b != lower);
            collapse(filtered, &mut buf);
            buf.len()
        })
        .min()
        .unwrap()
}

extern crate test;
#[bench]
fn bench_p2(b: &mut test::Bencher) { b.iter(|| part2_rayon(include_str!("../input/day5.txt"))) }
