use crate::world::Tile;
use crate::world::World;

use crate::helpers::compute_index;
use crate::helpers::Position;
use crate::helpers::Size;

use crate::robot::Robot;

// World builder
pub struct WorldBuilder {
    karel: Option<Robot>,
    heigth: Size,
    width: Size,
    tiles: Vec<Tile>,
}

impl WorldBuilder {
    pub fn new() -> WorldBuilder {
        WorldBuilder {
            karel: None,
            width: 0,
            heigth: 0,
            tiles: Vec::new(),
        }
    }

    pub fn dimensions(&mut self, width: Size, heigth: Size) -> &mut WorldBuilder {
        self.heigth = heigth;
        self.width = width;

        self.tiles = vec![Tile::Empty; heigth as usize * width as usize];

        self
    }

    pub fn karel(&mut self, karel: Robot) -> &mut WorldBuilder {
        self.karel = Some(karel);

        self
    }

    pub fn tile(&mut self, position: Position, tile: Tile) -> &mut WorldBuilder {
        let tile_index = compute_index(position, self.width) as usize;

        // handle the loaded content colision here
        self.tiles[tile_index] = tile;

        self
    }

    pub fn build(self) -> World {
        // TODO handle error states
        // e.g. parsing values validation
        // required lines must be present and valid
        World::new(self.width, self.heigth, self.karel.unwrap(), self.tiles)
    }
}
