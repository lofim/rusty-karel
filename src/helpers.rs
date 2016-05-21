use text_io::*;

use std::io::prelude::*;
use std::io;
use std::fs::File;

use robot::Orientation;
use robot::Robot;

use world::World;
use world::Tile;

// Type alias for World size and quantity types
pub type Size = u32;

// Robot definition
#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: Size,
    y: Size,
}

impl Position {
    pub fn new(x: Size, y: Size) -> Position {
        Position{x: x, y: y}
    }
    
    pub fn extract(&self) -> (Size, Size) {
        (self.x, self.y)
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

pub fn compute_index(position: &Position, width: Size) -> Size {
    (position.y * width + position.x)
}

/*
world definition file should look like this

5 5
0 0 North 3

E E E E E 
E W W E E 
E E > E E 
E E E E E 
E 1 E 2 E 

*/

pub fn load_file(file_name: &str) -> io::Result<String> {
    let mut file = try!(File::open(file_name));
    
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    
    Ok(contents)
}

/* 
    TODO: implement config parse error type
    error will be of sort like: 
        - error parsing dimension, 
        - error parsing karel, 
        - error parsing world 
*/

/*
    TODO: extract parsing into a separate parsing module
    TODO: explore RUST regex parsing capabilities
    NOTE: plain text file parsing is used on purpose, yaml and json variant are on roadmap
*/
fn parse_integer(token: &str) -> Result<u32, String> {
    token
        .parse::<u32>()
        .map_err(|e| "Expecting a number".to_string())
}

fn parse_orientation(token: &str) -> Result<Orientation, String> {
    token
        .parse::<Orientation>()
        .map_err(|e| "Expecting string describing orientation".to_string())
}

pub fn parse_size_line(line: &str) -> Result<(u32, u32), String> {
    let mut line_tokens = line.split_whitespace();
    
    line_tokens.next(); // discard the first value - descriptor
    
    let width = try!(
        line_tokens.next()
            .ok_or("Error parsing world width".to_string())
            .and_then(parse_integer));
    
    let height = try!(
        line_tokens.next()
            .ok_or("Error parsing world height".to_string())
            .and_then(parse_integer));
    
    Ok((width, height))
}

pub fn parse_karel_line(line: &str) -> Result<Robot, String> {
    let mut line_tokens = line.split_whitespace();
    
    line_tokens.next(); // discard the first value - descriptor
    
    // karel x y north 3
    
    // repetition is definitely not ok - explore posibilities of
    // removing the biolerplate with a macro 
    let x = try!(
        line_tokens.next()
            .ok_or("Error parsing karel's x position".to_string())
            .and_then(parse_integer)
    );
            
    let y = try!(
        line_tokens.next()
            .ok_or("Error parsing karel's y position".to_string())
            .and_then(parse_integer)
    );
    
    let orientation = try!(
        line_tokens.next()
            .ok_or("Error parsing karel's orientation".to_string())
            .and_then(parse_orientation)
    );
    
    let beepers = try!(
        line_tokens.next()
            .ok_or("Error parsing karel's beepers".to_string())
            .and_then(parse_integer)
    );
    
    Ok(Robot::new(Position::new(x, y), orientation, beepers))
}

pub fn parse_beeper_line(line: &str) -> Result<Tile, String> {
    // let mut line_tokens = line.split_whitespace();
    
    // line_tokens.next();
    
    // let x = try!(
    //     line_tokens.next()
    //         .ok_or("Error parsing object's x position".to_string())
    //         .and_then(parse_integer)
    // );
            
    // let y = try!(
    //     line_tokens.next()
    //         .ok_or("Error parsing object's y position".to_string())
    //         .and_then(parse_integer)
    // );
    unimplemented!();
}

pub fn parse_wall_line(line: &str) -> Result<Tile, String> {
    unimplemented!();
}