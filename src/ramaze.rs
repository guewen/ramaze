use std::fmt;

struct Move {
    dx: i8,
    dy: i8,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.dx, self.dy)
    }
}

struct Location {
    x: u32,
    y: u32,
}

trait Movable {
    fn relative_move(&self, direction: Move) -> Location;
}

impl Movable for Location {
    fn relative_move(&self, direction: Move) -> Location {
        Location { x: self.x + direction.dx as u32, y: self.y + direction.dy as u32}
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

struct Door {
    from: Location,
    to: Location,
}

struct MazeGrid {
    width: u32,
    height: u32,
    doors: Vec<Door>,
    visited: Vec<Location>,
}

impl MazeGrid {
    fn new(width: u32, height: u32) -> MazeGrid {
        MazeGrid {
            width: width,
            height: height,
            doors: vec![],
            visited: vec![],
        }
    }
}

impl fmt::Display for MazeGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MazeGrid ({}, {})", self.width, self.height)
    }
}

fn move_from_direction(dir: Direction) -> Move {
    match dir {
        Direction::North => Move { dx:  0, dy:  1 },
        Direction::East  => Move { dx:  1, dy:  0 },
        Direction::West  => Move { dx: -1, dy:  0 },
        Direction::South => Move { dx:  0, dy: -1 },
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
    let maze = MazeGrid::new(32, 32);
    println!("{}", maze);
}
