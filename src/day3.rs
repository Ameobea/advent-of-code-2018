use std::collections::HashSet;

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new("#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)").unwrap();
}

const INPUT: &str = include_str!("../input/day3.txt");

#[derive(Default, Debug)]
struct Claim {
    pub id: usize,
    pub from_left: usize,
    pub from_top: usize,
    pub width: usize,
    pub height: usize,
}

fn parse_input() -> impl Iterator<Item = Claim> {
    RGX.captures_iter(INPUT).map(|cap| Claim {
        id: cap[1].parse().unwrap(),
        from_left: cap[2].parse().unwrap(),
        from_top: cap[3].parse().unwrap(),
        width: cap[4].parse().unwrap(),
        height: cap[5].parse().unwrap(),
    })
}

#[derive(Clone, Default)]
struct FabricSquare(HashSet<usize>);

fn build_fabric() -> Vec<Vec<FabricSquare>> {
    let mut fabric: Vec<Vec<FabricSquare>> = vec![vec![FabricSquare::default(); 1000]; 1000];

    for claim in parse_input() {
        for row in &mut fabric[claim.from_top..(claim.from_top + claim.height)] {
            for square in &mut row[claim.from_left..(claim.from_left + claim.width)] {
                square.0.insert(claim.id);
            }
        }
    }

    fabric
}

fn part1() -> usize {
    build_fabric()
        .into_iter()
        .flat_map(|v| v.into_iter())
        .fold(0, |acc, v| if v.0.len() >= 2 { acc + 1 } else { acc })
}

fn part2() -> usize {
    let mut invalid_claims: HashSet<usize> = HashSet::with_capacity(1300);

    build_fabric()
        .into_iter()
        .flat_map(|v| v.into_iter())
        .for_each(|mut square| {
            if square.0.len() > 1 {
                for claim_id in square.0.drain() {
                    invalid_claims.insert(claim_id);
                }
            }
        });

    for i in 1..=1295 {
        if invalid_claims.get(&i).is_none() {
            return i;
        }
    }

    unreachable!();
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
