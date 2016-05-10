mod helpers;
mod robot;
mod world;

use helpers::Size;
use helpers::Position;
use helpers::compute_index;

use robot::Robot;
use robot::Orientation;

use world::World;
use world::Tile;

fn main() {
    const HEIGHT:Size = 5;
    const WIDTH:Size = 5;
    
    let pos = Position::new(3, 2);
    let mut karel = Robot::new(pos, Orientation::North, 5);
    karel.turn_left();
    
    let mut tiles = vec![Tile::Empty; (HEIGHT * WIDTH) as usize];
    tiles[compute_index(&Position::new(1, 2), WIDTH) as usize] = Tile::Wall;
    tiles[compute_index(&Position::new(1, 1), WIDTH) as usize] = Tile::Beepers(2); 
    
    let mut world = World::new(HEIGHT, WIDTH, karel, tiles);
    world.project_robot();
    
    let (position, orientation, beepers) = world.get_robot().info();
    let (x, y) = position.extract();
    println!("Karel: ({}, {}), {:?}, Beepers: {}", 
        x, y, orientation, beepers.to_string()
    );
    
    println!("{}", world.render());
}
