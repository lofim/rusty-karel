// Type alias for World size and quantity types
type Size = u32;

// Robot definition
struct Position {
    x: Size,
    y: Size,
}

#[derive(Clone, Copy)]
enum Orientation {
    North,
    East,
    South,
    West,
}

struct Robot {
    position: Position,
    beepers: Size,
    orientation: Orientation,
}

impl Robot {
    fn new(position: Position, orientation: Orientation, beepers: Size) -> Robot {
        Robot {
            position: position,
            orientation: orientation,
            beepers: beepers
        }
    }
}

//World definition
#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Beepers(Size),
    Robot(Orientation)
}

struct World {
    karel: Robot,
    height: Size,
    width: Size,
    tiles: Vec<Tile>
}

impl World {
    fn new(height: Size, width: Size, karel: Robot, tiles: Vec<Tile>) -> World {
        World {
            height: height,
            width: width,
            karel: karel,
            tiles: tiles
        }
    }
    
    fn render(&self) -> String {
        let mut renderedWorld = String::new();
        
        for (index, tile) in self.tiles.iter().enumerate() {            
            if index as u32  % self.width == 0 { 
                renderedWorld.push_str("\n");
            }
            
            renderedWorld.push_str(" ");
            renderedWorld.push_str(&self.render_tile(tile));
        }
        
        renderedWorld
    }
    
    fn render_tile(& self, tile: &Tile) -> String {
        match *tile {
            Tile::Empty => "E".to_string(),
            Tile::Wall => "W".to_string(),
            Tile::Beepers(quantity) => quantity.to_string(),
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
}

fn main() {
    let pos = Position {x: 0, y: 0};
    let karel = Robot::new(pos, Orientation::North, 0);
    
    const HEIGHT:Size = 5;
    const WIDTH:Size = 5;
    
    let mut tiles = vec![Tile::Empty; (HEIGHT * WIDTH) as usize];
    tiles[10] = Tile::Robot(Orientation::East);
    
    let world = World::new(HEIGHT, WIDTH, karel, tiles);
    
    println!("World: {}", world.render());
}

/* 
    indexing Tile in vector will be done like this:
    
    0 0 1 2 3 4 X
    0 B B B B B 
    1 B W W B B 
    2 B B > B B 
    3 B B B B B 
    4 B B B B B 
    Y
    
    karel is on (2, 2)
    tile left of karel will be ie(x=1, y=2):
    pos = (y * width + x) 
    11 = 2 * 5 + 1
*/
