#![feature(hash)]

use std::fmt;
use std::collections::HashSet;

#[derive(Debug)]
struct Move {
    dx: i8,
    dy: i8,
}

impl Move {
    fn from_direction(dir: Direction) -> Move {
        match dir {
            Direction::North => Move { dx:  0, dy:  1 },
            Direction::East  => Move { dx:  1, dy:  0 },
            Direction::West  => Move { dx: -1, dy:  0 },
            Direction::South => Move { dx:  0, dy: -1 },
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
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

enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Door {
    from: Location,
    to: Location,
}

struct MazeGrid {
    width: u32,
    height: u32,
    doors: HashSet<Door>,
    visited: HashSet<Location>,
}

impl MazeGrid {
    fn new(width: u32, height: u32) -> MazeGrid {
        MazeGrid {
            width: width,
            height: height,
            doors: HashSet::new(),
            visited: HashSet::new(),
        }
    }
}

impl fmt::Display for MazeGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MazeGrid ({}, {})", self.width, self.height)
    }
}

fn main() {
    println!("{:?}", Move::from_direction(Direction::North));
    println!("{:?}", Move::from_direction(Direction::East));
    println!("{:?}", Move::from_direction(Direction::West));
    println!("{:?}", Move::from_direction(Direction::South));
    let loc = Location { x: 5, y: 2 };
    println!("{:?}", loc);
    let new_loc = loc.relative_move(Move::from_direction(Direction::South));
    println!("{:?}", new_loc);
    let maze = MazeGrid::new(32, 32);
    println!("{}", maze);
}
