use helpers::Size;
use helpers::compute_index;
use helpers::Position;

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
    heigth: Size,
    width: Size,
    tiles: Vec<Tile>
}

pub struct WorldBuilder {
    karel: Option<Robot>,
    heigth: Size,
    width: Size,
    tiles: Vec<Tile>
}

impl WorldBuilder {
    pub fn new() -> WorldBuilder {
        WorldBuilder {
            karel: None,
            width: 0,
            heigth: 0,
            tiles: Vec::new()
        } 
    }
    
    pub fn dimensions<'a>(&'a mut self, width: Size, heigth: Size) -> &'a mut WorldBuilder {
        self.heigth = heigth;
        self.width = width;
        
        self.tiles = vec![Tile::Empty; heigth as usize * width as usize];
        
        self
    }
    
    pub fn karel<'a>(&'a mut self, karel: Robot) -> &'a mut WorldBuilder {
        self.karel = Some(karel);
        
        self
    }
    
    pub fn tile<'a>(&'a mut self, position: Position, tile: Tile) -> &'a mut WorldBuilder {
        let tile_index = compute_index(&position, self.width) as usize;
        
        // handle the loaded content colision here
        self.tiles[tile_index] = tile;
        
        self
    }
    
    pub fn build(self) -> World {
        // TODO handle error states
        // e.g. parsing values validation
        // required lines must be present and valid
        World::new(self.width, self.heigth, self.karel.unwrap(), self.tiles)
    }
}

impl World {
    pub fn new(width: Size, heigth: Size, karel: Robot, tiles: Vec<Tile>) -> World {
        
        World {
            heigth: width,
            width: heigth,
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
        (self.width, self.heigth)
    }
    
    pub fn get_robot(&self) -> &Robot {
        &self.karel
    }
}