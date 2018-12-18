const INPUT: &str = include_str!("../input/day18.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Ground,
    Trees,
    Lumberyard,
}

impl Cell {
    pub fn next(
        self,
        top: Option<Cell>,
        right: Option<Cell>,
        bottom: Option<Cell>,
        left: Option<Cell>
    ) -> Self {
        let neighbors = [top, right, bottom, left];

        match self {
            Cell::Ground => {
                if neighbors
                    .iter()
                    .filter(|&&c| c == Some(Cell::Trees))
                    .count()
                    >= 3
                {
                    return Cell::Trees;
                }},
            Cell::Trees => {
                if neighbors
                    .iter()
                    .filter(|&&c| c == Some(Cell::Lumberyard))
                    .count()
                    >= 3
                {
                    return Cell::Lumberyard;
                }},
            Cell::Lumberyard => {
                if neighbors.iter().any(|&c| c == Some(Cell::Lumberyard)) {
                    return self;
                } else {
                    return Cell::Ground;
                }},
        }

        return Self;
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

fn tick(cells: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut new_cells = cells.clone();
    for y in 0..cells.len() {
        let row = &cells[y];
        let mut new_row = Vec::with_capacity(row.len());
        for (x, c) in row.into_iter().enumerate() {
            let new_cell = c.next(
                cells.get(y - 1).map(|row| row[x]),
                cells[y].get(x + 1).cloned(),
                cells.get(y + 1).map(|row| row[x]),
                cells[y].get(x - 1).cloned(),
            );
            new_row.push(new_cell);
        }
        new_cells.push(new_row);
    }

    new_cells
}

fn part1() -> usize { 0 }

fn part2() -> usize { 0 }

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
