mod helpers;
mod robot;
mod world;
mod world_builder;

mod plain_world_parser;
mod plain_command_parser;
mod program;

use plain_world_parser::parse_world;
use plain_command_parser::parse_program;

use helpers::load_file;

fn main() {
    let world_config = load_file("world.kr").unwrap();

    let mut world = parse_world(world_config.as_str()).unwrap();
    let program = parse_program(world_config.as_str()).unwrap();

    program.start(&mut world);
}
