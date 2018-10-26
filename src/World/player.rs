use renderer::vector::Vec2 as Vec2;
use world::out_of_world_bounds;
use world::wall::Wall;
pub const MOVE_RATE : f32 = 5.0;

#[derive(Copy, Clone)]
pub struct Player {
    ///Their position in the world
    pub pos: Vec2<f32>,
    ///Direction their facing
    pub dir: Vec2<f32>,
}

impl Player {
    pub fn move_player(&mut self,w: &Vec<Vec<Wall>>, delta_pos : Vec2<f32>){
        let new_pos = self.pos + delta_pos;
        if out_of_world_bounds(new_pos) == false {
            self.pos = new_pos;
        }
    }
}
