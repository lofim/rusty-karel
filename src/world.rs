use helpers::Size;
use helpers::compute_index;

use robot::Orientation;
use robot::Robot;

// World definition
#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Wall,
    Beepers(Size),
    Robot(Orientation)
}

pub struct World {
    karel: Robot,
    height: Size,
    width: Size,
    tiles: Vec<Tile>
}

impl World {
    pub fn new(height: Size, width: Size, karel: Robot, tiles: Vec<Tile>) -> World {
        World {
            height: height,
            width: width,
            karel: karel,
            tiles: tiles
        }
    }
    
    pub fn render(&self) -> String {
        let mut rendered_world = String::new();
        
        for (index, tile) in self.tiles.iter().enumerate() {
            if index as u32  % self.width == 0 {
                rendered_world.push_str("\n");
            }
            
            rendered_world.push_str(" ");
            rendered_world.push_str(&self.render_tile(tile));
        }
        
        rendered_world
    }
    
    fn render_tile(&self, tile: &Tile) -> String {
        match *tile {
            Tile::Empty => ".".to_string(),
            Tile::Wall => "W".to_string(),
            Tile::Beepers(_) => "*".to_string(),
            Tile::Robot(ref orientation) => self.render_orientation(orientation), 
        }
    }
    
    fn render_orientation(&self, orientation: &Orientation) -> String {
        match *orientation {
            Orientation::North => "^".to_string(),
            Orientation::East => ">".to_string(),
            Orientation::South => "v".to_string(),
            Orientation::West => "<".to_string(),
        }
    }
    
    pub fn project_robot(&mut self) {
        let (position, orientation, _) = self.karel.info();
        let index = compute_index(&position, self.width) as usize;
        self.tiles[index] = Tile::Robot(orientation); 
    }
    
    pub fn dimensions(&self) -> (Size, Size) {
        (self.width, self.height)
    }
    
    pub fn get_robot(&self) -> &Robot {
        &self.karel
    }
}