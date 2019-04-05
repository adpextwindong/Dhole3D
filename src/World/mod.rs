pub mod player;
pub mod wall;
pub mod test_util;

use std;
use std::f32;

use world::wall::Wall as Wall;
use world::wall::NULL_COLOR;
use world::player::Player as Player;

use renderer::ray2D::Ray2D;
use renderer::vector::Vec2 as Vec2;

pub const WORLD_SIZE_X: usize = 10;
pub const WORLD_SIZE_Y: usize = 10;
pub const WORLD_CELL_SIZE: u32 = 10; // 10 Meters?

#[derive(Debug)]
pub struct GameState{
    pub the_world : Vec<Vec<Wall>>,
    pub p : Player,
    pub camera_plane : Vec2<f32>,
}

#[derive(Debug)]
pub struct DebugWindowFlags{
    pub distsView : bool,
    pub inspect_ray : Option<usize>,
    pub inspect_ray_info : Option<Ray2D>
}

impl GameState{
    pub fn get_world_cell_at_vec2_pos(&self, pos: Vec2<f32>, debug : bool) -> Wall {
        let x: usize = (pos.x / WORLD_CELL_SIZE as f32).floor() as usize;
        let y: usize = (pos.y / WORLD_CELL_SIZE as f32).floor() as usize;
//        if debug {
//            println!("GET_WORLD POS {:?} w[x] : {:?} w[y] : {:?}",pos,x,y);
//        }
        self.the_world[x][y]
    }

    pub fn move_player(&mut self, delta_pos : Vec2<f32>){
        let new_pos = self.p.pos + delta_pos;
        if out_of_world_bounds(new_pos) == false {
            if self.get_world_cell_at_vec2_pos(new_pos, false).full == false{
                self.p.pos = new_pos;
            }

        }
    }

}

pub fn find_next_cell_boundary(line_pos: f32, positive: bool) -> i32 {
    // TODO Make tests for this
    // |   * |  positive gives WORLD_CELL_SIZE
    // |_*___|  negative gives 0
    // 0 ..  WORLD_CELL_SIZE
    if positive {
        WORLD_CELL_SIZE as i32 * ((line_pos.floor() as i32 / WORLD_CELL_SIZE as i32) + 1)
    } else {
        WORLD_CELL_SIZE as i32 * (line_pos.floor() as i32 / WORLD_CELL_SIZE as i32)
    }
}

pub fn out_of_world_bounds(pos: Vec2<f32>) -> bool {
    if pos.x >= 0.0 && pos.y >= 0.0{
        if pos.x == std::f32::INFINITY || pos.y == std::f32::INFINITY {
            return true;
        }else{
            return (pos.x >= (WORLD_SIZE_X as f32 * WORLD_CELL_SIZE as f32)) ||
                (pos.y >= (WORLD_SIZE_Y as f32 * WORLD_CELL_SIZE as f32))
        }
    }else{
        return true;
    }

}

pub fn gen_blank_world(x: usize, y: usize) -> Vec<Vec<Wall>> {
    let mut ret = Vec::new();
    let reg_wall = Wall {
        full: false,
        color: NULL_COLOR,
    };
    for _ in 0..y {
        let mut w = Vec::<Wall>::with_capacity(x as usize);
        for _ in 0..x {
            w.push(reg_wall);
        }
        ret.push(w);
    }

    return ret;
}