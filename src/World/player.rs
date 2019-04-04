use renderer::vector::Vec2 as Vec2;
use world::out_of_world_bounds;
use world::wall::Wall;

use serde::{Deserialize, Serialize};

pub const MOVE_RATE : f32 = 5.0;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    ///Their position in the world
    pub pos: Vec2<f32>,
    ///Direction their facing
    pub dir: Vec2<f32>,
}

impl Player {

}
