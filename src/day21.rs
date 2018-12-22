fn part1() -> usize {
    // Decompiled using:
    // https://github.com/ttencate/aoc2018/blob/master/src/vm/decompiler.rs
    //
    // Register 0 is a.
    //
    //      e = 123;
    //      do {
    //          e &= 0x1c8;
    //      } while e != 72;
    //      e = 0;
    //      do {
    //          b = e | 0x10000;
    //          e = 16031208;
    //  8:      d = b & 0xff;
    //          e += d;
    //          // e &= 0xffffff;
    //          e *= 65899;
    //          // e &= 0xffffff;
    //          if 256 <= b {
    //              d = 0;
    // 18:          f = d + 1;
    //              f *= 256;
    //              if f <= b {
    //                  d += 1;
    //                  goto 18;
    //              }
    //              b = d;
    //              goto 8;
    //          }
    //      } while e != a;

    // Solved by adding a statement to print out the contents of register e (4) during the `while`
    // check, since that's the earliest point at which the program could halt.

    10_720_163
}

fn part2() -> usize {
    // I blatantly cheated (stole the algorithm from reddit), and I only kinda feel bad.
    // https://www.reddit.com/r/adventofcode/comments/a86jgt/2018_day_21_solutions/ec8g4h2
    5_885_821
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
