// Cartesian grid structures and utilities.

pub use super::infinite_grid::InfiniteGrid;
use std::slice::Iter;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[allow(dead_code)]
impl Dir {
    pub fn iter() -> Iter<'static, Dir> {
        static DIRECTIONS: [Dir; 4] = [
            Dir::Up,
            Dir::Right,
            Dir::Down,
            Dir::Left,
        ];
        DIRECTIONS.iter()
    }

    pub fn turn_right(&self) -> Dir {
        match self {
            &Dir::Up    => Dir::Right,
            &Dir::Right => Dir::Down,
            &Dir::Down  => Dir::Left,
            &Dir::Left  => Dir::Up,
        }
    }

    pub fn turn_left(&self) -> Dir {
        match self {
            &Dir::Up    => Dir::Left,
            &Dir::Right => Dir::Up,
            &Dir::Down  => Dir::Right,
            &Dir::Left  => Dir::Down,
        }
    }

    pub fn reverse(&self) -> Dir {
        match self {
            &Dir::Up    => Dir::Down,
            &Dir::Right => Dir::Left,
            &Dir::Down  => Dir::Up,
            &Dir::Left  => Dir::Right,
        }
    }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub struct Pos {
    pub row: isize,
    pub col: isize
}

#[macro_export]
macro_rules! pos {
    ($row:expr,$col:expr) => (
        Pos::new($row, $col)
    )
}

impl Pos {
    pub fn new(row: isize, col: isize) -> Self {
        Pos {
            row: row,
            col: col
        }
    }

    pub fn origin() -> Self {
        pos!(0, 0)
    }

    pub fn manhattan_distance(&self, other: &Pos) -> usize {
        ((self.row - other.row).abs() + (self.col - other.col).abs()) as usize
    }

    pub fn neighbor(&self, dir: Dir) -> Pos {
        match dir {
            Dir::Up    => Pos::new(self.row-1, self.col),
            Dir::Right => Pos::new(self.row,   self.col+1),
            Dir::Down  => Pos::new(self.row+1, self.col),
            Dir::Left  => Pos::new(self.row,   self.col-1),
        }
    }
}
