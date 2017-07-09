use world::Tile;
use world::World;

use robot::Orientation;
use robot::Robot;

use helpers::Position;

use world_builder::WorldBuilder;

/* 
    TODO: implement config parse error type
    error will be of sort like: 
        - error parsing dimension, 
        - error parsing karel, 
        - error parsing world 
*/

/*
    TODO: remove parse line boilerplate with macro perhaps?
 */

/*
    TODO: explore RUST regex parsing capabilities
    NOTE: plain text file parsing is used on purpose, yaml and json variant are on roadmap
*/
fn parse_integer(token: &str) -> Result<u32, String> {
    token
        .parse::<u32>()
        .map_err(|_| "Expecting a number".to_string())
}

fn parse_orientation(token: &str) -> Result<Orientation, String> {
    token
        .parse::<Orientation>()
        .map_err(|_| "Expecting string describing orientation".to_string())
}

fn parse_size_line(line: &str) -> Result<(u32, u32), String> {
    let mut line_tokens = line.split_whitespace();
    
    line_tokens.next(); // discard the first value - descriptor
    
    let width = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing world width".to_string())
            .and_then(parse_integer));
    
    let height = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing world height".to_string())
            .and_then(parse_integer));
    
    Ok((width, height))
}

fn parse_karel_line(line: &str) -> Result<Robot, String> {
    let mut line_tokens = line.split_whitespace();
    
    line_tokens.next(); // discard the first value - descriptor
    
    // karel x y north 3
    
    // repetition is definitely not ok - explore posibilities of
    // removing the biolerplate with a macro 
    let x = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing karel's x position".to_string())
            .and_then(parse_integer)
    );
            
    let y = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing karel's y position".to_string())
            .and_then(parse_integer)
    );
    
    let orientation = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing karel's orientation".to_string())
            .and_then(parse_orientation)
    );
    
    let beepers = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing karel's beepers".to_string())
            .and_then(parse_integer)
    );
    
    Ok(Robot::new(Position::new(x, y), orientation, beepers))
}

fn parse_beeper_line(line: &str) -> Result<(Position, Tile), String> {
    let mut line_tokens = line.split_whitespace();
    
    line_tokens.next();
    
    let x = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing beeper's x position".to_string())
            .and_then(parse_integer)
    );
            
    let y = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing beeper's y position".to_string())
            .and_then(parse_integer)
    );
    
    let quantity = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing beeper's quantity".to_string())
            .and_then(parse_integer)
    );
    
    let beeper_tile = Tile::Beepers(quantity);
    
    Ok((Position::new(x, y), beeper_tile))
}

fn parse_wall_line(line: &str) -> Result<(Position, Tile), String> {
    let mut line_tokens = line.split_whitespace();
    
    line_tokens.next();
    
    let x = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing wall's x position".to_string())
            .and_then(parse_integer)
    );
            
    let y = try!(
        line_tokens.next()
            .ok_or_else(|| "Error parsing wall's y position".to_string())
            .and_then(parse_integer)
    );
    
    let wall_tile = Tile::Wall;
    
    Ok((Position::new(x, y), wall_tile))
}

pub fn parse_world(file_contents: &str) -> Result<World, String> {
    let mut world_builder = WorldBuilder::new();

    let lines = file_contents.lines();

    // parsing config file - building world
    for line in lines {
        if line.starts_with("size") {
            let (width, heigth) = parse_size_line(line).unwrap();
            world_builder.dimensions(width, heigth);
        }

        if line.starts_with("karel") {
            let karel = parse_karel_line(line).unwrap();
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

    Ok(world_builder.build())
}