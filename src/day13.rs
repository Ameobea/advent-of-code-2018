use std::collections::{HashMap, HashSet};
const INPUT: &str = include_str!("../input/day13.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Track {
    Vertical,
    Horizontal,
    Intersection,
    ConnectRight,
    ConnectLeft,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn left(self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub fn right(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Cart {
    pub direction: Direction,
    pub x: usize,
    pub y: usize,
    pub turn_direction: u8,
}

impl Cart {
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        Cart {
            x,
            y,
            direction,
            turn_direction: 0,
        }
    }

    pub fn next_movement(&mut self, track: Track) {
        if track == Track::Intersection {
            let next_direction = match self.turn_direction {
                0 => self.direction.left(),
                1 => self.direction,
                2 => self.direction.right(),
                _ => unreachable!(),
            };
            self.turn_direction = (self.turn_direction + 1) % 3;
            self.direction = next_direction;

            let mut c = self.clone();
            c.next_movement(Track::Horizontal);
            self.x = c.x;
            self.y = c.y;
            return;
        }

        let (offset_x, offset_y, new_direction) = match self.direction {
            Direction::Up => match track {
                Track::ConnectRight => (1, 0, Direction::Right),
                Track::ConnectLeft => (-1, 0, Direction::Left),
                _ => (0, -1, self.direction),
            },
            Direction::Down => match track {
                Track::ConnectRight => (-1, 0, Direction::Left),
                Track::ConnectLeft => (1, 0, Direction::Right),
                _ => (0, 1, self.direction),
            },
            Direction::Left => match track {
                Track::ConnectRight => (0, 1, Direction::Down),
                Track::ConnectLeft => (0, -1, Direction::Up),
                _ => (-1, 0, self.direction),
            },
            Direction::Right => match track {
                Track::ConnectRight => (0, -1, Direction::Up),
                Track::ConnectLeft => (0, 1, Direction::Down),
                _ => (1, 0, self.direction),
            },
        };
        self.x = (self.x as isize + offset_x) as usize;
        self.y = (self.y as isize + offset_y) as usize;
        self.direction = new_direction;
    }
}

fn parse_input() -> (Vec<Vec<Option<Track>>>, Vec<Cart>) {
    let mut carts = Vec::new();
    let tracks = INPUT
        .lines()
        .filter(|l| l.len() > 1)
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    let mut mkcart = |direction: Direction| carts.push(Cart::new(x, y, direction));

                    match c {
                        ' ' => None,
                        '|' => Some(Track::Vertical),
                        '-' => Some(Track::Horizontal),
                        '+' => Some(Track::Intersection),
                        '>' => {
                            mkcart(Direction::Right);
                            Some(Track::Horizontal)
                        },
                        '<' => {
                            mkcart(Direction::Left);
                            Some(Track::Horizontal)
                        },
                        '^' => {
                            mkcart(Direction::Up);
                            Some(Track::Vertical)
                        },
                        'v' => {
                            mkcart(Direction::Down);
                            Some(Track::Vertical)
                        },
                        '/' => Some(Track::ConnectRight),
                        '\\' => Some(Track::ConnectLeft),
                        _ => unreachable!(),
                    }
                })
                .collect()
        })
        .collect();
    (tracks, carts)
}

#[allow(dead_code)]
fn debug_tracks_carts(tracks: &[Vec<Option<Track>>], carts: &[Cart], pause: bool) {
    let mut s = String::new();
    for y in 0..tracks.len() {
        // NOT 141,45; NOT 141,44; NOT 48,31; NOT 105,11
        for x in 0..tracks[y].len() {
            let mut found_cart = false;
            for cart in carts {
                if cart.x == x && cart.y == y {
                    let c = match cart.direction {
                        Direction::Up => '^',
                        Direction::Right => '>',
                        Direction::Left => '<',
                        Direction::Down => 'v',
                    };
                    print!("{}", c);
                    found_cart = true;
                    break;
                }
            }

            if !found_cart {
                let c = match tracks[y][x] {
                    None => ' ',
                    Some(Track::ConnectLeft) => '\\',
                    Some(Track::ConnectRight) => '/',
                    Some(Track::Horizontal) => '-',
                    Some(Track::Vertical) => '|',
                    Some(Track::Intersection) => '+',
                };
                print!("{}", c);
            }
        }
        println!();
    }
    println!();
    if pause {
        std::io::stdin().read_line(&mut s).unwrap();
    }
    println!("{:?}", carts);
}

fn find_collision_location(mut iter: impl Iterator<Item = Cart>) -> Option<(usize, usize)> {
    let mut uniq = HashSet::new();
    iter.find(move |cart| !uniq.insert((cart.x, cart.y)))
        .map(|cart| (cart.x, cart.y))
}

fn part1() -> (usize, usize) {
    let (tracks, mut carts) = parse_input();

    loop {
        carts.sort_unstable_by(|c1, c2| (c1.y, c1.x).cmp(&(c2.y, c2.x)));
        let mut new_carts = carts.clone();
        for (i, cart) in carts.iter_mut().enumerate() {
            cart.next_movement(tracks[cart.y][cart.x].unwrap());
            new_carts[i] = cart.clone();
            if let Some(collision) = find_collision_location(new_carts.iter().cloned()) {
                return collision;
            }
        }
        carts = new_carts;
    }
}

fn find_collision_location_and_indices(
    iter: impl Iterator<Item = Cart>,
    removed_indices: &[usize],
) -> Option<((usize, usize), usize, usize)> {
    let mut uniq: HashMap<(usize, usize), usize> = HashMap::new();
    iter.enumerate()
        .find(|(i, cart)| {
            // skip removed carts
            if removed_indices.iter().any(|&i2| *i == i2) {
                return false;
            }

            if uniq.get(&(cart.x, cart.y)).is_some() {
                true
            } else {
                uniq.insert((cart.x, cart.y), *i);
                false
            }
        })
        .map(|(i, cart)| ((cart.x, cart.y), i, uniq[&(cart.x, cart.y)]))
}

fn part2() -> (usize, usize) {
    let (tracks, mut carts) = parse_input();

    loop {
        carts.sort_unstable_by(|c1, c2| (c1.y, c1.x).cmp(&(c2.y, c2.x)));
        let mut new_carts = carts.clone();
        let mut removed_indices = Vec::new();
        for (i, cart) in carts.iter_mut().enumerate() {
            cart.next_movement(tracks[cart.y][cart.x].unwrap());
            new_carts[i] = cart.clone();
            if let Some(((_collion_x, _collision_y), i1, i2)) =
                find_collision_location_and_indices(new_carts.iter().cloned(), &removed_indices)
            {
                removed_indices.push(i1);
                removed_indices.push(i2);
            }
        }
        carts = new_carts
            .into_iter()
            .enumerate()
            .filter(|(i, _cart)| removed_indices.iter().all(|&i2| *i != i2))
            .map(|(_i, cart)| cart)
            .collect();
        if carts.len() == 1 {
            let last_cart = &carts[0];
            return (last_cart.x, last_cart.y);
        }
    }
}

pub fn run() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}
