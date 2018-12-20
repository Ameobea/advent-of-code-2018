use std::collections::HashMap;

mod parser;

const INPUT: &[u8] = include_bytes!("../../input/day20.txt");

#[derive(Clone, Debug)]
pub enum MovementDirection {
    N,
    S,
    E,
    W,
    Branch(Vec<Vec<MovementDirection>>, bool),
}

impl MovementDirection {
    pub fn offset(&self) -> (isize, isize) {
        match self {
            MovementDirection::N => (0, -1),
            MovementDirection::S => (0, 1),
            MovementDirection::E => (1, 0),
            MovementDirection::W => (-1, 0),
            _ => panic!("Tried to get offset on branch"),
        }
    }
}

type Cursor = ((isize, isize), usize);

fn record_door_distance(
    offset_x: isize,
    offset_y: isize,
    distance: usize,
    distances: &mut HashMap<(isize, isize), usize>,
) {
    distances
        .entry((offset_x, offset_y))
        .and_modify(|last_distance| {
            // We only want to keep track of the shortest recorded path to each
            // door, discarding any longer paths
            if *last_distance > distance {
                *last_distance = distance;
            }
        })
        .or_insert(distance);
}

/// `cur_distance` is how far has been walked before getting to the starting point of this current
/// path.
///
/// (offset_x, offset_y)` is the offset of this current path's starting point wrt the original
/// starting point of the whole direction set.
fn traverse_path(
    directions_iterator: impl Iterator<Item = MovementDirection> + Clone,
    door_distances: &mut HashMap<(isize, isize), usize>,
    (offset_x, offset_y): (isize, isize),
    cur_distance: usize,
) -> impl Iterator<Item = Cursor> {
    // Offset from the starting point of this path
    let (mut local_offset_x, mut local_offset_y) = (0, 0);
    // How far we've walked along the current path so far
    let mut local_distance = 0;
    // a set of relative (wrt `(offset_x, offset_y)`) cursor offsets and relative (wrt
    // `cur_distance`) distances representing the endpoints of all possible branches that have
    // been taken so far along the current path.
    let mut cursors: Vec<Cursor> = Vec::new();

    for dir in directions_iterator {
        match dir {
            MovementDirection::Branch(branches, has_empty) => {
                let branch_cursors_iter = branches
                    .into_iter()
                    .flat_map(|branch| {
                        traverse_path(
                            branch.into_iter(),
                            door_distances,
                            (offset_x + local_offset_x, offset_y + local_offset_y),
                            cur_distance + local_distance,
                        )
                    })
                    .map(|((cursor_offset_x, cursor_offset_y), cursor_distance)| {
                        // Since the local distance and offset may end up changing as later items in
                        // the current path are processed, we subtract them out of the cursors.
                        //
                        // This can be thought of as a bunch of pens that all move together.  We
                        // have the big arm that holds all of the pens located at `(offset_x,
                        // offset_y)` and, during the process of processing the current path, move
                        // it `(local_offset_x, local_offset_y)`.  Each step of the way, we want to
                        // record the distances for each of the cursors, so the current local offset
                        // and distance are added to them when distances are inserted into the
                        // hashmap.
                        (
                            (
                                cursor_offset_x - local_offset_x,
                                cursor_offset_y - local_offset_y,
                            ),
                            cursor_distance - local_distance,
                        )
                    });

                cursors.extend(branch_cursors_iter);
                if has_empty {
                    cursors.push(((offset_x, offset_y), cur_distance));
                }
            },
            dir => {
                let (cur_offset_x, cur_offset_y) = dir.offset();
                local_offset_x += cur_offset_x;
                local_offset_y += cur_offset_y;
                local_distance += 1;

                for &((cursor_offset_x, cursor_offset_y), cursor_distance) in &cursors {
                    record_door_distance(
                        cursor_offset_x + local_offset_x,
                        cursor_offset_y + local_offset_y,
                        cursor_distance + local_distance,
                        door_distances,
                    );
                }
                record_door_distance(
                    offset_x + local_offset_x,
                    offset_y + local_offset_y,
                    cur_distance + local_distance,
                    door_distances,
                );
            },
        }
    }

    if cursors.is_empty() {
        cursors.push(((offset_x, offset_y), cur_distance));
    }

    // Before returning the cursors to be merged in with the cursors of other paths, we normalize
    // them with the final local offset and distance of the path.
    cursors.into_iter().map(
        move |((cursor_offset_x, cursor_offset_y), cursor_distance)| {
            (
                (
                    cursor_offset_x + local_offset_x,
                    cursor_offset_y + local_offset_y,
                ),
                cursor_distance + local_distance,
            )
        },
    )
}

fn part1() -> usize {
    let directions = parser::parse_input(INPUT);

    let mut door_distances: HashMap<(isize, isize), usize> = HashMap::new();
    let _ = traverse_path(
        directions.into_iter(),
        &mut door_distances,
        (0isize, 0isize),
        0,
    )
    .collect::<Vec<_>>();

    let (_, shortest_distance_to_furthest_door) =
        door_distances.drain().max_by_key(|&(_, dst)| dst).unwrap();

    shortest_distance_to_furthest_door
}

fn part2() -> usize {
    let directions = parser::parse_input(INPUT);

    let mut door_distances: HashMap<(isize, isize), usize> = HashMap::new();
    let _ = traverse_path(
        directions.into_iter(),
        &mut door_distances,
        (0isize, 0isize),
        0,
    )
    .collect::<Vec<_>>();

    door_distances
        .drain()
        .filter(|&(_, distance)| distance >= 1000)
        .count()

    // 10317: too high
    // 10312: too high
}

#[test]
fn cursor_generation() {
    let directions = parser::parse_input(b"^EEE(NN|SS|(E|N|))EEE$");

    let cur_distance = 3usize;
    let (offset_x, offset_y) = (3, 0);
    let expected_branch_cursors = vec![
        ((3, -2), 5),
        ((3, 2), 5),
        ((4, 0), 4),
        ((3, -1), 4),
        ((3, 0), 3),
    ];

    println!("{:?}", directions[3]);
    let branch_paths = if let MovementDirection::Branch(branch_paths, _) = &directions[3] {
        branch_paths.clone()
    } else {
        unreachable!();
    };
    let actual_branch_cursors: Vec<Cursor> = branch_paths
        .into_iter()
        .flat_map(|path| {
            traverse_path(
                path.into_iter(),
                &mut HashMap::new(),
                (offset_x, offset_y),
                cur_distance,
            )
        })
        .collect();

    assert_eq!(expected_branch_cursors, actual_branch_cursors);
}

#[test]
fn distance_hashmap_population() {
    let mut expected = HashMap::new();
    let expected_s = "...5678.\n...4567.\n.1234567\n...4....\n...5678.";
    let c_iter = expected_s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| -> ((isize, isize), Option<usize>) {
                    (
                        (x as isize, y as isize - 2),
                        c.to_digit(10).map(|d| d as usize),
                    )
                })
        })
        .filter_map(|(coord, distance_opt)| distance_opt.map(|distance| (coord, distance)));

    for (coord, distance) in c_iter {
        expected.insert(coord, distance);
    }
    assert_eq!(expected.len(), 20);

    let mut actual = HashMap::new();
    let directions = parser::parse_input(b"^EEE(NN|SS|(E|N|))EEE$");
    let _ =
        traverse_path(directions.into_iter(), &mut actual, (0isize, 0isize), 0).collect::<Vec<_>>();

    assert_eq!(expected, actual);
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
