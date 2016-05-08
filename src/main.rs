// Type alias for World size and quantity types
type Size = u32;

// Robot definition
struct Position {
    x: Size,
    y: Size,
}

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
enum Tile {
    Empty,
    Wall,
    Beepres(Size),
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
    
    fn render(& self) -> &str {
        "World"
    }
}

fn main() {
    let pos = Position {x: 0, y: 0};
    let karel = Robot::new(pos, Orientation::North, 0);
    const HEIGHT:Size = 5;
    const WIDTH:Size = 5;
    let tiles = vec![
        Tile::Empty, Tile::Empty,Tile::Empty,Tile::Empty,Tile::Empty,
        Tile::Empty, Tile::Empty,Tile::Empty,Tile::Empty,Tile::Empty,
        Tile::Empty, Tile::Empty,Tile::Empty,Tile::Empty,Tile::Empty,
        Tile::Empty, Tile::Empty,Tile::Empty,Tile::Empty,Tile::Empty, 
    ];
    
    let world = World::new(HEIGHT, WIDTH, karel, tiles);
    
    println!("World: {:?}", world.render());
}
