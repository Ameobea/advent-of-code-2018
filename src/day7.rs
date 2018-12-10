use std::collections::{HashMap, HashSet};
use std::usize;

use regex::Regex;
use slab::Slab;

lazy_static! {
    static ref RGX: Regex =
        Regex::new("Step (.) must be finished before step (.) can begin\\.").unwrap();
}

const INPUT: &str = include_str!("../input/day7.txt");

fn parse_input() -> impl Iterator<Item = (char, char)> {
    RGX.captures_iter(INPUT)
        .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
}

#[derive(Debug)]
struct DagNode(Vec<usize>, char);

impl DagNode {
    pub fn new(c: char) -> Self {
        DagNode(Vec::new(), c)
    }
}

fn init_nodes() -> (
    Slab<DagNode>,
    HashMap<char, usize>,
    HashMap<char, usize>,
    HashSet<char>,
) {
    let mut nodes: Slab<DagNode> = Slab::new();
    let mut prereq_counts: HashMap<char, usize> = HashMap::new();
    let mut key_mappings: HashMap<char, usize> = HashMap::new();
    let mut linked_to = HashSet::new(); // used to find the head

    for (from, to) in parse_input() {
        *prereq_counts.entry(to).or_insert(0) += 1;
        linked_to.insert(to);
        let from_key = *key_mappings
            .entry(from)
            .or_insert_with(|| nodes.insert(DagNode::new(from)));
        let to_key = *key_mappings
            .entry(to)
            .or_insert_with(|| nodes.insert(DagNode::new(to)));
        let from_node = &mut nodes[from_key];

        from_node.0.push(to_key);
    }

    (nodes, prereq_counts, key_mappings, linked_to)
}

fn init_active_nodes<'a>(
    nodes: &'a Slab<DagNode>,
    prereq_counts: &mut HashMap<char, usize>,
    key_mappings: &HashMap<char, usize>,
    linked_to: &HashSet<char>,
) -> Vec<&'a DagNode> {
    let mut active_nodes = Vec::new();
    (b'A'..=b'Z')
        .filter(|c| linked_to.get(&(*c as char)).is_none())
        .for_each(|head_char| {
            prereq_counts.insert(head_char as char, 0);
            let head_node = &nodes[key_mappings[&(head_char as char)]];
            active_nodes.push(head_node);
        });

    active_nodes
}

fn part1() -> String {
    let (nodes, mut prereq_counts, key_mappings, linked_to) = init_nodes();
    let mut active_nodes = init_active_nodes(&nodes, &mut prereq_counts, &key_mappings, &linked_to);

    let mut ordered = String::new();
    while !active_nodes.is_empty() {
        let (best_index, _) = active_nodes
            .iter()
            .enumerate()
            .min_by_key(|(_i, node)| node.1)
            .unwrap();

        let best_node = active_nodes.swap_remove(best_index);
        ordered.push(best_node.1);
        for child_key in best_node.0.iter() {
            let node = &nodes[*child_key];
            *prereq_counts.get_mut(&node.1).unwrap() -= 1;
            if prereq_counts[&node.1] == 0 {
                active_nodes.push(node);
            }
        }
    }

    ordered
}

fn dur(c: char) -> usize {
    ((c as u8) - 64) as usize + 60
}

fn part2() -> usize {
    let (nodes, mut prereq_counts, key_mappings, linked_to) = init_nodes();
    let mut active_nodes = init_active_nodes(&nodes, &mut prereq_counts, &key_mappings, &linked_to);

    let mut time = 0;
    let mut workers: [Option<(&DagNode, usize)>; 5] = [None; 5];
    while !active_nodes.is_empty() || workers.iter().any(|worker| worker.is_some()) {
        time += 1;

        // check off completed work
        for slot in &mut workers {
            let c_opt: Option<&DagNode> = if let Some(worker) = slot {
                worker.1 -= 1;
                if worker.1 == 0 {
                    Some(worker.0)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(completed_node) = c_opt {
                *slot = None;
                for child_key in (completed_node.0).iter() {
                    let node = &nodes[*child_key];
                    *prereq_counts.get_mut(&node.1).unwrap() -= 1;
                    if prereq_counts[&node.1] == 0 {
                        active_nodes.push(node);
                    }
                }
            }
        }

        // get workers work
        for slot in &mut workers {
            if slot.is_none() {
                if let Some(node) = active_nodes.pop() {
                    *slot = Some((node, dur(node.1)));
                }
            }
        }
    }

    time - 1
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
