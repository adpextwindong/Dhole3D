pub mod ray2D;
pub mod vector;

mod dda;

use std::f32;
use std::io;
use std::io::Read;

use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use world::wall::Wall;
use world::player::Player;
use super::SCREEN_SIZE_X;
use super::SCREEN_SIZE_Y;
use world::wall::FLOOR_GREY;
use world::WORLD_CELL_SIZE;

use renderer::vector::rotate_clockwise;
use renderer::vector::rotate_counter_clockwise;
use renderer::vector::Vec2;
use renderer::dda::find_wall_and_distance;
use renderer::ray2D::Ray2D;
use world::GameState;

pub const FOV: f32 = f32::consts::FRAC_PI_2;

pub struct renderer<'a>{
    canvas: &'a mut Canvas<Window>,
}


impl<'a> renderer<'a> {
    pub fn new(canvas: &'a mut Canvas<Window>) -> renderer<'a> {
        renderer {
            canvas,
        }
    }

    pub fn draw_frame(&mut self, texture: &mut Texture,gs : &GameState, changed_frame_outer :  bool){

        {//TODO Draw statics texture
            let statics_renderer = |buffer: &mut [u8], pitch: usize| {
                draw_ceiling(buffer, pitch);
                draw_floor(buffer, pitch);

                let p_copy: Player = gs.p;
                let mut changed_frame_inner = changed_frame_outer;

                //Currently this is disabled for looking at the middle ray
                //filled accross the whole screen for simpler debugging.
                //let mut ray_curr_dir = rotate_counter_clockwise(p_copy.dir, FOV / 2.0);
                let mut ray_curr_dir = p_copy.dir;

                let delta_theta_y = FOV / SCREEN_SIZE_X as f32;

                'raycasting: for mut y in 0..SCREEN_SIZE_X as usize {
                    let ray: Ray2D = Ray2D::new(ray_curr_dir, p_copy.pos);
                    let possible_wall : Option<(Wall, Vec2<f32>)> = find_wall_and_distance(gs, ray, changed_frame_inner);
                    //TODO remove
                    changed_frame_inner = false;

                    if let Some((sampled_wall, dist)) = possible_wall {

                        //use rayfishfix???
                        let ang = ray.dir.normalized().angle();
                        let fixed_dist : f32 = Vec2{
                            x: (dist.x * f32::cos(ang)) - (dist.y * f32::sin(ang)),
                            y: (dist.x * f32::sin(ang)) + (dist.y * f32::cos(ang))
                        }.length();

                        draw_col(buffer, pitch, y, sampled_wall.color, fixed_dist);

                    }
                    //ray_curr_dir = rotate_clockwise(ray_curr_dir, delta_theta_y);
                }
            };

            texture.with_lock(None, &statics_renderer).unwrap();
        }
        //Present Frame
        self.canvas.copy(texture, None, None).unwrap();
        self.canvas.present();
    }

    //TODO make these colors world constants
    //TODO_FAR add support for different colored ceilings
    //      and actually make them world surfaces once we get to true'r 3d

}

fn clear_texture(buffer: &mut [u8], pitch: usize){
    for x in 0 .. SCREEN_SIZE_X as usize {
        for y in 0 .. SCREEN_SIZE_Y as usize {
            let offset = y * pitch + x * 3;
            buffer[offset] = 0 as u8;
            buffer[offset + 1] = 0 as u8;
            buffer[offset + 2] = 0 as u8;
        }
    }
}

fn draw_ceiling(buffer: &mut [u8], pitch: usize) {
    for x in 0 .. SCREEN_SIZE_X as usize {
        for y in 0 .. (SCREEN_SIZE_Y / 2) as usize {
            let offset = y * pitch + x * 3;
            buffer[offset] = 0 as u8;
            buffer[offset + 1] = 0 as u8;
            buffer[offset + 2] = 0 as u8;
        }
    }
}

fn draw_floor(buffer: &mut [u8], pitch: usize) {
    let pos_y = SCREEN_SIZE_Y as usize / 2;

    for x in 0..SCREEN_SIZE_X as usize {
        for y in pos_y..SCREEN_SIZE_Y as usize {
            let offset = y * pitch + x * 3;
            buffer[offset] = FLOOR_GREY.r as u8;
            buffer[offset + 1] = FLOOR_GREY.g as u8;
            buffer[offset + 2] = FLOOR_GREY.b as u8;
        }
    }
}

fn draw_col(buffer: &mut [u8], pitch: usize, x: usize, color: Color, dist: f32) {
    //println!("SCALING BY DIST {:?}", dist);
    let h = SCREEN_SIZE_Y as f32 / dist * WORLD_CELL_SIZE as f32; //This dist will have to be normalized for fix eye
    let col_start = h /2.0;
    let mut clamp_end = SCREEN_SIZE_Y as f32 - (h / 2.0);
    if clamp_end < 0.0{
        clamp_end = 0.0;
    }
    let col_end = clamp_end;


//    println!("X is :{:?}, col start :{:?}, col end :{:?}", x, col_start, col_end);
    for y in col_start as usize .. col_end as usize {
        let offset = y * pitch + x * 3;
//        if(offset > (SCREEN_SIZE_Y*SCREEN_SIZE_X*3) as usize){
//            println!("y is :{:?}", y);
//        }
//        assert!(offset < (SCREEN_SIZE_Y*SCREEN_SIZE_X*3) as usize);
        buffer[offset] = color.r as u8;
        buffer[offset + 1] = color.g as u8;
        buffer[offset + 2] = color.b as u8;
    }
}