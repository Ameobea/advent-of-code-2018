use std::str::FromStr;

use crate::asm_common::*;

const INPUT: &str = include_str!("../input/day19.txt");

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new("(\\w+) (\\d+) (\\d+) (\\d+)").unwrap();
}

fn parse_input() -> (Registers, impl Iterator<Item = Instruction>) {
    let mut reg = Registers::new(None);

    let mut lines = INPUT.lines();
    let ip_decl_line = lines.next().unwrap();
    reg.ip_register = parse_ip_decl(ip_decl_line);

    let iter = lines.filter_map(|l| {
        if l.is_empty() {
            return None;
        }

        let cap = RGX.captures(l).unwrap();
        let instr = Instruction {
            opcode: Opcode::from_str(&cap[1]).unwrap(),
            in1: cap[2].parse().unwrap(),
            in2: cap[3].parse().unwrap(),
            out: cap[4].parse().unwrap(),
        };
        Some(instr)
    });

    (reg, iter)
}

fn part1() -> usize {
    let (registers, instructions_iter) = parse_input();
    let mut vm = VM {
        regs: registers,
        instructions: Instructions {
            ip: 0usize,
            instructions: instructions_iter.collect(),
        },
    };

    vm.run();
    // println!("Registers after running: {:?}", vm.regs);

    vm.regs.regs[0]
    // not 18
}

fn part2() -> usize {
    let (registers, instructions_iter) = parse_input();
    let mut vm = VM {
        regs: registers,
        instructions: Instructions {
            ip: 0usize,
            instructions: instructions_iter.collect(),
        },
    };
    vm.regs.regs[0] = 1;

    vm.run();
    // println!("Registers after running: {:?}", vm.regs);

    vm.regs.regs[0]
    // not 18
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
