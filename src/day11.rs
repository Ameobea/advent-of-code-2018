const INPUT: &str = include_str!("../input/day11.txt");

fn parse_input() -> usize {
    INPUT.trim().parse().unwrap()
}

fn populate_powers() -> [[i32; 300]; 300] {
    let input = parse_input();

    let mut powers = [[0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {
            let rack_id = x + 10;
            let mut power = rack_id * y;
            power += input;
            power *= rack_id;
            let power_s = power.to_string();
            let hundreds: u32 = match power_s.chars().nth(power_s.len() - 3) {
                Some(c) => c.to_digit(10).unwrap(),
                None => 5,
            };
            powers[y][x] = (hundreds as i32) - 5;
        }
    }

    powers
}

fn part1() -> (usize, usize) {
    let powers = populate_powers();
    let (mut best_x, mut best_y, mut best) = (0, 0, 0);
    for y in 0..(300 - 3) {
        for x in 0..(300 - 3) {
            let mut sum = 0;
            for ya in 0..3 {
                for xa in 0..3 {
                    sum += powers[y + ya][x + xa];
                }
            }

            if sum > best {
                best_x = x;
                best_y = y;
                best = sum;
            }
        }
    }

    (best_x, best_y)
}

fn part2() -> (usize, usize, usize) {
    let powers: [[i32; 300]; 300] = populate_powers();

    let (mut best_x, mut best_y, mut best, mut best_size) = (0, 0, 0, 0);
    let mut sums = [[0; 300]; 300];
    for size in 1..300 {
        for y in 0..(300 - size) {
            for x in 0..(300 - size) {
                for ya in 0..size {
                    sums[y][x] += powers[y + ya][x + size - 1];
                }

                for xa in 0..(size - 1) {
                    sums[y][x] += powers[y + size - 1][x + xa];
                }

                if sums[y][x] > best {
                    best_x = x;
                    best_y = y;
                    best = sums[y][x];
                    best_size = size;
                }
            }
        }
    }

    (best_x, best_y, best_size)
}

pub fn run() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}
