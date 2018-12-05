extern crate rayon;

use rayon::{iter::IntoParallelIterator, prelude::*};

const INPUT: &str = include_str!("../input/day5.txt");

fn parse_input() -> impl Iterator<Item = char> { INPUT.chars() }

fn needs_delete(c1: char, c2: char) -> bool {
    c1 != c2 && c1.to_ascii_uppercase() == c2.to_ascii_uppercase()
}

fn react_polymer(polymer: impl Iterator<Item = char>) -> usize {
    let mut acc = String::with_capacity(INPUT.len());
    acc.push('*');
    acc.push('*');

    let reacted_polymer = polymer.fold(acc, |mut acc, c| -> String {
        acc.push(c);
        loop {
            let [second_last_char, last_char] = unsafe {
                [
                    *acc.as_bytes().get_unchecked(acc.len() - 2) as char,
                    *acc.as_bytes().get_unchecked(acc.len() - 1) as char,
                ]
            };

            if needs_delete(second_last_char, last_char) {
                acc.pop();
                acc.pop();
            } else {
                break;
            }
        }
        acc
    });

    reacted_polymer.len() - 3
}

fn part1() -> usize { react_polymer(parse_input()) }

pub fn part2() -> usize {
    // Props to https://github.com/CryZe for coming up with the Rayon idea
    // Can't use an inclusive range here because `IntoParallelIterator` isn't impelemented for it
    // upstream
    #[allow(clippy::range_plus_one)]
    (b'A'..b'Z' + 1)
        .into_par_iter()
        .map(|c| -> usize {
            let c = c as u8 as char;
            react_polymer(parse_input().filter(|c2| c2.to_ascii_uppercase() != c))
        })
        .reduce_with(|a, b| a.min(b))
        .unwrap()
}

#[cfg(test)]
mod test {
    extern crate test;

    #[bench]
    fn bench_p2(b: &mut test::Bencher) { b.iter(super::part2) }
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
