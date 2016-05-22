#[macro_use]
extern crate text_io;

mod helpers;
mod robot;
mod world;

use helpers::load_file;

use helpers::parse_size_line;
use helpers::parse_karel_line;
use helpers::parse_beeper_line;
use helpers::parse_wall_line;

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
            let (position, beeper) = parse_beeper_line(line).unwrap(); 
            world_builder.tile(position, beeper);
        }
        
        if line.starts_with("wall") {
            let (position, wall) = parse_wall_line(line).unwrap();
            world_builder.tile(position, wall);
        }
    }
    
    let mut world = world_builder.build();
    
    println!("size {:?}", world.dimensions());
    println!("karel info {:?}", world.get_robot().info());
    world.project_robot();
    println!("tiles {}", world.render());
}
