use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day16.txt");

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new(".*?(\\d+).+?(\\d+).+?(\\d+).+?(\\d+).*").unwrap();
}

fn parse_line(line: &str) -> [usize; 4] {
    let mut res = [0usize; 4];
    let captures = RGX
        .captures(line)
        .expect(&format!("regex captures failed for {:?}", line));
    res[0] = captures[1].parse().unwrap();
    res[1] = captures[2].parse().unwrap();
    res[2] = captures[3].parse().unwrap();
    res[3] = captures[4].parse().unwrap();

    res
}

fn parse_input() -> (
    impl Iterator<Item = ([usize; 4], [usize; 4], [usize; 4])>,
    impl Iterator<Item = [usize; 4]>,
) {
    let lines = INPUT.lines().collect::<Vec<_>>();

    let observed_executions = lines
        .chunks(4)
        .take_while(|chunk| !chunk[0].is_empty())
        .map(move |block| {
            (
                parse_line(block[0]),
                parse_line(block[1]),
                parse_line(block[2]),
            )
        })
        .collect::<Vec<_>>();

    let instructions = lines
        .into_iter()
        .skip(observed_executions.len() * 4 + 2)
        .take_while(|l| !l.is_empty())
        .map(parse_line);

    (observed_executions.into_iter(), instructions)
}

fn btou(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

fn exec(opcode: &str, in1: usize, in2: usize, out: usize, reg: &mut [usize; 4]) {
    reg[out] = match opcode {
        "addr" => reg[in1] + reg[in2],
        "addi" => reg[in1] + in2,
        "mulr" => reg[in1] * reg[in2],
        "muli" => reg[in1] * in2,
        "barr" => reg[in1] & reg[in2],
        "bari" => reg[in1] & in2,
        "borr" => reg[in1] | reg[in2],
        "bori" => reg[in1] | in2,
        "setr" => reg[in1],
        "seti" => in1,
        "gtir" => btou(in1 > reg[in2]),
        "gtri" => btou(reg[in1] > in2),
        "gtrr" => btou(reg[in1] > reg[in2]),
        "eqir" => btou(in1 == reg[in2]),
        "eqri" => btou(reg[in1] == in2),
        "eqrr" => btou(reg[in1] == reg[in2]),
        _ => panic!("Invalid opcode: {}", opcode),
    }
}

const ALL_OPCODES: &[&str] = &[
    "addr", "addi", "mulr", "muli", "barr", "bari", "borr", "bori", "setr", "seti", "gtir", "gtri",
    "gtrr", "eqir", "eqri", "eqrr",
];

fn part1() -> usize {
    let mut three_or_more_valid = 0;
    for (before, op, after) in parse_input().0 {
        let mut count = 0;
        for opcode in ALL_OPCODES {
            let mut reg = before;
            exec(opcode, op[1], op[2], op[3], &mut reg);
            if reg == after {
                count += 1;
            }
        }

        if count >= 3 {
            three_or_more_valid += 1;
        }
    }

    three_or_more_valid
}

fn part2() -> usize {
    let (observed_executions, instructions) = parse_input();

    let mut reg: [usize; 4] = [0; 4];
    let mut valid_opcodes: Vec<Vec<HashSet<&str>>> = vec![Vec::new(); 16];

    for (before, op, after) in observed_executions {
        let mut possible_opcodes = HashSet::new();
        for opcode in ALL_OPCODES {
            reg = before;
            exec(opcode, op[1], op[2], op[3], &mut reg);
            if after == reg {
                possible_opcodes.insert(opcode.clone());
            }
        }

        valid_opcodes[op[0]].push(possible_opcodes);
    }

    let mut mappings: [&str; 16] = [""; 16];
    let mut found_mappings = 0;

    loop {
        for i in 0..16 {
            if mappings[i] != "" {
                continue;
            }

            let mut valid_for_all = HashSet::new();
            for opcode in &valid_opcodes[i][0] {
                valid_for_all.insert(opcode);
            }

            for matched_opcode_list in &valid_opcodes[i][1..] {
                valid_for_all.retain(|opcode| matched_opcode_list.iter().any(|o| *o == **opcode));
            }

            for opcode in &mappings {
                valid_for_all.remove(opcode);
            }

            if valid_for_all.len() == 1 {
                mappings[i] = valid_for_all.drain().next().unwrap();
                found_mappings += 1;
            }
        }

        if found_mappings == 16 {
            break;
        }
    }

    for [op, a, b, c] in instructions {
        exec(mappings[op], a, b, c, &mut reg);
    }

    reg[0]
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
