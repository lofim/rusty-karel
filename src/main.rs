mod helpers;
mod robot;
mod world;
mod plain_file_parser;
mod world_builder;

use helpers::load_file;

use plain_file_parser::parse_size_line;
use plain_file_parser::parse_karel_line;
use plain_file_parser::parse_beeper_line;
use plain_file_parser::parse_wall_line;

use world_builder::WorldBuilder;

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
    println!("karel info {:?}", world.get_robot().info());
    println!("tiles {}", world.render());

    world.get_robot().turn_left();
    // world.get_robot().turn_left();

    if !world.move_robot() {
        println!("Error moving robot");
    }

    println!("tiles {}", world.render());

    if !world.move_robot() {
        println!("Error moving robot");
    }

    println!("tiles {}", world.render());

    if !world.move_robot() {
        println!("Error moving robot");
    }

    println!("tiles {}", world.render());
}
