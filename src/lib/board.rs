use itertools::Itertools;
use std::collections::HashMap;

use create::lib::coordinates::Coordinates;
use create::lib::game::Game;
use create::lib::player::Player;

#[derive(PartialEq, Eq, Clone)]
pub struct Board {
    hash: HashMap<Coordinates, Player>,
    min_x: i8,
    min_y: i8,
    max_x: i8,
    max_y: i8,
}

impl Board {
    
    fn on_board(&self, coordinates: &Coordinates) -> bool {
        coordinates.x >= self.min_x
            && coordinates.x <= self.max_x
            && coordinates.y >= self.min_y
            && coordinates.y <= self.max_y
    }

    pub fn new(game: &Game) -> Board {
        let hash = HashMap::new();
        Board {
            hash,
            min_x: game.min_x,
            max_x: game.max_x,
            min_y: game.min_y,
            max_y: game.max_y
        }
    }
}