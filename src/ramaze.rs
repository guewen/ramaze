use std::fmt;

struct Move {
    x: i8,
    y: i8,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Location {
    x: u32,
    y: u32,
}

trait Movable {
    fn relative_move(&self, coords: Move) -> Location;
}

impl Movable for Location {
    fn relative_move(&self, coords: Move) -> Location {
        Location { x: self.x + coords.x as u32, y: self.y + coords.y as u32}
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

enum Direction {
    North,
    East,
    West,
    South,
}

fn move_from_direction(dir: Direction) -> Move {
    match dir {
        Direction::North => Move { x:  0, y:  1 },
        Direction::East  => Move { x:  1, y:  0 },
        Direction::West  => Move { x:  0, y: -1 },
        Direction::South => Move { x: -1, y:  0 },
    }
}

fn main() {
    println!("{}", move_from_direction(Direction::North));
    println!("{}", move_from_direction(Direction::East));
    println!("{}", move_from_direction(Direction::West));
    println!("{}", move_from_direction(Direction::South));
    let loc = Location { x: 5, y: 2 };
    println!("Location: {}", loc);
    let new_loc = loc.relative_move(move_from_direction(Direction::South));
    println!("Location: {}", new_loc);
}
