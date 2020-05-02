mod helpers;
mod parsers;
mod program;
mod robot;
mod world_builder;
mod world;

use crate::parsers::plain_text::command::parse_program;
use crate::parsers::plain_text::world::parse_world;

use crate::helpers::load_file;

fn main() {
    let world_config = load_file("world.kr").unwrap();

    let mut world = parse_world(world_config.as_str()).unwrap();
    let program = parse_program(world_config.as_str()).unwrap();

    program.start(&mut world);
}
