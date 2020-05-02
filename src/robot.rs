use crate::helpers::Position;
use crate::helpers::Size;

use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl FromStr for Orientation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "north" => Ok(Orientation::North),
            "east" => Ok(Orientation::East),
            "south" => Ok(Orientation::South),
            "west" => Ok(Orientation::West),
            _ => Err("Error Parsing Orientation".to_owned()),
        }
    }
}

pub struct Robot {
    position: Position,
    beepers: Size,
    orientation: Orientation,
}

impl Robot {
    pub fn new(position: Position, orientation: Orientation, beepers: Size) -> Robot {
        Robot {
            position,
            orientation,
            beepers,
        }
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    #[allow(dead_code)]
    pub fn add_beeper(&mut self) -> bool {
        self.beepers += 1;

        true
    }

    #[allow(dead_code)]
    pub fn remove_beeper(&mut self) -> bool {
        if self.beepers > 0 {
            self.beepers -= 1;
            return true;
        }

        false
    }

    pub fn turn_left(&mut self) -> bool {
        self.orientation = match self.orientation {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
        };

        true
    }

    pub fn info(&self) -> (Position, Orientation, Size) {
        (self.position, self.orientation, self.beepers)
    }
}
