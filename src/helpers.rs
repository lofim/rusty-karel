use std::io::prelude::*;
use std::io;
use std::fs::File;

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

    #[allow(dead_code)]
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

pub fn load_file(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
