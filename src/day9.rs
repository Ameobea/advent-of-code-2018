use regex::Regex;
use slab::Slab;

lazy_static! {
    static ref RGX: Regex =
        Regex::new("(\\d+) players; last marble is worth (\\d+) points").unwrap();
}

const INPUT: &str = include_str!("../input/day9.txt");

fn parse_input() -> (usize, usize) {
    let captures = RGX.captures(INPUT).unwrap();
    (captures[1].parse().unwrap(), captures[2].parse().unwrap())
}

struct Node {
    pub val: usize,
    pub prev: usize,
    pub next: usize,
}

fn solve() -> (usize, usize) {
    let (players, last_marble_value) = parse_input();
    let mut board: Slab<Node> = Slab::new();

    let mut i = 0;
    let mut player_scores = vec![0; players];
    let mut cur_marble_key = board.insert(Node {
        val: 0,
        prev: 0,
        next: 0,
    });

    let remove_child = |i, board: &mut Slab<Node>| {
        let next = board[i].next;
        let new_child_key = board[next].next;
        let removed = board.remove(next);
        board[i].next = new_child_key;
        board[new_child_key].prev = removed.prev;
        removed.val
    };

    let insert_child = |i, val, board: &mut Slab<Node>| {
        let old_next = board[i].next;
        let new_child = Node {
            val,
            prev: board[old_next].prev,
            next: old_next,
        };
        let new_next = board.insert(new_child);
        board[i].next = new_next;
        board[old_next].prev = new_next;
    };

    let mut part1 = 0;
    loop {
        for player in 0..players {
            i += 1;
            if i % 23 == 0 {
                player_scores[player] += i;
                for _ in 0..=7 {
                    cur_marble_key = board[cur_marble_key].prev;
                }

                let removed = remove_child(cur_marble_key, &mut board);
                player_scores[player] += removed;
                cur_marble_key = board[cur_marble_key].next;
                continue;
            }

            cur_marble_key = board[cur_marble_key].next;
            insert_child(cur_marble_key, i, &mut board);
            cur_marble_key = board[cur_marble_key].next;

            if i == last_marble_value {
                part1 = *player_scores.iter().max().unwrap();
            } else if i == last_marble_value * 100 {
                return (part1, player_scores.into_iter().max().unwrap())
            }
        }
    }
}

pub fn run() {
    let (part1, part2) = solve();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
