use std::collections::HashSet;

use crate::asm_common::*;

const INPUT: &str = include_str!("../input/day16.txt");

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new(".*?(\\d+).+?(\\d+).+?(\\d+).+?(\\d+).*").unwrap();
}

fn parse_line(line: &str) -> [usize; 6] {
    let mut res = [0usize; 6];
    let captures = RGX
        .captures(line)
        .unwrap_or_else(|| panic!("regex captures failed for {:?}", line));
    res[0] = captures[1].parse().unwrap();
    res[1] = captures[2].parse().unwrap();
    res[2] = captures[3].parse().unwrap();
    res[3] = captures[4].parse().unwrap();

    res
}

fn parse_input() -> (
    impl Iterator<Item = ([usize; 6], [usize; 6], [usize; 6])>,
    impl Iterator<Item = [usize; 6]>,
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

const ALL_OPCODES: &[Opcode] = &[
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Barr,
    Opcode::Bari,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

fn part1() -> usize {
    let mut three_or_more_valid = 0;
    for (before, op, after) in parse_input().0 {
        let mut count = 0;
        for &opcode in ALL_OPCODES {
            let mut reg = Registers {
                ip_register: None,
                regs: before,
            };
            let instr = Instruction {
                opcode,
                in1: op[1],
                in2: op[2],
                out: op[3],
            };
            reg.exec(0, instr);
            if reg.regs == after {
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

    let mut reg = Registers::new(None);
    let mut valid_opcodes: Vec<Vec<HashSet<Opcode>>> = vec![Vec::new(); 16];

    for (before, op, after) in observed_executions {
        let mut possible_opcodes: HashSet<Opcode> = HashSet::new();
        for &opcode in ALL_OPCODES {
            reg.regs = before;
            let instr = Instruction {
                opcode,
                in1: op[1],
                in2: op[2],
                out: op[3],
            };
            reg.exec(0, instr);
            if after == reg.regs {
                #[allow(clippy::clone_double_ref)]
                possible_opcodes.insert(opcode);
            }
        }

        valid_opcodes[op[0]].push(possible_opcodes);
    }

    let mut working_mappings: [Option<Opcode>; 16] = [None; 16];
    let mut found_mappings = 0;

    while found_mappings < 16 {
        for i in 0..16 {
            if working_mappings[i].is_some() {
                continue;
            }

            let mut valid_for_all: HashSet<Opcode> = HashSet::new();
            for &opcode in &valid_opcodes[i][0] {
                valid_for_all.insert(opcode);
            }

            for matched_opcode_list in &valid_opcodes[i][1..] {
                valid_for_all.retain(|opcode| matched_opcode_list.iter().any(|&o| o == *opcode));
            }

            for opcode in working_mappings.iter().filter_map(|&opc_opt| opc_opt) {
                valid_for_all.remove(&opcode);
            }

            if valid_for_all.len() == 1 {
                working_mappings[i] = valid_for_all.drain().next();
                found_mappings += 1;
            }
        }
    }

    let mut mappings: [Opcode; 16] = [Opcode::Addi; 16];
    for (i, mapping) in working_mappings.into_iter().enumerate() {
        mappings[i] = mapping.unwrap();
    }

    for instr in instructions
        .into_iter()
        .map(|[opcode_code, in1, in2, out, ..]| Instruction {
            opcode: mappings[opcode_code],
            in1,
            in2,
            out,
        })
    {
        reg.exec(0, instr);
    }

    reg[0]
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
