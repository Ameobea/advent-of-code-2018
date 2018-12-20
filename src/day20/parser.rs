use super::MovementDirection;

fn movement_direction_from_c(c: char) -> MovementDirection {
    match c {
        'N' => MovementDirection::N,
        'S' => MovementDirection::S,
        'E' => MovementDirection::E,
        'W' => MovementDirection::W,
        _ => unreachable!(),
    }
}

named!(branch<&[u8], MovementDirection>, do_parse!(
    tag!("(") >>
    path_list: separated_list!(tag!("|"), many0!(direction)) >>
    has_empty: opt!(char!('|')) >>
    tag!(")") >>
    (MovementDirection::Branch(path_list, has_empty.is_some()))
));

named!(
    direction<&[u8], MovementDirection>,
    alt!(map!(one_of!("NSEW"), movement_direction_from_c) | branch)
);

named!(all_directions<&[u8], Vec<MovementDirection> >, do_parse!(
    tag!("^") >>
    directions: many0!(direction) >>
    tag!("$") >>
    (directions)
));

pub fn parse_input(raw: &[u8]) -> Vec<MovementDirection> {
    all_directions(raw).expect("Failed to parse input").1
}
