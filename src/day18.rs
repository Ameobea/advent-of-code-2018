use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day18.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Cell {
    Ground,
    Trees,
    Lumberyard,
}

impl Cell {
    pub fn next(self, neighbors: impl Iterator<Item = Cell>) -> Self {
        match self {
            Cell::Ground =>
                if neighbors.filter(|&c| c == Cell::Trees).count() >= 3 {
                    return Cell::Trees;
                },
            Cell::Trees =>
                if neighbors.filter(|&c| c == Cell::Lumberyard).count() >= 3 {
                    return Cell::Lumberyard;
                },
            Cell::Lumberyard =>
                if neighbors.fold((false, false), |(found_lumberyard, found_trees), c| {
                    (
                        c == Cell::Lumberyard || found_lumberyard,
                        c == Cell::Trees || found_trees,
                    )
                }) == (true, true)
                {
                    return self;
                } else {
                    return Cell::Ground;
                },
        }

        return self;
    }
}

fn parse_input() -> Vec<Vec<Cell>> {
    INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Cell::Lumberyard,
                    '|' => Cell::Trees,
                    '.' => Cell::Ground,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn iter_neighbors<'a>(
    cells: &'a [Vec<Cell>],
    cur_x: usize,
    cur_y: usize,
) -> impl Iterator<Item = Cell> + 'a {
    let y_range = (cur_y.saturating_sub(1))..=(cur_y + 1).min(cells.len() - 1);
    y_range.flat_map(move |y| {
        let x_range = cur_x.saturating_sub(1)..=(cur_x + 1).min(cells[y].len() - 1);
        x_range
            .map(move |x| (x, y))
            .filter(move |(x, y)| (*x, *y) != (cur_x, cur_y))
            .map(move |(x, y)| cells[y][x])
    })
}

fn tick(cells: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut new_cells = Vec::with_capacity(cells.len());
    for (y, row) in cells.iter().enumerate() {
        let mut new_row = Vec::with_capacity(row.len());
        for (x, c) in row.into_iter().enumerate() {
            let new_cell = (*c).next(iter_neighbors(&cells, x, y));
            new_row.push(new_cell);
        }
        new_cells.push(new_row);
    }

    assert_eq!(cells.len(), new_cells.len());
    new_cells
}

#[allow(dead_code)]
fn debug_world(world: &[Vec<Cell>]) {
    for row in world {
        for c in row {
            print!(
                "{}",
                match c {
                    Cell::Ground => '.',
                    Cell::Lumberyard => '#',
                    Cell::Trees => '|',
                }
            );
        }

        println!();
    }
    println!("\n\n");
}

fn compute_solution(world: &[Vec<Cell>]) -> usize {
    let (tree_count, lumberyard_count) = world.into_iter().flat_map(|row| row.into_iter()).fold(
        (0, 0),
        |(trees, lumberyards), c| match c {
            Cell::Trees => (trees + 1, lumberyards),
            Cell::Lumberyard => (trees, lumberyards + 1),
            Cell::Ground => (trees, lumberyards),
        },
    );
    tree_count * lumberyard_count
}

fn part1() -> usize {
    let mut world = parse_input();
    for _ in 0..10 {
        world = tick(world);
    }

    compute_solution(&world)
}

type State = Vec<Vec<Cell>>;

const TARGET_TICK: usize = 1_000_000_000;

fn find_cycle(mut state: State) -> (usize, usize, State) {
    // This holds `State -> tick` snapshots of the state that are used to identify the first point
    // where a cycle occurs.
    let mut cycles: HashMap<State, usize> = HashMap::new();

    let (mut first_cycle_tick, mut first_repeat_tick) = (0, 0);
    for cur_tick in 1.. {
        state = tick(state);
        if let Some(cycle_start_tick) = cycles.insert(state.clone(), cur_tick) {
            first_cycle_tick = cycle_start_tick;
            first_repeat_tick = cur_tick;
            break;
        }
    }

    (first_cycle_tick, first_repeat_tick, state)
}

fn part2() -> usize {
    let state = parse_input();

    let (first_cycle_tick, first_repeat_tick, mut new_state) = find_cycle(state);
    let cycle_length = first_repeat_tick - first_cycle_tick;

    let start_tick = TARGET_TICK - ((TARGET_TICK - first_cycle_tick) % cycle_length);
    assert_eq!((start_tick - first_cycle_tick) % cycle_length, 0);
    for _ in start_tick..TARGET_TICK {
        new_state = tick(new_state);
    }

    compute_solution(&new_state)
}

#[test]
fn test_cycle_durability() {
    let state = parse_input();
    let (first_cycle_tick, first_repeat_tick, cycle_init_state) = find_cycle(state.clone());
    let cycle_length = first_repeat_tick - first_cycle_tick;
    println!("Cycle length: {}", cycle_length);

    let mut test_state = cycle_init_state.clone();
    for _ in 0..5 {
        for _ in 0..cycle_length {
            test_state = tick(test_state);
        }
        assert_eq!(test_state, cycle_init_state);
    }
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
