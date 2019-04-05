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
use world::DebugWindowFlags;

use debug_window;

pub const FOV: f32 = f32::consts::FRAC_PI_2;

pub struct renderer<'a>{
    canvas: &'a mut Canvas<Window>,
    pub ray_results: Vec<(f32, Color)>
}


impl<'a> renderer<'a> {
    pub fn new(canvas: &'a mut Canvas<Window>) -> renderer<'a> {
        renderer {
            canvas,
            ray_results: Vec::with_capacity(SCREEN_SIZE_X as usize),
        }
    }

    pub fn cast_rays(&mut self, gs : &GameState, changed_frame: bool, dflags: &mut DebugWindowFlags){
        let mut changed_frame_inner = changed_frame;
        'raycasting: for x in 0..SCREEN_SIZE_X as usize {
            let cameraX = ((2.0 * x as f32) / SCREEN_SIZE_X as f32) - 1.0;
            let ray: Ray2D = Ray2D::new(gs.p.dir, gs.camera_plane, cameraX, x);
            if let Some(insp_i) = dflags.inspect_ray{
                changed_frame_inner = x == insp_i;
                dflags.inspect_ray_info = Some(ray);
            }

            let possible_wall: Option<(Wall, Vec2<f32>)> = find_wall_and_distance(gs, ray, changed_frame_inner, dflags);
            //TODO remove
//                changed_frame_inner = false;

            if let Some((sampled_wall, dist)) = possible_wall {
                //Ray Fish Eye Fix
                let ang = ray.dir.normalized().angle();
                let fixed_dist : f32 = Vec2{
                    x: (dist.x * f32::cos(ang)) - (dist.y * f32::sin(ang)),
                    y: (dist.x * f32::sin(ang)) + (dist.y * f32::cos(ang))
                }.length();

                self.ray_results.push((fixed_dist, sampled_wall.color));
            }
        }
    }

    //returns last frame info
    pub fn draw_frame(&mut self,texture: &mut Texture){
        {
            let statics_renderer = |buffer: &mut [u8], pitch: usize| {
                draw_ceiling(buffer, pitch);
                draw_floor(buffer, pitch);
                draw_collumns(buffer, pitch, &self.ray_results);
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


fn draw_collumns(buffer: &mut [u8], pitch: usize, col_dists_and_colors : &Vec<(f32, Color)>){
    for (x, (dist, color)) in col_dists_and_colors.iter().enumerate() {
        draw_col(buffer, pitch, x, *color, *dist);
    }
}
fn draw_col(buffer: &mut [u8], pitch: usize, x: usize, color: Color, dist: f32) {
    //println!("SCALING BY DIST {:?}", dist);
    let h = (SCREEN_SIZE_Y as f32 / dist)  * 2.2 * WORLD_CELL_SIZE as f32; //This dist will have to be normalized for fix eye
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