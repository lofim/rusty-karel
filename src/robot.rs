use helpers::Size;
use helpers::Position;

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    North,
    East,
    South,
    West,
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
    
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    
    pub fn add_beeper(&mut self) {
        self.beepers += 1;
    }
    
    pub fn remove_beeper(&mut self) {
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
