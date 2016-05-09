// Type alias for World size and quantity types
type Size = u32;

// Robot definition
struct Position {
    x: Size,
    y: Size,
}

#[derive(Clone, Copy, Debug)]
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
    
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    
    fn add_beeper(&mut self) {
        self.beepers += 1;
    }
    
    fn remove_beeper(&mut self) {
        self.beepers -= 1;
    }
    
    fn turn_left(&mut self) {
        self.orientation = match self.orientation {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
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
    
    fn project_robot(&mut self) {
        let index = compute_index(&self.karel.position, self.width) as usize;
        self.tiles[index] = Tile::Robot(self.karel.orientation); 
    }
}


/* 
    indexing Tile in vector will be done like this:
    
       0 1 2 3 4 X
    
    0  B B B B B 
    1  B W W B B 
    2  B B > B B 
    3  B B B B B 
    4  B B B B B 
    Y
    
    karel is on (2, 2)
    tile left of karel will be ie(x=1, y=2):
    pos = (y * width + x) 
    11 = 2 * 5 + 1
*/
fn compute_index(position: &Position, width: Size) -> Size {
    (position.y * width + position.x)
}

fn main() {
    const HEIGHT:Size = 5;
    const WIDTH:Size = 5;
    
    let pos = Position {x: 3, y: 2};
    let mut karel = Robot::new(pos, Orientation::North, 5);
    karel.turn_left();
    
    let mut tiles = vec![Tile::Empty; (HEIGHT * WIDTH) as usize];
    tiles[compute_index(&Position {x: 1, y: 2}, WIDTH) as usize] = Tile::Wall;
    tiles[compute_index(&Position {x: 1, y: 1}, WIDTH) as usize] = Tile::Beepers(2); 
    
    let mut world = World::new(HEIGHT, WIDTH, karel, tiles);
    world.project_robot();
    
    println!("Karel: ({}, {}), {:?}, Beepers: {}", 
        world.karel.position.x, 
        world.karel.position.y,
        world.karel.orientation,
        world.karel.beepers.to_string()
    );
    
    println!("{}", world.render());
}
