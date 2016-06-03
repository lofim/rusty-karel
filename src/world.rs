use helpers::Size;
use helpers::compute_index;
use helpers::Position;

use robot::Orientation;
use robot::Robot;

// World definition
#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Wall,
    Beepers(Size),
    Robot(Orientation)
}

pub struct World {
    karel: Robot,
    heigth: Size,
    width: Size,
    tiles: Vec<Tile>
}

impl World {
    pub fn new(width: Size, heigth: Size, karel: Robot, tiles: Vec<Tile>) -> World {

        World {
            heigth: width,
            width: heigth,
            karel: karel,
            tiles: tiles
        }
    }

    pub fn render(&self) -> String {
        let tiles_projection = self.project_robot();
        let mut rendered_world = String::new();

        for (index, tile) in tiles_projection.iter().enumerate() {
            if index as u32  % self.width == 0 {
                rendered_world.push_str("\n");
            }

            rendered_world.push_str(" ");
            rendered_world.push_str(&self.render_tile(tile));
        }

        rendered_world
    }

    fn render_tile(&self, tile: &Tile) -> String {
        match *tile {
            Tile::Empty => ".".to_string(),
            Tile::Wall => "W".to_string(),
            Tile::Beepers(_) => "*".to_string(),
            Tile::Robot(ref orientation) => self.render_orientation(orientation),
        }
    }

    fn render_orientation(&self, orientation: &Orientation) -> String {
        match *orientation {
            Orientation::North => "^".to_string(),
            Orientation::East => ">".to_string(),
            Orientation::South => "v".to_string(),
            Orientation::West => "<".to_string(),
        }
    }

    fn project_robot(&self) -> Vec<Tile> {
        let (position, orientation, _) = self.karel.info();
        let index = compute_index(&position, self.width) as usize;
        let mut tiles_projection = self.tiles.clone();

        tiles_projection[index] = Tile::Robot(orientation);
        tiles_projection
    }

    #[allow(dead_code)]
    pub fn dimensions(&self) -> (Size, Size) {
        (self.width, self.heigth)
    }

    pub fn get_robot(&mut self) -> &mut Robot {
        &mut self.karel
    }

    pub fn move_robot(&mut self) -> bool {
        let (karel_position, orientation, _) = self.karel.info();
        let (karel_x, karel_y) = karel_position.extract();

        let (new_option_x, new_option_y) = match orientation {
            Orientation::North => (Some(karel_x), karel_y.checked_sub(1)),
            Orientation::South => (Some(karel_x), karel_y.checked_add(1)),
            Orientation::East => (karel_x.checked_add(1), Some(karel_y)),
            Orientation::West => (karel_x.checked_sub(1), Some(karel_y)),
        };

        // check  if not moving out of world bounds
        let new_x = match new_option_x {
            Some(x) => x,
            None => self.width + 1 // put out of bound
        };

        let new_y = match new_option_y {
            Some(y) => y,
            None => self.width + 1 // put out of bound
        };

        if new_x >= self.width || new_y >= self.heigth {
            return false;
        }

        let new_position = Position::new(new_x, new_y);
        let new_position_index = compute_index(&new_position, self.width);

        // check for not running into wall
        match self.tiles[new_position_index as usize] {
            Tile::Wall => return false,
            _ => ()
        }

        // move karel
        self.karel.set_position(new_position);

        return true;
    }
}
