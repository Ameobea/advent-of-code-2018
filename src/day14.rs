const INPUT: &str = include_str!("../input/day14.txt");

fn parse_input() -> usize {
    INPUT.trim().parse().unwrap()
}

fn part1() -> u64 {
    let input = parse_input();
    let mut recipes = vec![3, 7];

    let mut elf1_index = 0;
    let mut elf2_index = 1;
    for _ in 0..(input + 10) {
        (recipes[elf1_index] + recipes[elf2_index])
            .to_string()
            .chars()
            .for_each(|c| recipes.push(c.to_digit(10).unwrap()));

        elf1_index = (elf1_index + (1 + recipes[elf1_index]) as usize) % recipes.len();
        elf2_index = (elf2_index + (1 + recipes[elf2_index]) as usize) % recipes.len();
    }

    recipes[input..(input + 10)]
        .iter()
        .enumerate()
        .fold(0u64, |acc, (i, d)| {
            acc + (10u64.pow(9 - i as u32) as u64 * *d as u64)
        })
}

fn part2() -> usize {
    let input: Vec<u32> = parse_input()
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut recipes = vec![3, 7];

    let mut elf1_index = 0;
    let mut elf2_index = 1;
    let mut res = None;
    loop {
        (recipes[elf1_index] + recipes[elf2_index])
            .to_string()
            .chars()
            .for_each(|c| {
                recipes.push(c.to_digit(10).unwrap());

                if recipes.len() > input.len()
                    && &recipes[recipes.len() - input.len()..recipes.len()] == input.as_slice()
                {
                    res = Some(recipes.len() - input.len());
                }
            });
        if res.is_some() {
            break;
        }

        elf1_index = (elf1_index + (1 + recipes[elf1_index]) as usize) % recipes.len();
        elf2_index = (elf2_index + (1 + recipes[elf2_index]) as usize) % recipes.len();
    }

    res.unwrap()
}

pub fn run() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}
