#[macro_use]
extern crate text_io;

mod helpers;
mod robot;
mod world;

use helpers::Size;
use helpers::Position;
use helpers::compute_index;
use helpers::load_file;
use helpers::parse_size_line;
use helpers::parse_karel_line;

use robot::Robot;
use robot::Orientation;

use world::World;
use world::Tile;
use world::WorldBuilder;

fn main() {
    
    let mut world_builder = WorldBuilder::new();
    
    let world_config = load_file("world.kr").unwrap();
    let lines = world_config.lines();
    
    // parsing config file - building world
    for line in lines {
        if line.starts_with("size") {
            let (width, heigth) = parse_size_line(line).unwrap();
            world_builder.dimensions(width, heigth);
        }
        
        if line.starts_with("karel") {
            let karel = parse_karel_line(line).unwrap();
            println!("karel stuf");
            world_builder.karel(karel);
        }
        
        if line.starts_with("beeper") {
            // world = parse_beeper_line(line).unwrap();
        }
        
        if line.starts_with("wall") {
            // world = parse_wall_line(line).unwrap();
        }
    }
    
    let world = world_builder.build();
    
    println!("size {:?}", world.dimensions());
    println!("karel info {:?}", world.get_robot().info());
    println!("tiles {:?}", world.render());
}
