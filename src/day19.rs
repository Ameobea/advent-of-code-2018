use std::str::FromStr;

use crate::asm_common::*;

const INPUT: &str = include_str!("../input/day19.txt");

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new("(?:\\d+:)?(\\w+) (\\d+) (\\d+) (\\d+)").unwrap();
}

fn parse_input() -> (Registers, impl Iterator<Item = Instruction>) {
    let mut reg = Registers::new(None);

    let mut lines = INPUT.lines();
    let ip_decl_line = lines.next().unwrap();
    reg.ip_register = parse_ip_decl(ip_decl_line);

    let iter = lines.filter_map(|l| {
        if l.is_empty() || l.starts_with(';') {
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
}

fn get_all_factors(n: usize) -> impl Iterator<Item = usize> {
    (1..=n).filter(move |factor| n % factor == 0)
}

/// The algorithm works roughly like this:
///   1. Register 4 is populated with some number by doing some random math using register 1
///   2. The sum of all factors of the number in register 4 is accumulated in register [0].  This
///      includes the (1, n) factor pair.
///   3. The program halts, leaving the accumulated sum in register 0
fn part2() -> usize {
    // #ip 1
    // 0: addi 1 16 1; jmp 17
    // 1: seti 1 1 3; [3] = 1
    // ;;; initial regs: [1, 33, 10550400, 0, 10551370, 1]
    // ;;;
    // ;;; [0] = 1;
    // ;;; [5] = 1;
    // ;;; [4] = 10551370;
    // ;;; [3] = 0;
    // ;;;
    // ;;; LBL b:
    // ;;; [5] = 1
    // ;;; LBL a:
    // ;;; if ([3] * [5] == [4]):
    // ;;;    [0] += [3]
    // ;;; [5] += 1
    // ;;; if [5] <= [4]:
    // ;;;    goto a;
    // ;;;
    // ;;; [3] += 1
    // ;;; if [3] <= [4]:
    // ;;;    goto b
    // ;;; HALT
    // ;;;
    // ;;;
    // ;;;
    // 2: seti 1 9 5; [5] = 1
    // 3: mulr 3 5 2; [2] = [3] * [5]
    // 4: eqrr 2 4 2; [2] = [2] == [4]
    // 5: addr 2 1 1; jmp (ip + [2])
    // 6: addi 1 1 1; jmp (ip + 1)
    // 7: addr 3 0 0; [0] += [3]
    // ;;; if ([2] * [3] == [4]), [0] += [3]
    // 8: addi 5 1 5; [5] += 1
    // ;;; if [5] <= [4], jmp 3
    // 9: gtrr 5 4 2; [2] = [5] > [4]
    // 10: addr 1 2 1; if ![2]:
    // 11: seti 2 6 1;    jmp 3
    // 12: addi 3 1 3; [3] += 1
    // ;;; [3] += 3; if [3] <= 4, jmp 2
    // 13: gtrr 3 4 2; [2] = [3] > [4]
    // 14: addr 2 1 1; if ![2]:
    // 15: seti 1 6 1;    jmp 2
    // ;;; while [3] <= [4], [3] += 1
    // 16: mulr 1 1 1; HALT
    // 17: addi 4 2 4; [4] += 2
    // 18: mulr 4 4 4; [4] *= [4]
    // 19: mulr 1 4 4; [4] *= ip
    // 20: muli 4 11 4; [4] *= 11
    // 21: addi 2 6 2; [2] += 6
    // 22: mulr 2 1 2; [2] *= ip
    // 23: addi 2 2 2; [2] += 2
    // 24: addr 4 2 4; [4] += [2]
    // ;;; will halt if [0] >= 10
    // 25: addr 1 0 1; jmp (ip + [0])
    // 26: seti 0 3 1; jmp 1
    // 27: setr 1 4 2; [2] = [1]
    // 28: mulr 2 1 2; [2] *= [1]
    // 29: addr 1 2 2; [2] += [1]
    // 30: mulr 1 2 2; [2] *= [1]
    // 31: muli 2 14 2; [2] *= 2
    // 32: mulr 2 1 2; [2] *= [1]
    // 33: addr 4 2 4; [4] += [4]
    // 34: seti 0 0 0; [0] = 0
    // 35: seti 0 4 1; jmp 1

    let number = 10_551_370;
    get_all_factors(number).sum()
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
