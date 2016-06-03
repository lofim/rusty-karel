use helpers::Size;
use helpers::Position;

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
            position: position,
            orientation: orientation,
            beepers: beepers
        }
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    #[allow(dead_code)]
    pub fn add_beeper(&mut self) {
        // this need to be rewritten using checked_add
        self.beepers += 1;
    }

    #[allow(dead_code)]
    pub fn remove_beeper(&mut self) {
        // This needs to be rewritten using checked_sub
        self.beepers -= 1;
    }

    pub fn turn_left(&mut self) {
        self.orientation = match self.orientation {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
        }
    }

    pub fn info(&self) -> (Position, Orientation, Size) {
        (self.position, self.orientation, self.beepers)
    }
}
