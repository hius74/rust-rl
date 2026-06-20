
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left = 0,
    Right = 1
}

impl Direction {
    pub const COUNT: usize = 2;

    pub fn from_index(index: usize) -> Direction {
        match index {
            0 => Direction::Left,
            1 => Direction::Right,
            _ => panic!("Invalid direction index {}", index)
        }
    }

    pub fn to_index(self) -> usize {
        self as usize
    }
}