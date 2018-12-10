const INPUT: &str = include_str!("../input/day8.txt");

#[derive(Default, Debug)]
struct Node {
    pub children: Vec<Node>,
    pub metadata: Vec<usize>,
}

impl Node {
    pub fn sum_metadata(&self) -> usize {
        let child_metadata_sums: usize = self.children.iter().map(|node| node.sum_metadata()).sum();
        self.metadata.iter().sum::<usize>() + child_metadata_sums
    }

    pub fn value(&self) -> usize {
        if self.children.is_empty() {
            self.sum_metadata()
        } else {
            self.metadata
                .iter()
                .filter(|&&i| i != 0 && i <= self.children.len())
                .map(|&i| self.children[i - 1].value())
                .sum()
        }
    }
}

fn parse_node(data: &mut Vec<usize>) -> Node {
    let child_count = data.pop().unwrap();
    let meta_count = data.pop().unwrap();
    let mut item = Node::default();

    for _ in 0..child_count {
        let child = parse_node(data);
        item.children.push(child);
    }

    for _ in 0..meta_count {
        let n = data.pop().unwrap();
        item.metadata.push(n);
    }

    item
}

fn parse_input() -> impl Iterator<Item = Node> {
    let mut data: Vec<usize> = INPUT
        .split_whitespace()
        .map(|n| -> usize { n.parse().unwrap() })
        .collect();
    data.reverse();

    let mut items = Vec::new();
    while !data.is_empty() {
        items.push(parse_node(&mut data));
    }

    items.into_iter()
}

fn part1() -> usize {
    parse_input().map(|node| node.sum_metadata()).sum()
}

fn part2() -> usize {
    parse_input().next().unwrap().value()
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
